/// Errors that can be results of [`kdl validation`](`crate::Schema::parse_and_validate`).
#[derive(Debug)]
pub enum Error {
	/// There was an error in kdl during [`parse_document`](kdl::parse_document).
	KdlError(kdl::KdlError),
	/// The node name does not match the name in the node schema.
	NameInvalid(String, crate::Name),
	/// There is a schema missing for an item in a [`values`](crate::Node::values) or [`children`](crate::Node::children) list.
	MissingItem(Option<kdl::KdlNode>, usize, String, usize),
	/// A provided [`Value`](kdl::KdlValue) has a conflicting type with its [`schema`](crate::Value) in some node.
	ValueDoesNotMatch(crate::Value, kdl::KdlValue, Option<kdl::KdlNode>),
	/// A provided [`Value`](kdl::KdlValue) is not in a list of valid values for a [`Validation::InList`](crate::Validation::InList).
	ValueInvalid(kdl::KdlValue, String),
	/// There are too many items in a provided [`schema list`](crate::Items::Ordered).
	/// This could be a value or a child node.
	TooManyItems(Option<kdl::KdlNode>, usize, &'static str),
	/// There is an item provided that is not valid for a [`schema list`](crate::Items::Select).
	/// This could be a value or a child node.
	ItemNotInOptions(String, Option<kdl::KdlNode>, String, Vec<Error>),
	/// There is a property provided that is not specified by the [`schema`](crate::Node::properties) for the node.
	PropertyNotInSchema(String, Vec<&'static str>, kdl::KdlNode),
	/// There is one or more [`non-optional`](crate::Property::optional) properties that are not provided in a [`kdl node`](kdl::KdlNode).
	MissingProperties(std::collections::HashSet<&'static str>, kdl::KdlNode),
	/// There is a [`collection value`](crate::Validation::IsInVariable) provided that [`was not added`](crate::Name::Variable) to the collection.
	MissingCollectionValue(&'static str, kdl::KdlValue, Vec<kdl::KdlValue>),
}
impl Error {
	fn parent_node_name(&self) -> &str {
		match self {
			Self::MissingItem(parent, _, _, _) => parent,
			Self::TooManyItems(parent, _, _) => parent,
			_ => return "None",
		}
		.as_ref()
		.map(|node| node.name().value())
		.unwrap_or("")
	}

	fn parent_node_display(&self) -> String {
		match self {
			Self::ValueDoesNotMatch(_, _, parent) => parent,
			Self::MissingItem(parent, _, _, _) => parent,
			Self::TooManyItems(parent, _, _) => parent,
			Self::ItemNotInOptions(_, parent, _, _) => parent,
			_ => return "None".to_owned(),
		}
		.as_ref()
		.map(|node| format!("{}", node))
		.unwrap_or(String::new())
	}
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::KdlError(kdl_err) => kdl_err.fmt(f),
			Self::NameInvalid(name, schema_name) => write!(f, "\"{}\" != {:?}", name, schema_name),
			Self::MissingItem(_, value_index, schema_string, expected_count) => write!(
				f,
				"Node({}) is missing value at index {}. {} values are expected, per the schema {}. Full node: {}",
				self.parent_node_name(), value_index, expected_count, schema_string, self.parent_node_display()
			),
			Self::ValueDoesNotMatch(expected, kdl_value, _) => write!(
				f,
				"Expected type {} but found {} for node {}",
				expected.name(),
				kdl_value,
				self.parent_node_display()
			),
			Self::ValueInvalid(value, stringified_options) => write!(
				f,
				"Value \"{}\" is not in the list of value options: {}",
				value, stringified_options
			),
			Self::TooManyItems(kdl_node, expected_count, kind) =>
				write!(
				f, "Too many {} in node({}), Found {} {}, but only expected {}. Full node: {}",
				kind, self.parent_node_name(),
				kdl_node.as_ref().map(|node| node.entries().len()).unwrap_or(0),
				kind, expected_count, self.parent_node_display()
			),
			Self::ItemNotInOptions(value, _, options_string, option_errors) => write!(
				f,
				"\"{}\" is not an option in the schema {}. Errors: {}, Full node: {}",
				value, options_string, option_errors.iter().map(|err| format!("{},", err)).collect::<String>(), self.parent_node_display()
			),
			Self::PropertyNotInSchema(name, options, node) => write!(f, "Found property named \"{}\", but it was not in the list of options {:?}. Full node: {}", name, options, node),
			Self::MissingProperties(remaining_options, node) => write!(f, "Missing properties {:?} for node {}", remaining_options, node),
			Self::MissingCollectionValue(collection_id, value, options) => write!(f, "Found value \"{}\" in collection \"{}\", but no such value was defined. Valid values are: {:?}", value, collection_id, options),
		}
	}
}

impl std::convert::From<kdl::KdlError> for Error {
	fn from(kdl_error: kdl::KdlError) -> Self {
		Self::KdlError(kdl_error)
	}
}
