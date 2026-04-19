//! The context definitions for the AstoIR HIR layer.

use std::{collections::{HashMap, HashSet}};

use compiler_typing::{storage::TypeStorage, tree::Type};
use compiler_utils::{hash::SelfHash, utils::indexed::IndexStorage};
use diagnostics::{DiagnosticResult, DiagnosticSpanOrigin, builders::{make_cannot_find_func, make_cannot_find_var, make_doesnt_exist_in_era}};

use crate::{nodes::HIRNode, storage::GlobalScopeStorage, structs::HIRStructContainer};

pub type HIRFunction = (Option<Type>, Vec<(u64, Type)>, String);

/// The function HIR context. Contains a mapping from element name hash to element index and other variable information. 
/// Uses a branch based system to contain variables.
/// 
/// # Resolution system
/// First, `hash_to_ind` is used to convert any element (like variable) name into an element index. This element index can then be used to quickly retrieve information.
/// 
/// `HIRBranchedContext` uses a global system to store every local variable of any branch within the function very simply safely. Every stored variable is a `HRBRanchedVariable` which contains the era from when it was created.
///
/// # Indexing system
/// The branch index system is fairly simple, as soon as a new branch is parsed, we increment one. This allows for branches inside of another branch to have an index higher than the original branch which is easier for the era system!
/// 
/// # Era system
/// Every variable has a specific branch period called era in which they are allowed to live in. An era can simply be defined as a branch index. 
/// 
/// Every branch index stores an end branch index from when it ends (inside of `ending_eras`). This end branch index will be used to calculate when the era of a variable ends.
/// 
/// 
#[derive(Debug, Clone)]
pub struct HIRBranchedContext {
	pub hash_to_ind: HashMap<SelfHash, usize>, // TODO: add a layer system to this so you are able to put multiple variables with the same name.
	pub ending_eras: HashMap<usize, usize>,

	pub variables: Vec<HIRBranchedVariable>, // index is the resolved indec

	pub current_branch: usize,
	pub current_element_index: usize,
}

impl HIRBranchedContext {
	pub fn new() -> Self {
		HIRBranchedContext { hash_to_ind: HashMap::new(), ending_eras: HashMap::new(), variables: Vec::new(), current_branch: 0, current_element_index: 0 }
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

	/// Introduces a new variable in the next branch era
	pub fn introduce_variable_next_era(&mut self, hash: u64, t: Type, has_default: bool) -> Result<usize, ()> {
		let identity = SelfHash { hash };

		if self.hash_to_ind.contains_key(&identity) {
			return Err(());
		}

		let mut var: HIRBranchedVariable = HIRBranchedVariable { introduced_in_era: self.current_branch + 1, variable_type: t, has_default, introduced_values: HashSet::new(), requires_address: false, mutation_count: 0, usage_count: 0 };
				
		if has_default {
			var.mutation_count += 1;
		}

		self.variables.push(var);

		let ind: usize = self.current_element_index;
		self.current_element_index += 1;

		self.hash_to_ind.insert(identity, ind);

		return Ok(ind);
	} 

	/// Introduces a new variable in the current branch era.
	pub fn introduce_variable(&mut self, hash: u64, t: Type, has_default: bool) -> Result<usize, ()> {
		let identity = SelfHash { hash };

		if self.hash_to_ind.contains_key(&identity) {
			return Err(());
		}

		let mut var: HIRBranchedVariable = HIRBranchedVariable { introduced_in_era: self.current_branch, variable_type: t, has_default, introduced_values: HashSet::new(), requires_address: false, mutation_count: 0, usage_count: 0};
				
		if has_default {
			var.mutation_count += 1;
		}

		self.variables.push(var);

		let ind: usize = self.current_element_index;
		self.current_element_index += 1;

		self.hash_to_ind.insert(identity, ind);

		return Ok(ind);
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

	/// Obtains the variable index from the hash if it's available, otherwise returns an error explaining why it failed
	pub fn obtain<K: DiagnosticSpanOrigin>(&mut self, hash: u64, origin: &K) -> DiagnosticResult<usize> {
		let identity = SelfHash { hash };

		match self.hash_to_ind.get(&identity) {
			None => return Err(make_cannot_find_var(origin, &hash).into()),
			Some(ind) => {
				let ind = *ind;

				if !self.is_alive(ind) {
					if self.is_dropped_before(ind) {
						return Err(make_doesnt_exist_in_era(origin, &hash).into())
					}

					panic!("Dropped unalived variable")
				}

				self.variables[ind].usage_count += 1;

				return Ok(ind)
			}
		}
	}

	pub fn is_eligible_for_ssa(&self, ind: usize) -> bool {
		let var = &self.variables[ind];

		return !var.requires_address && var.mutation_count <= 1 && !var.variable_type.can_use_index_access() && false
	}
	
}

#[derive(Debug, Clone)]
pub struct HIRBranchedVariable {
	pub introduced_in_era: usize,
	pub variable_type: Type,

	pub usage_count: usize,

	pub requires_address: bool,

	/// The amount of times the variable has been changed
	pub mutation_count: usize,

	pub has_default: bool,
	pub introduced_values: HashSet<usize> // TODO: try to potentially reduce this
}

#[derive(Debug)]
pub struct HIRContext {
	pub functions: IndexStorage<HIRFunction>, 
	pub function_declarations: Vec<Option<Box<HIRNode>>>,
	pub function_contexts: Vec<Option<HIRBranchedContext>>,
	pub static_variables: IndexStorage<Type>,
	pub struct_func_impls: HashMap<usize, HIRStructContainer>,
	pub type_storage: TypeStorage,
	pub global_scope: GlobalScopeStorage
}

#[derive(PartialEq)]
pub enum VariableKind {
	STATIC,
	LOCAL
}

impl HIRContext {
	pub fn new() -> Self {
		return HIRContext { functions: IndexStorage::new(), static_variables: IndexStorage::new(), type_storage: TypeStorage::new().unwrap(), function_contexts: vec![], function_declarations: vec![], struct_func_impls: HashMap::new(), global_scope: GlobalScopeStorage::new() }
	}

	pub fn translate_function<K: DiagnosticSpanOrigin>(&self, func_hash: u64, origin: &K) -> DiagnosticResult<usize> {
		return match self.functions.get_index(func_hash) {
			Some(v) => Ok(v),
			None => return Err(make_cannot_find_func(origin, &func_hash).into())
		}
	}
}

pub fn get_variable<K: DiagnosticSpanOrigin>(context: &HIRContext, curr_ctx: &mut HIRBranchedContext, hash: u64, origin: &K) -> DiagnosticResult<(VariableKind, Type, usize)> {
	if curr_ctx.hash_to_ind.contains_key(&SelfHash { hash }) {
		let ind = curr_ctx.obtain(hash, origin)?;

		return Ok((VariableKind::LOCAL, curr_ctx.variables[ind].variable_type.clone(), ind));
	}

	match context.static_variables.get_index(hash) {
		Some(v) => {
			return Ok((VariableKind::STATIC, context.static_variables.vals[v].clone(), v))
		},

		None => return Err(make_cannot_find_var(origin, &hash).into())
	};
} 