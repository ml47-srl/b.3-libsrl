use error::SRLError;
use super::*;

// splits string into tokens, fix_whitespaces has to be called prior. Defined behaviour only for chars in VALID_CHARS without \n \t and .
pub fn tokenize(mut string : String) -> Result<Vec<String>, SRLError> {
	let mut tokens : Vec<String> = Vec::new();

	#[allow(non_camel_case_types)]
	enum State { NONE, VAR, SIMPLE, CONST, EQ, AFTER_CELL };
	let mut state : State = State::NONE;

	let mut tmp_string : String = String::new();

	loop {
		if string.is_empty() {
			if ! tmp_string.is_empty() {
				tokens.push(tmp_string);
			}
			break;
		}
		match (string.remove(0), &state) {
			(x, &State::NONE) => {
				if PARENS.contains(x) {
					tokens.push(x.to_string())
				} else if x == ' ' {
					/* nothing */
				} else if x == '\'' {
					state = State::CONST;
					tmp_string.push('\'');
				} else if x == '=' {
					state = State::EQ;
				} else if SIMPLE_CELL_FILL_CHARS.contains(x) {
					tmp_string.push(x);
					state = State::SIMPLE;
				} else if VAR_FILL_CHARS.contains(x) {
					tmp_string.push(x);
					state = State::VAR;
				} else {
					return Err(SRLError("split_tokens".to_string(), format!("forgot handling for '{}' in NONE state", x)))
				}
			}
			(x, &State::VAR) => {
				if PARENS.contains(x) {
					tokens.push(tmp_string);
					tokens.push(x.to_string());
					tmp_string = String::new();
					state = State::NONE;
				} else if x == ' ' {
					tokens.push(tmp_string);
					tmp_string = String::new();
					state = State::NONE;
				} else if x == '\'' {
					return Err(SRLError("split_tokens".to_string(), format!("trying to close var {} with constant tick ' ", tmp_string)));
				} else if x == '=' {
					return Err(SRLError("split_tokens".to_string(), format!("trying to close var {} with =", tmp_string)));
				} else if SIMPLE_CELL_FILL_CHARS.contains(x) {
					return Err(SRLError("split_tokens".to_string(), format!("trying to close var {} with {}", tmp_string, x)));
				} else if VAR_FILL_CHARS.contains(x) {
					tmp_string.push(x);
				} else {
					return Err(SRLError("split_tokens".to_string(), format!("forgot handling for '{}' in VAR state", x)))
				}
			}
			(x, &State::SIMPLE) => {
				if PARENS.contains(x) {
					tokens.push(tmp_string);
					tokens.push(x.to_string());
					tmp_string = String::new();
					state = State::NONE;
				} else if x == ' ' {
					tokens.push(tmp_string);
					tmp_string = String::new();
					state = State::NONE;
				} else if x == '\'' {
					return Err(SRLError("split_tokens".to_string(), format!("trying to end simple cell \"{}\" with tick '", tmp_string)));
				} else if x == '=' {
					return Err(SRLError("split_tokens".to_string(), format!("trying to end simple cell \"{}\" with =", tmp_string)));
				} else if SIMPLE_CELL_FILL_CHARS.contains(x) {
					tmp_string.push(x);
				} else if VAR_FILL_CHARS.contains(x) {
					return Err(SRLError("split_tokens".to_string(), format!("trying to end simple cell \"{}\" with {}", tmp_string, x)));
				} else {
					return Err(SRLError("split_tokens".to_string(), format!("forgot handling for '{}' in SIMPLE state", x)))
				}
			}
			(x, &State::CONST) => {
				if PARENS.contains(x) {
					tokens.push(tmp_string);
					tokens.push(x.to_string());
					tmp_string = String::new();
					state = State::NONE;
				} else if x == ' ' {
					tokens.push(tmp_string);
					tmp_string = String::new();
					state = State::NONE;
				} else if x == '\'' {
					tmp_string.push('\'');
					tokens.push(tmp_string);
					tmp_string = String::new();
					state = State::AFTER_CELL;
				} else if x == '=' {
					return Err(SRLError("split_tokens".to_string(), format!("trying to end const cell \"{}\" with =", tmp_string)));
				} else if SIMPLE_CELL_FILL_CHARS.contains(x) {
					tmp_string.push(x);
				} else if VAR_FILL_CHARS.contains(x) {
					return Err(SRLError("split_tokens".to_string(), format!("trying to end const cell \"{}\" with {}", tmp_string, x)));
				} else {
					return Err(SRLError("split_tokens".to_string(), format!("forgot handling for '{}' in CONST state", x)));
				}
			},
			(x, &State::EQ) => {
				if PARENS.contains(x) {
					tokens.push("=".to_string());
					tokens.push(x.to_string());
					state = State::NONE;
				} else if x == ' ' {
					tokens.push("=".to_string());
					state = State::NONE;
				} else if x == '\'' {
					return Err(SRLError("split_tokens".to_string(), "trying to put ' after '='".to_string()));
				} else if x == '=' {
					return Err(SRLError("split_tokens".to_string(), "trying to put '=' after '='".to_string()));
				} else if SIMPLE_CELL_FILL_CHARS.contains(x) {
					return Err(SRLError("split_tokens".to_string(), format!("trying to put '{}' after '='", x)));
				} else if VAR_FILL_CHARS.contains(x) {
					return Err(SRLError("split_tokens".to_string(), format!("trying to put '{}' after '='", x)));
				} else {
					return Err(SRLError("split_tokens".to_string(), format!("forgot handling for '{}' in EQ state", x)));
				}
			}
			(x, &State::AFTER_CELL) => {
				if PARENS.contains(x) {
					tokens.push(x.to_string());
					state = State::NONE;
				} else if x == ' ' {
					state = State::NONE;
				} else if x == '\'' {
					return Err(SRLError("split_tokens".to_string(), format!("reading '{}' directly after cell end", x)));
				} else if x == '=' {
					return Err(SRLError("split_tokens".to_string(), "reading '=' directly after cell end".to_string()));
				} else if SIMPLE_CELL_FILL_CHARS.contains(x) {
					return Err(SRLError("split_tokens".to_string(), format!("reading '{}' directly after cell end", x)));
				} else if VAR_FILL_CHARS.contains(x) {
					return Err(SRLError("split_tokens".to_string(), format!("reading '{}' directly after cell end", x)));
				} else {
					return Err(SRLError("split_tokens".to_string(), format!("forgot handling for '{}' in AFTER_CELL state", x)))
				}
			}
		}
	}

	Ok(tokens)
}

#[test]
fn test_tokenize() {
	assert_eq!(tokenize("(wow good)".to_string()).unwrap(), vec!["(".to_string(), "wow".to_string(), "good".to_string(), ")".to_string()]);
	assert_eq!(tokenize("wow".to_string()).unwrap(), vec!["wow".to_string()]);
	assert_eq!(tokenize("{x}".to_string()).unwrap(), vec!["{".to_string(), "x".to_string(), "}".to_string()]);
}
