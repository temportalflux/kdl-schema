use crate::{Error, State};

/// Trait for schema types which can be validated.
pub(crate) trait Validatable<TKdlValue, TStruct> {
	fn as_string(&self) -> String;
	fn name() -> &'static str;
	fn validate(
		&self,
		value: &TKdlValue,
		parent: Option<&kdl::KdlNode>,
		data: &mut State<TStruct>,
	) -> Result<(), Error>;
}
