use crate::{Error, Items, Node, State};

/// Parent/root structure for handling the validation of [`kdl`](kdl) file content,
/// (beyond just grammar correctness).
pub struct Schema<TStruct> {
	/// A list of schemas for all the nodes that can appear in the root layer of the document.
	pub nodes: Items<Node<TStruct>>,
	/// Optional: Callback provided which is executed when the `parse_and_validate` completes successfully.
	pub on_validation_successful: Option<fn(&mut TStruct)>,
}

impl<TStruct> Default for Schema<TStruct> {
	fn default() -> Self {
		Self {
			nodes: Items::Select(vec![]),
			on_validation_successful: None,
		}
	}
}

impl<TStruct> Schema<TStruct> {
	/// [`Parses a kdl document`](kdl::parse_document) and
	/// validates the resulting contents according to the schema.
	pub fn parse_and_validate(&self, content: &str) -> Result<TStruct, Error>
	where
		TStruct: Default,
	{
		let doc = content.parse::<kdl::KdlDocument>()?;
		let mut data = State::<TStruct>::default();

		// Validate each node in the root-layer, validating all child nodes
		// in the process. This is a depth-first operation where each
		// node visited is validated and then its children are validated.
		self.nodes.validate(None, &doc.nodes(), &mut data)?;

		// Validate any dynamicly named/aliased values
		for (_id, collection) in data.collections.iter() {
			collection.validate()?;
		}

		if let Some(callback) = &self.on_validation_successful {
			callback(&mut data.output);
		}

		Ok(data.output)
	}
}
