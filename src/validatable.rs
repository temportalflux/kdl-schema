use crate::{State, Error};

/// Trait for schema types which can be validated.
pub(crate) trait Validatable<TKdlValue, TStruct> {
	fn as_string(&self) -> String;
	fn name() -> &'static str;
	fn validate(
		&self,
		value: &TKdlValue,
		node: &kdl::KdlNode,
		data: &mut State<TStruct>,
	) -> Result<(), Error>;
}
