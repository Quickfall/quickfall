use compiler_typing::tree::Type;

pub enum HIRNode {
	CastValue { intentional: bool, value: Box<HIRNode>, new_type: Type}
}