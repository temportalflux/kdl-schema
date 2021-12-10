use crate::{Error, State, Validatable};

/// A list of items ([`values`](crate::Value) or [`nodes`](crate::Node)).
#[derive(Clone)]
pub enum Items<TValue> {
	/// A discrete ordered set of items. The document must provide
	/// the same number of items and the items will be validated
	/// in the same order the schema defines.
	Ordered(Vec<TValue>),
	/// A variable number of items where each item must match any
	/// one of the validations provided by the schema.
	Select(Vec<TValue>),
}

impl<TItemType> std::fmt::Display for Items<TItemType>
where
	TItemType: std::fmt::Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Self::Ordered(ordered) => write!(
				f,
				"Ordered(items={})",
				ordered
					.iter()
					.map(|t| format!("{},", t))
					.collect::<String>()
			),
			Self::Select(options) => write!(
				f,
				"Select(options={})",
				options
					.iter()
					.map(|t| format!("{},", t))
					.collect::<String>()
			),
		}
	}
}

impl<TItemType> Items<TItemType> {
	pub(crate) fn validate<TStruct, TKdlValue>(
		&self,
		parent: Option<&kdl::KdlNode>,
		items: &Vec<TKdlValue>,
		data: &mut State<TStruct>,
	) -> Result<(), Error>
	where
		TItemType: Validatable<TKdlValue, TStruct> + std::fmt::Display,
		TKdlValue: std::fmt::Display,
	{
		match self {
			Self::Ordered(expected) => {
				for i in 0..expected.len() {
					if i >= items.len() {
						return Err(Error::MissingItem(
							parent.cloned(),
							i,
							expected[i].as_string(),
							expected.len(),
						));
					}
					expected[i].validate(&items[i], parent, data)?;
				}
				if items.len() > expected.len() {
					return Err(Error::TooManyItems(
						parent.cloned(),
						expected.len(),
						TItemType::name(),
					));
				}
			}
			Self::Select(options) => {
				for value in items.iter() {
					let mut found_option = false;
					let mut option_errors = vec![];
					for option in options.iter() {
						match option.validate(&value, parent, data) {
							Ok(()) => {
								found_option = true;
								break;
							}
							Err(err) => option_errors.push(err),
						}
					}
					if !found_option {
						return Err(Error::ItemNotInOptions(
							format!("{}", value),
							parent.cloned(),
							format!(
								"[{}]",
								options.iter().map(|v| format!("{}", v)).collect::<String>()
							),
							option_errors,
						));
					}
				}
			}
		}
		Ok(())
	}
}
