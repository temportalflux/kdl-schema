use crate::*;

/// Schema equivalent of [`KdlNode`](kdl::KdlNode).
#[derive(Clone)]
pub struct Node<TStruct> {
	pub name: Name,
	pub values: Items<Value>,
	pub properties: Vec<Property>,
	pub children: Items<Node<TStruct>>,
	/// Executed when validation has finished and is successful (if provided).
	pub on_validation_successful: Option<fn(&mut TStruct, &kdl::KdlNode)>,
}

impl<TAsset> Default for Node<TAsset> {
	fn default() -> Self {
		Self {
			name: Name::Defined(""),
			values: Items::Ordered(vec![]),
			properties: vec![],
			children: Items::Ordered(vec![]),
			on_validation_successful: None,
		}
	}
}

impl<TStruct> std::fmt::Display for Node<TStruct> {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"Node(name={:?}, values={}, properties={:?}, children={})",
			self.name, self.values, self.properties, self.children
		)
	}
}

impl<TStruct> Validatable<kdl::KdlNode, TStruct> for Node<TStruct> {
	fn as_string(&self) -> String {
		format!("{}", self)
	}

	fn name() -> &'static str {
		"children"
	}

	fn validate(
		&self,
		value: &kdl::KdlNode,
		_node: &kdl::KdlNode,
		data: &mut State<TStruct>,
	) -> Result<(), Error> {
		self.validate_node(value, data)
	}
}

impl<TStruct> Node<TStruct> {
	fn validate_node(
		&self,
		node: &kdl::KdlNode,
		data: &mut State<TStruct>,
	) -> Result<(), Error> {
		// Save the names of nodes into the collective memory (so they can be used in validation once all nodes have been visited).
		if let Name::Variable(collection_id) = self.name {
			data.insert_collection_name(&collection_id, node.name.clone());
		}

		self.values.validate(&node, &node.values, data)?;
		self.validate_properties(&node, data)?;
		self.children.validate(&node, &node.children, data)?;
		if let Some(callback) = &self.on_validation_successful {
			callback(&mut data.output, &node);
		}
		Ok(())
	}

	fn validate_properties(
		&self,
		node: &kdl::KdlNode,
		data: &mut State<TStruct>,
	) -> Result<(), Error> {
		use std::iter::FromIterator;
		let valid_names = self
			.properties
			.iter()
			.map(|prop| prop.name)
			.collect::<Vec<_>>();
		let mut found_names = std::collections::HashSet::from_iter(valid_names.clone().into_iter());
		let expected_properties = self
			.properties
			.iter()
			.map(|property| (property.name, property.clone()))
			.collect::<std::collections::HashMap<&'static str, Property>>();
		for (prop_name, value) in node.properties.iter() {
			match expected_properties.get(prop_name.as_str()) {
				None => {
					return Err(Error::PropertyNotInSchema(
						prop_name.clone(),
						valid_names.clone(),
						node.clone(),
					));
				}
				Some(property) => {
					property.value.validate(value, node, data)?;
					found_names.remove(prop_name.as_str());
				}
			}
		}
		found_names.retain(|name| !expected_properties.get(name).unwrap().optional);
		if !found_names.is_empty() {
			return Err(Error::MissingProperties(found_names, node.clone()));
		}
		Ok(())
	}
}