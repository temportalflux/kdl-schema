use crate::{Error, State, Validatable, Validation};

/// Schema equivalent of [`KdlValue`](kdl::KdlValue).
/// Provides validation for a handful of primitives.
#[derive(Debug, Clone)]
pub enum Value {
	IntBase2(Option<Validation<i64>>),
	IntBase8(Option<Validation<i64>>),
	IntBase10(Option<Validation<i64>>),
	IntBase16(Option<Validation<i64>>),
	FloatBase10(Option<Validation<f64>>),
	String(Option<Validation<String>>),
	Boolean,
	Null,
}

impl Value {
	/// Returns the stringified simple identifier of the value-schema.
	pub(crate) fn name(&self) -> &'static str {
		match self {
			Self::IntBase2(_) => "Binary",
			Self::IntBase8(_) => "Oct",
			Self::IntBase10(_) => "Integer",
			Self::IntBase16(_) => "Hexadecimal",
			Self::FloatBase10(_) => "Float",
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
		parent: Option<&kdl::KdlNode>,
		data: &mut State<TStruct>,
	) -> Result<(), Error> {
		use kdl::KdlValue as Kdl;
		match (self, value) {
			(Self::IntBase2(validation), Kdl::Base2(prim)) => match validation {
				None => Ok(()),
				Some(validation) => validation.validate(prim, value, data),
			},
			(Self::IntBase8(validation), Kdl::Base8(prim)) => match validation {
				None => Ok(()),
				Some(validation) => validation.validate(prim, value, data),
			},
			(Self::IntBase10(validation), Kdl::Base10(prim)) => match validation {
				None => Ok(()),
				Some(validation) => validation.validate(prim, value, data),
			},
			(Self::IntBase16(validation), Kdl::Base16(prim)) => match validation {
				None => Ok(()),
				Some(validation) => validation.validate(prim, value, data),
			},
			(Self::FloatBase10(validation), Kdl::Base10Float(prim)) => match validation {
				None => Ok(()),
				Some(validation) => validation.validate(prim, value, data),
			},
			(Self::String(validation), Kdl::String(prim)) => match validation {
				None => Ok(()),
				Some(validation) => validation.validate(prim, value, data),
			},
			(Self::Boolean, Kdl::Bool(_value)) => Ok(()),
			(Self::Null, Kdl::Null) => Ok(()),
			(expected, value) => Err(Error::ValueDoesNotMatch(
				expected.clone(),
				value.clone(),
				parent.cloned(),
			)),
		}
	}
}
