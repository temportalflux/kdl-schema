/// The name of a node, which can be mandated to a specific value,
/// or can be used as an alias for some longer string that the `.kdl` author specifies.
#[derive(Debug, Clone)]
pub enum Name {
	Defined(/*literal*/ &'static str),
	Variable(/*collection id*/ &'static str),
}

impl Name {
	pub(crate) fn supports(&self, name: &str) -> bool {
		match *self {
			Self::Defined(literal) => name == literal,
			Self::Variable(_collection_id) => true,
		}
	}
}
