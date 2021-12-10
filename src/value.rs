use crate::{Validation, Validatable, Error, State};

/// Schema equivalent of [`KdlValue`](kdl::KdlValue).
/// Provides validation for a handful of primitives.
#[derive(Debug, Clone)]
pub enum Value {
	Int(Option<Validation<i64>>),
	Float(Option<Validation<f64>>),
	String(Option<Validation<String>>),
	Boolean,
	Null,
}

impl Value {
	/// Returns the stringified simple identifier of the value-schema.
	pub(crate) fn name(&self) -> &'static str {
		match self {
			Self::Int(_) => "Int",
			Self::Float(_) => "Float",
			Self::String(_) => "String",
			Self::Boolean => "Boolean",
			Self::Null => "Null",
		}
	}
}

impl std::fmt::Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.name())
	}
}

impl<TStruct> Validatable<kdl::KdlValue, TStruct> for Value {
	fn as_string(&self) -> String {
		self.name().to_owned()
	}

	fn name() -> &'static str {
		"values"
	}

	fn validate(
		&self,
		value: &kdl::KdlValue,
		node: &kdl::KdlNode,
		data: &mut State<TStruct>,
	) -> Result<(), Error> {
		use kdl::KdlValue as Kdl;
		match (self, value) {
			(Self::Int(validation), Kdl::Int(prim)) => match validation {
				None => Ok(()),
				Some(validation) => validation.validate(prim, value, data),
			},
			(Self::Float(validation), Kdl::Float(prim)) => match validation {
				None => Ok(()),
				Some(validation) => validation.validate(prim, value, data),
			},
			(Self::String(validation), Kdl::String(prim)) => match validation {
				None => Ok(()),
				Some(validation) => validation.validate(prim, value, data),
			},
			(Self::Boolean, Kdl::Boolean(_value)) => Ok(()),
			(Self::Null, Kdl::Null) => Ok(()),
			(expected, value) => Err(Error::ValueDoesNotMatch(
				expected.clone(),
				value.clone(),
				node.clone(),
			)),
		}
	}
}
