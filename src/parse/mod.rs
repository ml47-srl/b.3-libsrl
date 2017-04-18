pub mod assemble;
pub mod tokenize;

pub static VALID_CHARS : &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_=0123456789'(){} \n\t.";
pub static SIMPLE_CELL_FILL_CHARS : &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_";
pub static SIMPLE_CELL_CHARS : &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_''=";
pub static VAR_FILL_CHARS : &'static str = "0123456789";
pub static LPARENS : &'static str = "{(";
pub static RPARENS : &'static str = "})";
pub static PARENS : &'static str = "{(})";

pub fn find_invalid_char(string : &str) -> Option<i32> {
	let mut x = 0;
	for i in string.chars() {
		if ! VALID_CHARS.contains(i) {
			return Some(x);
		}
		x += 1;
	}
	None
}

#[test]
fn test_find_invalid_char() {
	assert_eq!(find_invalid_char("wowäsdf"), Some(3));
}

pub fn fix_whitespaces(string : &str) -> String {
	// '\t', '\n'  => ' '
	let mut string : String = string.chars().map(|c| {
		match c {
			'\t' | '\n' | ' ' => ' ',
			x => x
		}
	}).collect();
	// "  " => " "
	loop {
		match string.find("  ") {
			Some(x) => { string.remove(x); },
			None => break
		}
	}
	// "equals a b. " => "equals a b."
	match string.chars().rev().nth(0) {
		Some(x) => {
			if x == ' ' {
				string.pop();
			}
		}
		None => {}
	}
	// " equals a b." => "equals a b."
	match string.chars().nth(0) {
		Some(x) => {
			if x == ' ' {
				string = string.chars().skip(1).collect();
			}
		}
		None => {}
	}

	// "( " => "("
	for paren in LPARENS.chars() {
		loop {
			let mut pattern : String = String::new();
			pattern.push(paren);
			pattern.push(' ');

			match string.find(&pattern) {
				Some(x) => { string.remove(x+1); }
				None => break
			}
		}
	}
	// " )" => ")"
	for paren in RPARENS.chars() {
		loop {
			let mut pattern : String = " ".to_string();
			pattern.push(paren);
			match string.find(&pattern) {
				Some(x) => { string.remove(x); }
				None => break
			}
		}
	}
	string
}

#[test]
fn test_fix_whitespaces() {
	assert_eq!(fix_whitespaces("wow.
	 x 
	 "), "wow. x".to_string());
	assert_eq!(fix_whitespaces("abc   "), "abc");
	assert_eq!(fix_whitespaces("   abc"), "abc");
	assert_eq!(fix_whitespaces(" \n  ( abc  )"), "(abc)");
	assert_eq!(fix_whitespaces(" abc def ()  \t ( abc  )"), "abc def () (abc)");
}

// splits a database string, where fix_whitespaces was already called into rule-strings
pub fn split_rules(string : String) -> Vec<String> {
	let mut vec : Vec<String> = Vec::new();
	let mut string : String = string.to_string();

	if string.is_empty() {
		return Vec::new();
	}

	if ! string.ends_with(".") {
		panic!("string does not end with '.'");
	}

	loop {
		match string.find(".") {
			Some(x) => {
				if x == 0 {
					panic!("invalid '.'-expression at beginning");
				}
				let string_clone = string.clone();
				let (new_rule_string, tmp_string) = string_clone.split_at(x+1);
				string = tmp_string.to_string();
				let mut new_rule_string : String = new_rule_string.to_string();
				new_rule_string.pop();
				if ! new_rule_string.is_empty() {
					vec.push(new_rule_string);
				}
			}
			None => {
				if ! string.is_empty() {
					vec.push(string);
				}
				break;
			}
		}
	}
	vec
}

#[test]
fn test_split_rules() {
	assert_eq!(split_rules("wow.nice.good.".to_string()), vec!["wow".to_string(), "nice".to_string(), "good".to_string()]);
	assert!(split_rules("".to_string()).is_empty());
}

#[test]
#[should_panic]
fn test_split_rules2() {
	split_rules(".nice.good.".to_string());
}

#[test]
#[should_panic]
fn test_split_rules3() {
	split_rules("nice..good.".to_string());
}

#[test]
#[should_panic]
fn test_split_rules4() {
	split_rules("good".to_string());
}

pub fn is_simple_token(token : &str) -> bool {
	match token.chars().next() {
		Some(chr) => return SIMPLE_CELL_FILL_CHARS.contains(chr) || chr == '=' || chr == '\'',
		None => false
	}
}

pub fn is_var_token(token: &str) -> bool {
	match token.chars().next() {
		Some(chr) => return VAR_FILL_CHARS.contains(chr),
		None => false
	}
}
