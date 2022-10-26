pub fn value_as_string(node: &kdl::KdlNode, index: usize) -> Option<&String> {
	match node.get(index) {
		Some(entry) => match entry.value() {
			kdl::KdlValue::String(asset_str) => Some(asset_str),
			_ => None,
		},
		_ => None,
	}
}
