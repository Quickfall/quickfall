use std::collections::{HashMap, HashSet};

use compiler_utils::hash::HashedString;
use diagnostics::{
    DiagnosticResult, DiagnosticSpanOrigin,
    builders::{make_cannot_find, make_doesnt_exist_in_era},
};
use typing::container::Type;

pub struct BranchedContext {
    pub hash_to_ind: HashMap<HashedString, usize>,
    pub ending_eras: HashMap<usize, usize>,

    pub variables: Vec<HIRBranchedVariable>, // index is the resolved index
    pub ending_points: Vec<HIRBranchedEndingPoint>,

    pub current_branch: usize,
    pub current_element_index: usize,
}

#[derive(Clone)]
pub struct HIRBranchedVariable {
    pub introduced_in_era: usize,
    pub variable_type: Type,

    pub usage_count: usize,

    pub requires_address: bool,

    /// The amount of times the variable has been changed
    pub mutation_count: usize,

    pub has_default: bool,
    pub introduced_values: HashSet<usize>, // TODO: try to potentially reduce this
}

#[derive(Clone, Debug)]
pub struct HIRBranchedEndingPoint {
    pub introduced_in_era: usize,
    pub kind: EndingPointKind,
}

#[derive(Clone, Debug)]
pub enum EndingPointKind {
    Return,
    Crash,
    NoneReturn,
}

impl BranchedContext {
    pub fn new() -> Self {
        BranchedContext {
            hash_to_ind: HashMap::new(),
            ending_eras: HashMap::new(),
            variables: vec![],
            ending_points: vec![],
            current_branch: 0,
            current_element_index: 0,
        }
    }

    pub fn introduce_variable(
        &mut self,
        name: String,
        t: Type,
        has_default: bool,
    ) -> Result<usize, ()> {
        let identity = HashedString::new(name);

        let mut var = HIRBranchedVariable {
            introduced_in_era: self.current_branch,
            variable_type: t,
            has_default,
            introduced_values: HashSet::new(),
            requires_address: false,
            mutation_count: 0,
            usage_count: 0,
        };

        if has_default {
            var.mutation_count += 1; // TODO: investigate this
        }

        self.variables.push(var);

        let ind = self.current_element_index;
        self.current_element_index += 1;

        self.hash_to_ind.insert(identity, ind);

        Ok(ind)
    }

    pub fn obtain<K: DiagnosticSpanOrigin>(
        &mut self,
        name: String,
        origin: &K,
    ) -> DiagnosticResult<usize> {
        let identity = HashedString::new(name);

        match self.hash_to_ind.get(&identity) {
            None => return Err(make_cannot_find(origin, &identity.val).into()),
            Some(ind) => {
                let ind = *ind;

                if !self.is_alive(ind) {
                    return Err(make_doesnt_exist_in_era(origin, &identity.val).into());
                }

                self.variables[ind].usage_count += 1;
                Ok(ind)
            }
        }
    }

    /// Starts a new branch by incrementing the `current_branch` by one. Returns the newly started branch's index
    pub fn start_branch(&mut self) -> usize {
        self.current_branch += 1;
        return self.current_branch;
    }

    /// Moves to the given branch index. This is unsafe and will not handle anything, should ONLY be used AFTER AST lowering
    pub fn move_branch(&mut self, branch: usize) {
        self.current_branch = branch;
    }

    /// Ends the branch with the given branch index. Must use `start_branch` to start a new branch after.
    pub fn end_branch(&mut self, branch: usize) -> usize {
        self.ending_eras.insert(branch, self.current_branch);

        return self.current_branch;
    }

    /// Checks whenever the code is currently beyond an ending point and thus is invalid
    pub fn is_code_alive(&self) -> bool {
        for ending in &self.ending_points {
            let end = match self.ending_eras.get(&ending.introduced_in_era) {
                Some(v) => *v,
                None => ending.introduced_in_era,
            };

            if self.current_branch >= end {
                return false;
            }
        }

        return true;
    }

    pub fn introduce_ending_point(&mut self, point: EndingPointKind) {
        self.ending_points.push(HIRBranchedEndingPoint {
            introduced_in_era: self.current_branch,
            kind: point,
        })
    }

    pub fn introduce_variable_assign(&mut self, ind: usize) -> bool {
        let var = &mut self.variables[ind];

        var.mutation_count += 1;

        if var.has_default {
            return true;
        }

        var.introduced_values.insert(self.current_branch);

        return true;
    }

    pub fn introduce_variable_refer(&mut self, ind: usize) -> bool {
        let var = &mut self.variables[ind];

        var.requires_address = true;

        return true;
    }

    /// Determines if the element with the given index is still alive in the current branch.
    pub fn is_alive(&self, ind: usize) -> bool {
        let start_branch = self.variables[ind].introduced_in_era;

        if start_branch > self.current_element_index {
            return false;
        }

        return self.is_era_alive(start_branch);
    }

    pub fn is_era_alive(&self, era: usize) -> bool {
        if !self.ending_eras.contains_key(&era) {
            // If the era hasn't ended yet, (the ending era isn't added for branch start_branch)
            // this means that the variable is still alive and we are still inside of the branch start_branch
            return true;
        }

        return false;
    }

    pub fn is_dropped_before(&self, ind: usize) -> bool {
        let start_branch: usize = self.variables[ind].introduced_in_era;

        if !self.ending_eras.contains_key(&start_branch) {
            return false;
        }

        return self.ending_eras[&start_branch] < self.current_branch;
    }

    pub fn has_variable_value(&self, ind: usize) -> bool {
        let var = &self.variables[ind];

        if var.has_default {
            return true;
        }

        for era in var.introduced_values.iter() {
            if self.is_era_alive(*era) {
                return true;
            }
        }

        return false;
    }

    pub fn get_ending_era(&self, ind: usize) -> usize {
        return self.ending_eras[&self.variables[ind].introduced_in_era];
    }
}
