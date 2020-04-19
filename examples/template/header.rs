#![feature(proc_macro_hygiene)]

extern crate tokio;
extern crate iref;
#[macro_use]
extern crate static_iref;
extern crate json_ld;

use std::fs::File;
use std::io::{{Read, BufReader}};
use tokio::runtime::Runtime;
use iref::{{Iri, IriBuf}};
use json_ld::{{
	context::{{
		ActiveContext,
		JsonLdContextLoader,
		Context,
		LocalContext
	}},
	AsJson,
	json_ld_eq
}};

fn positive_test(expand_context: Option<&str>, input_url: Iri, input_filename: &str, output_filename: &str) {{
	let mut runtime = Runtime::new().unwrap();
	let mut loader = JsonLdContextLoader::new();

	let input_file = File::open(input_filename).unwrap();
	let mut input_buffer = BufReader::new(input_file);
	let mut input_text = String::new();
	input_buffer.read_to_string(&mut input_text).unwrap();
	let input = json::parse(input_text.as_str()).unwrap();

	let output_file = File::open(output_filename).unwrap();
	let mut output_buffer = BufReader::new(output_file);
	let mut output_text = String::new();
	output_buffer.read_to_string(&mut output_text).unwrap();
	let output = json::parse(output_text.as_str()).unwrap();

	let mut input_context: Context<IriBuf> = Context::new(input_url, input_url);

	if let Some(context_filename) = expand_context {{
		let context_file = File::open(context_filename).unwrap();
		let mut context_buffer = BufReader::new(context_file);
		let mut context_text = String::new();
		context_buffer.read_to_string(&mut context_text).unwrap();
		let mut doc = json::parse(context_text.as_str()).unwrap();
		input_context = runtime.block_on(doc.remove("@context").process(&input_context, &mut loader, Some(input_url), false, false, true)).unwrap();
	}}

	let result = runtime.block_on(json_ld::expand(&input_context, None, &input, Some(input_url), &mut loader)).unwrap();

	let result_json = result.as_json();
	let success = json_ld_eq(&result_json, &output);

	if !success {{
		println!("output=\n{{}}", result_json.pretty(2));
		println!("\nexpected=\n{{}}", output.pretty(2));
	}}

	assert!(success)
}}

fn negative_test(expand_context: Option<&str>, input_url: Iri, input_filename: &str, output_filename: &str) {{
	//
}}