pub fn condense_tags(in_taglist: Vec<String>) -> Vec<String> {
	let mut out_taglist: Vec<String> = vec![];
	
	for tag in &in_taglist {
		let mut found_longer_path: bool = false;
		for inner_tag in &in_taglist {
			if inner_tag.starts_with(&format!("{}/", tag)) {
				found_longer_path = true;
				break;
			}
		}
		if !found_longer_path {
			out_taglist.push(tag.to_string());
		}
	}

	out_taglist
}