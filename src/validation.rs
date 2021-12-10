use crate::{Error, State};

/// Some basic validations for [`values`](crate::Value).
#[derive(Debug, Clone)]
pub enum Validation<TPrimitive> {
	/// Passes validation if the file-provided value is in the set of aliases provided by [`Name::Variable`](crate::Name::Variable).
	/// [`MissingCollectionValue`](crate::Error::MissingCollectionValue) will be emitted if validation fails.
	IsInVariable(/*collection id*/ &'static str),
	/// Passes validation if the file-provided value is in a discrete set of code-specified values.
	/// [`ValueInvalid`](crate::Error::ValueInvalid) will be emitted if validation fails.
	InList(Vec<TPrimitive>),
}

impl<TPrim> Validation<TPrim>
where
	TPrim: std::fmt::Debug + PartialEq,
{
	pub(crate) fn validate<TStruct>(
		&self,
		primitive: &TPrim,
		value: &kdl::KdlValue,
		data: &mut State<TStruct>,
	) -> Result<(), Error> {
		match self {
			Self::IsInVariable(collection_id) => {
				// add the value to the data to be validated once all nodes have been visited (in case the alias is declared after its used).
				data.add_collection_validation(collection_id, value.clone());
				Ok(())
			}
			Self::InList(options) => match options.contains(primitive) {
				true => Ok(()),
				false => Err(Error::ValueInvalid(value.clone(), format!("{:?}", options))),
			},
		}
	}
}
