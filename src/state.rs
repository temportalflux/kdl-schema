use crate::{Collection, Collections};

#[derive(Default)]
pub(crate) struct State<TStruct> {
	pub output: TStruct,
	pub collections: Collections,
}

impl<TStruct> State<TStruct> {
	fn ensure_collection_exists(&mut self, collection_id: &'static str) {
		if !self.collections.contains_key(&collection_id) {
			let mut collection = Collection::default();
			collection.id = collection_id;
			self.collections.insert(collection_id, collection);
		}
	}

	pub fn insert_collection_name(&mut self, collection_id: &'static str, name: String) {
		self.ensure_collection_exists(collection_id);
		self.collections
			.get_mut(&collection_id)
			.unwrap()
			.names
			.push(name.into());
	}

	pub fn add_collection_validation(&mut self, collection_id: &'static str, value: kdl::KdlValue) {
		self.ensure_collection_exists(collection_id);
		self.collections
			.get_mut(&collection_id)
			.unwrap()
			.values_to_validate
			.push(value.into());
	}
}
