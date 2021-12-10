use crate::Error;

pub(crate) type Collections = std::collections::HashMap<&'static str, Collection>;

#[derive(Default)]
pub(crate) struct Collection {
	pub id: &'static str,
	pub names: Vec<kdl::KdlValue>,
	pub values_to_validate: Vec<kdl::KdlValue>,
}

impl Collection {
	pub fn validate(&self) -> Result<(), Error> {
		for value in self.values_to_validate.iter() {
			if !self.names.contains(value) {
				return Err(Error::MissingCollectionValue(
					self.id,
					value.clone(),
					self.names.clone(),
				));
			}
		}
		Ok(())
	}
}
