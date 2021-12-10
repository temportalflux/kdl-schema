pub fn value_as_string(node: &kdl::KdlNode, index: usize) -> Option<&String> {
	match node.values.get(index) {
		Some(kdl::KdlValue::String(asset_str)) => Some(asset_str),
		_ => None,
	}
}
