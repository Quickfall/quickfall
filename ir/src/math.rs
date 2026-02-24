//! Math and arithmetic code

use errors::errs::{BaseResult, base::BaseError};
use inkwell::{builder::Builder, values::IntValue};
use lexer::toks::math::MathOperator;

pub fn make_math_operation<'a>(builder: &Builder<'a>, a: IntValue<'a>, b: IntValue<'a>, name: String, operation: MathOperator) -> BaseResult<IntValue<'a>> {
	let res = match operation {
		MathOperator::ADD => builder.build_int_add(a, b, &name),
		MathOperator::SUBSTRACT => builder.build_int_sub(a, b, &name),
		MathOperator::DIVIDE => builder.build_int_signed_div(a, b, &name),
		MathOperator::MULTIPLY => builder.build_int_mul(a, b, &name)
	};

	match res {
		Ok(v) => return Ok(v),
		Err(_) => return Err(BaseError::err("Couldn't make math operation!".to_string()))
	};
}