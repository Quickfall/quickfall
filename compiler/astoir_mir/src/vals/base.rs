pub struct BaseHIRValue {
	val_index: usize,
	vtype: BaseValueType
}

pub enum BaseValueType {
	IntValue,
	FloatValue,
	FixedValue, // fixed point
	PointerValue, // variables
}

