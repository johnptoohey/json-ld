#[test]
fn {}() {{
	let input_url = iri!("{}");
	let base_url = iri!("{}");
	let output_url = iri!("{}");
	println!("{}");{}
	positive_test(
		Options {{
			processing_mode: ProcessingMode::{:?},
			expand_context: {}
		}},
		input_url,
		base_url,
		output_url
	)
}}
