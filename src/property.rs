use crate::Value;

/// Schema validation to apply to the [`properties`](kdl::KdlNode::properties) of a [`Node`](crate::Node).
#[derive(Debug, Clone)]
pub struct Property {
	pub name: &'static str,
	pub value: Value,
	/// True if this property can be omitted by files and still have a value.
	pub optional: bool,
}

impl Default for Property {
	fn default() -> Self {
		Self {
			name: "",
			value: Value::Null,
			optional: false,
		}
	}
}
