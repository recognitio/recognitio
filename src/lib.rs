#[macro_use]
extern crate stdweb;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate ascii;
use ascii::*;

extern crate rand;
use rand::*;

use stdweb::js_export;

extern crate ontologica;

use ontologica::Ontology;

#[derive(Serialize, Deserialize)]
struct Question //TODO: using the terminology already established for QANTA, struct Question: sequence: String, response: String
{
	challenge: String,
	response: String,
}
js_serializable!(Question);
js_deserializable!(Question);

macro_rules! astring
{
	($s:expr) =>
	{
		unsafe
		{
			AsciiString::from_ascii_unchecked($s)
		}
	};
}

macro_rules! achar
{
	($s:expr) =>
	{
		unsafe
		{
			AsciiChar::from_unchecked($s)
		}
	};
}

pub fn imperative(supernym: AsciiString) -> AsciiString
{
	let mut clause = astring!(if rand::random() {"name this "} else {"identify this "});

	clause.push_str(supernym.as_ascii_str().unwrap());

	clause
}

pub fn extensive_relative(hyponyms: &mut impl Iterator<Item = String>) -> AsciiString
{
	fn any_extension(examples: &AsciiString) -> AsciiString
	{
		let mut ext = astring!("exemplified by ");
		ext.push_str(&examples);
		ext
	}
	
	fn plural_extension(examples: &AsciiString) -> AsciiString
	{
		let mut ext = astring!("examples of which include ");
		ext.push_str(&examples);
		ext
	}

	let mut clause = astring!(", ");

	let mut max_examples: usize = thread_rng().gen_range(1, 4);
	let mut examples = Vec::new();

	while examples.len() <= max_examples
	{
		match hyponyms.next()
		{
			Some(hyponym) => examples.push(hyponym),
			None => break,
		}
	}

	let list = 
	astring!
	(
		{
			if examples.len() == 1
			{
				examples[0].clone()
			}
			else if examples.len() == 2
			{
				let mut r = examples[0].clone();
				r.push_str(" and ");
				r.push_str(examples[1].as_str());
				r
			}
			else if examples.len() == 3
			{
				let mut r = examples[0].clone();
				r.push_str(", ");
				r.push_str(examples[1].as_str());
				r.push_str(" and ");
				r.push_str(examples[2].as_str());
				r
			}
			else
			{
				"".to_string()
			}
		}
	);

	let ext = if rand::random() || examples.len() == 1 {any_extension(&list)} else {plural_extension(&list)};
	clause.push_str(&ext);

	if examples.len() >= 1 {clause} else {astring!("")}
}

fn nonterminal_sentence(_ontology: &Ontology) -> AsciiString
{
	let mut sentence: AsciiString;

	astring!("")
}

fn terminal_sentence(hypernyms: &mut impl Iterator<Item = String>, hyponyms: &mut impl Iterator<Item = String>) -> AsciiString
{
	let mut sentence = imperative(AsciiString::from_ascii(hypernyms.next().unwrap_or("entity".to_string()).clone()).unwrap()); //unsafe

	sentence.push_str(&extensive_relative(hyponyms));


//TODO

	sentence.push(achar!('.'));
	sentence[0].make_ascii_uppercase();

	sentence
}

#[js_export]
fn generate_question(ontology_source: String) -> Question
{
	let ontology: Ontology = serde_json::from_str(&ontology_source).unwrap_or(Ontology::new());

	let mut question = AsciiString::new();

	let mut vertices = ontology.vertex_labels();

	let concept = vertices[thread_rng().gen_range(0, vertices.len())].clone();//

	let _hypernyms = ontology.hypernyms(&concept);
	let mut hypernyms = _hypernyms.iter().map(|s| s.clone());

	let _hyponyms = ontology.hyponyms(&concept);
	let mut hyponyms = _hyponyms.iter().map(|s| s.clone());

	question.push_str(&terminal_sentence(&mut hypernyms, &mut hyponyms));

	Question {challenge: question.as_str().to_string(), response: concept}
}

#[js_export]
fn challenge(question: Question) -> String
{
	question.challenge
}

#[js_export]
fn response(question: Question) -> String
{
	question.response
}

#[js_export]
fn load_ontology(ontology_source: String) -> String
{
	let ontology = ontologica::parse_ontology_source(&ontology_source);

	serde_json::to_string(&ontology).unwrap()
}