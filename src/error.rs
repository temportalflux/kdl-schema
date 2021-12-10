
/// Errors that can be results of [`kdl validation`](`crate::Schema::parse_and_validate`).
#[derive(Debug)]
pub enum Error {
	/// There was an error in kdl during [`parse_document`](kdl::parse_document).
	KdlError(kdl::KdlError),
	/// There is a node at the root level which does not have a matching schema.
	NodeHasNoSchema(kdl::KdlNode),
	/// There is a schema missing for an item in a [`values`](crate::Node::values) or [`children`](crate::Node::children) list.
	MissingItem(kdl::KdlNode, usize, String, usize),
	/// A provided [`Value`](kdl::KdlValue) has a conflicting type with its [`schema`](crate::Value) in some node.
	ValueDoesNotMatch(crate::Value, kdl::KdlValue, kdl::KdlNode),
	/// A provided [`Value`](kdl::KdlValue) is not in a list of valid values for a [`Validation::InList`](crate::Validation::InList).
	ValueInvalid(kdl::KdlValue, String),
	/// There are too many items in a provided [`schema list`](crate::Items::Ordered).
	/// This could be a value or a child node.
	TooManyItems(kdl::KdlNode, usize, &'static str),
	/// There is an item provided that is not valid for a [`schema list`](crate::Items::Select).
	/// This could be a value or a child node.
	ItemNotInOptions(String, kdl::KdlNode, String, Vec<Error>),
	/// There is a property provided that is not specified by the [`schema`](crate::Node::properties) for the node.
	PropertyNotInSchema(String, Vec<&'static str>, kdl::KdlNode),
	/// There is one or more [`non-optional`](crate::Property::optional) properties that are not provided in a [`kdl node`](kdl::KdlNode).
	MissingProperties(std::collections::HashSet<&'static str>, kdl::KdlNode),
	/// There is a [`collection value`](crate::Validation::IsInVariable) provided that [`was not added`](crate::Name::Variable) to the collection.
	MissingCollectionValue(&'static str, kdl::KdlValue, Vec<kdl::KdlValue>),
}
impl std::error::Error for Error {}
impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::KdlError(kdl_err) => kdl_err.fmt(f),
			Self::NodeHasNoSchema(kdl_node) => write!(
				f,
				"No schema node for kdl node named \"{}\": {}",
				kdl_node.name, kdl_node
			),
			Self::MissingItem(kdl_node, value_index, schema_string, expected_count) => write!(
				f,
				"Node({}) is missing value at index {}. {} values are expected, per the schema {}. Full node: {}",
				kdl_node.name, value_index, expected_count, schema_string, kdl_node
			),
			Self::ValueDoesNotMatch(expected, kdl_value, node) => write!(
				f,
				"Expected type {} but found {} for node {}",
				expected.name(),
				kdl_value,
				node
			),
			Self::ValueInvalid(value, stringified_options) => write!(
				f,
				"Value \"{}\" is not in the list of value options: {}",
				value, stringified_options
			),
			Self::TooManyItems(node, expected_count, kind) =>
				write!(
				f, "Too many {} in node({}), Found {} {}, but only expected {}. Full node: {}",
				kind, node.name, node.values.len(), kind, expected_count, node
			),
			Self::ItemNotInOptions(value, node, options_string, option_errors) => write!(
				f,
				"\"{}\" is not an option in the schema {}. Errors: {}, Full node: {}",
				value, options_string, option_errors.iter().map(|err| format!("{},", err)).collect::<String>(), node
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
