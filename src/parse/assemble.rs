use parse::*;
use cell::Cell;
use error::SRLError;
use misc::*;
use gen::*;

// remove optional outer parens
// does not remove parens here: (a b)(c d), but here (a b c d)
fn trim_tokens(mut vec : Vec<String>) -> Vec<String> {
	loop {
		let len = vec.len();

		if len < 2 || (! (vec[0] == "(" && vec[len-1] == ")")) {
			return vec;
		}

		let mut parens = 0;

		for index in 0..len {
			let element : String = vec[index].clone();

			if element == "(" { parens += 1; }
			else if element == ")" { parens -= 1; }

			if parens == 0 && index > 0 && index < len-1 {
				return vec;
			}
		}

		vec.remove(len-1);
		vec.remove(0);
	}
}

#[test]
fn test_trim_tokens() {
	let mut tokens : Vec<String>;

	tokens = vec!["(".to_string(), "wow".to_string(), "nice".to_string(), ")".to_string(), "(".to_string(), "very".to_string(), "interesting".to_string(), ")".to_string()];
	assert_eq!(trim_tokens(tokens.clone()), tokens);

	tokens = vec!["(".to_string(), "wow".to_string(), "nice".to_string(), ")".to_string()];
	assert_eq!(trim_tokens(tokens), vec!["wow".to_string(), "nice".to_string()]);

	tokens = vec!["(".to_string(), "{".to_string(), "}".to_string(), ")".to_string()];
	assert_eq!(trim_tokens(tokens), vec!["{".to_string(), "}".to_string()]);
}

fn simple_by_trimmed_tokens(tokens : Vec<String>) -> Result<Cell, SRLError> {
	if tokens.len() != 1 {
		return Err(SRLError("simple_by_trimmed_tokens".to_string(), "tokens.len() != 1".to_string()));
	}

	return Ok(try_simple(tokens[0].clone())?);
}

fn find_cell_ending(cell_start : usize, tokens : &Vec<String>) -> Option<usize> {
	let len = tokens.len();

	let mut parens : i32 = 0;

	for index in cell_start..len {
		if tokens[index] == "(" || tokens[index] == "{" {
			parens += 1;
		} else if tokens[index] == ")" || tokens[index] == "}" {
			parens -= 1;
		}

		if parens == 0 {
			return Some(index);
		}
	}
	None
}

fn complex_by_trimmed_tokens(tokens : Vec<String>) -> Result<Cell, SRLError> {
	let mut cells : Vec<Cell> = Vec::new();
	let mut index = 0;
	let len = tokens.len();

	while index_in_len(index, len) {
		let ending = match find_cell_ending(index, &tokens) {
			Some(x) => x,
			None => break
		};
		let subtokens : Vec<String> = tokens[index..ending+1].to_vec();
		let cell = assemble(subtokens)?;
		cells.push(cell);
		index = ending + 1;
	}

	Ok(complex(cells))
}

#[test]
fn test_complex_by_trimmed_tokens() {
	assert_eq!(
		complex_by_trimmed_tokens(vec!["a".to_string(), "b".to_string()]).unwrap(),
		complex(vec![simple_by_str("a"), simple_by_str("b")])
	);
}

// accepts {0 (a b)} as well as {0 a b}
fn scope_by_trimmed_tokens(mut tokens : Vec<String>) -> Result<Cell, SRLError> {
	// cut { and }
	let len = tokens.len();

	if len < 3 { return Err(SRLError("scope_by_trimmed_tokens".to_string(), "tokens.len() < 3".to_string())); }
	if "}" != &tokens.remove(len-1) { return Err(SRLError("scope_by_trimmed_tokens".to_string(), "\"}\" != &tokens.remove(len-1)".to_string())); }
	if "{" != &tokens.remove(0) { return Err(SRLError("scope_by_trimmed_tokens".to_string(), "\"{\" != &tokens.remove(0)".to_string())); }

	let id = match var_by_trimmed_tokens(vec![tokens.remove(0)]) {
		Ok(Cell::Var { id : x }) => x,
		Ok(_) => return Err(SRLError("scope_by_trimmed_tokens".to_string(), "this is not a var cell".to_string())),
		Err(srl_error) => return Err(srl_error)
	};
	let body = assemble(tokens)?;
	return Ok(scope(id, body));
}

