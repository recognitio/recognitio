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
use rand::seq::SliceRandom;

use stdweb::js_export;

extern crate ontologica;

use ontologica::Ontology;

use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Serialize, Deserialize)]
struct Question //TODO: using the terminology already established for QANTA, struct Question: sequence: String, response: String
{
	challenge: String,
	response: String,
}
js_serializable!(Question);
js_deserializable!(Question);

struct RelativeOntology
{
	vertices: HashSet::<String>,
	hypernyms: Vec<String>,
	hyponyms: Vec<String>,
	holonyms: Vec<String>,
	meronyms: Vec<String>,
	types: Vec<String>,
	tokens: Vec<String>,
}

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

fn cascaded_clue(rel: &mut RelativeOntology, clues: Vec<&Fn(&mut RelativeOntology) -> Option<AsciiString>>) -> Option<AsciiString>
{
	for clue in clues
	{
		match (*clue)(rel)
		{
			Some(phrase) => return Some(phrase),
			None => continue,
		}
	}

	None
}

fn imperative(hypernym: AsciiString) -> AsciiString
{
	let mut clause = AsciiString::new();
	let mut rng = thread_rng();
	
	clause.push_str(&astring!(["name this ", "identify this "].choose(&mut rng).unwrap().clone()));

	clause.push_str(&astring!(["type of ", "kind of "].choose(&mut rng).unwrap().clone()));

	clause.push_str(hypernym.as_ascii_str().unwrap());

	clause
}

fn extensive_relative(rel: &mut RelativeOntology) -> AsciiString
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

	while examples.len() < max_examples
	{
		match rel.hyponyms.pop()
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

fn terminal_sentence(rel: &mut RelativeOntology) -> AsciiString
{
	let mut sentence = imperative(AsciiString::from_ascii(rel.hypernyms.pop().unwrap_or("entity".to_string()).clone()).unwrap()); //unsafe

	sentence.push_str(&extensive_relative(rel));


//TODO

	sentence.push(achar!('.'));
	sentence[0].make_ascii_uppercase();

	sentence
}

#[js_export]
fn generate_question(ontology_source: String) -> Question
{	
	let mut rng = thread_rng();
	let ontology: Ontology = serde_json::from_str(&ontology_source).unwrap_or(Ontology::new());

	let mut question = AsciiString::new();

	let mut vertices = HashSet::<String>::from_iter(ontology.vertex_labels().into_iter());

	let concept = ontology.vertex_labels().choose(&mut rng).unwrap().clone();

	let mut rel = 
	RelativeOntology
	{
		vertices: HashSet::from_iter(ontology.vertex_labels().into_iter()),
		hypernyms: ontology.hypernyms(&concept),
		hyponyms: ontology.hyponyms(&concept),
		holonyms: ontology.holonyms(&concept),
		meronyms: ontology.meronyms(&concept),
		types: ontology.types(&concept),
		tokens: ontology.tokens(&concept),
	};

	rel.hypernyms.as_mut_slice().shuffle(&mut rng);
	rel.hyponyms.as_mut_slice().shuffle(&mut rng);
	rel.holonyms.as_mut_slice().shuffle(&mut rng);
	rel.meronyms.as_mut_slice().shuffle(&mut rng);
	rel.types.as_mut_slice().shuffle(&mut rng);
	rel.tokens.as_mut_slice().shuffle(&mut rng);

	question.push_str(&terminal_sentence(&mut rel));

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