#[test]
fn test_scope_by_trimmed_tokens() {
	assert_eq!(scope(0, simple_by_str("b")), scope_by_trimmed_tokens(vec!["{".to_string(), "0".to_string(), "b".to_string(), "}".to_string()]).unwrap());
	assert_eq!(scope(0, complex(vec![simple_by_str("b"), simple_by_str("c")])), scope_by_trimmed_tokens(vec!["{".to_string(), "0".to_string(), "b".to_string(), "c".to_string(), "}".to_string()]).unwrap());

	match scope_by_trimmed_tokens(vec!["0".to_string(), "b".to_string(), "c".to_string(), "}".to_string()]) {
		Ok(_) => panic!("test_scope_by_trimmed_tokens should fail here (0)"),
		_ => {}
	}
	match scope_by_trimmed_tokens(vec!["0".to_string(), "b".to_string(), "c".to_string()]) {
		Ok(_) => panic!("test_scope_by_trimmed_tokens should fail here (1)"),
		_ => {}
	}
	match scope_by_trimmed_tokens(vec!["{".to_string(), "}".to_string()]) {
		Ok(_) => panic!("test_scope_by_trimmed_tokens should fail here (2)"),
		_ => {}
	}
	match scope_by_trimmed_tokens(vec!["{".to_string(), "0".to_string(), "}".to_string()]) {
		Ok(_) => panic!("test_scope_by_trimmed_tokens should fail here (3)"),
		_ => {}
	}
	match scope_by_trimmed_tokens(vec!["{".to_string(), "a".to_string(), "}".to_string()]) {
		Ok(_) => panic!("test_scope_by_trimmed_tokens should fail here (4)"),
		_ => {}
	}
	match scope_by_trimmed_tokens(vec!["{".to_string(), "a".to_string(), "b".to_string(), "}".to_string()]) {
		Ok(_) => panic!("test_scope_by_trimmed_tokens should fail here (5)"),
		_ => {}
	}
	match scope_by_trimmed_tokens(vec!["a".to_string(), "b".to_string(), "c".to_string()]) {
		Ok(_) => panic!("test_scope_by_trimmed_tokens should fail here (6)"),
		_ => {}
	}
}

fn var_by_trimmed_tokens(tokens : Vec<String>) -> Result<Cell, SRLError> {
	if tokens.len() != 1 {
		return Err(SRLError("var_by_trimmed_tokens".to_string(), "tokens.len() != 1".to_string()));
	}

	match tokens[0].parse::<u32>() {
		Ok(x) => return Ok(var(x)),
		Err(_) => return Err(SRLError("var_by_trimmed_tokens".to_string(), format!("failed parsing of '{}'", tokens[0])))
	};
}

#[test]
fn test_var_by_trimmed_tokens() {
	assert_eq!(var(0), var_by_trimmed_tokens(vec!["0".to_string()]).unwrap());
	match var_by_trimmed_tokens(vec!["a".to_string()]) {
		Ok(_) => panic!("should fail here (0)"),
		_ => {}
	}
	match var_by_trimmed_tokens(vec!["0".to_string(), "other".to_string()]) {
		Ok(_) => panic!("should fail here (1)"),
		_ => {}
	}
}

// consumes *all* tokens to create one Cell
// -- used to parse rules, does not accept cases
pub fn assemble(mut tokens : Vec<String>) -> Result<Cell, SRLError> {
	tokens = trim_tokens(tokens);

	let len = tokens.len();

	if len == 0 {
		return Err(SRLError("assemble".to_string(), "tokens.len() == 0".to_string()));
	} else if tokens.len() == 1 {
		let token : String = tokens[0].clone();
		if is_var_token(&token) {
			return var_by_trimmed_tokens(tokens);
		} else if is_simple_token(&token) {
			return simple_by_trimmed_tokens(tokens);
		} else {
			return Err(SRLError("assemble".to_string(), format!("lone token '{}' is weird", token)));
		}
	} else if tokens[0] == "{" && tokens[len-1] == "}" {
		return scope_by_trimmed_tokens(tokens);
	} else {
		return complex_by_trimmed_tokens(tokens);
	}
}

// see assemble
pub fn assemble_str(tokens : Vec<&str>) -> Result<Cell, SRLError> {
	let mut v : Vec<String> = Vec::new();
	for token in tokens {
		v.push(token.to_string());
	}
	assemble(v)
}

#[test]
fn test_assemble() {
	assert_eq!(complex(vec![simple_by_str("a"), simple_by_str("b")]),
		assemble_str(vec!["(", "a", ")", "b"]).unwrap());
	assert_eq!(simple_by_str("wow"),
		assemble_str(vec!["wow"]).unwrap());
	assert_eq!(complex(vec![simple_by_str("equals"), simple_by_str("a"), simple_by_str("b")]),
		assemble_str(vec!["(", "equals", "a", "b", ")"]).unwrap());
}

// checks for problems like unclosed parens, and ({)}-like things
pub fn check_paren_correctness(mut tokens : Vec<String>) -> bool {
	let mut index = 0;
	while index_in_len(index, tokens.len()) {
		match "{}[]()".to_string().find(&tokens[index]) {
			Some(_) => {
				index += 1;
			},
			None => {
				tokens.remove(index);
			}
		}
	}

	let mut string = tokens.join("");
	loop {
		match string.find("{}") {
			Some(i) => {
				string.remove(i);
				string.remove(i);
				continue;
			},
			None => {}
		};
		match string.find("()") {
			Some(i) => {
				string.remove(i);
				string.remove(i);
				continue;
			},
			None => {}
		};
		match string.find("[]") {
			Some(i) => {
				string.remove(i);
				string.remove(i);
				continue;
			},
			None => {}
		};
		break;
	}

	return string.is_empty();
}

#[test]
fn test_check_paren_correctness() {
	assert_eq!(check_paren_correctness(vec!["{".to_string(), "(".to_string(), "}".to_string(), ")".to_string()]), false);
	assert_eq!(check_paren_correctness(vec!["{".to_string(), "(".to_string(), ")".to_string(), "}".to_string()]), true);
	assert_eq!(check_paren_correctness(vec!["{".to_string(), "testy".to_string(), "(".to_string(), ")".to_string(), "}".to_string()]), true);
	assert_eq!(check_paren_correctness(vec!["{".to_string(), "testy".to_string(), "(".to_string(), ")".to_string(), "}".to_string()]), true);
}
