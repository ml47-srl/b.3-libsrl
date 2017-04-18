use std::fmt;
use error::SRLError;
use parse::SIMPLE_CELL_FILL_CHARS;
use parse::SIMPLE_CELL_CHARS;
use misc::*;

#[derive(PartialEq, Clone)]
pub struct SimpleString(String);

#[derive(PartialEq, Clone)]
pub enum Cell {
	Simple { string : SimpleString },
	Complex { cells: Vec<Cell> },
	Scope { id : u32, body : Box<Cell> }, // { ... }
	Var { id : u32 },
	Case { condition : Box<Cell>, conclusion : Box<Cell> }
}

#[derive(PartialEq)]
pub enum CellType { Simple, Complex, Scope, Var, Case }

impl SimpleString {
	pub fn create(string : String) -> Result<SimpleString, SRLError> {
		if string.len() == 0 {
			return Err(SRLError("SimpleString::create".to_string(), "string has length 0".to_string()));
		}
		if !contains_only(string.clone(), SIMPLE_CELL_CHARS.to_string()) {
			return Err(SRLError("SimpleString::create".to_string(), "invalid char".to_string()));
		}
		if string == "=" || contains_only(string.clone(), SIMPLE_CELL_FILL_CHARS.to_string()) {
			return Ok(SimpleString(string));
		}
		if string.starts_with('\'') && string.ends_with('\'') && string.matches('\'').count() == 2 {
			return Ok(SimpleString(string));
		}
		return Err(SRLError("SimpleString::create".to_string(), "weird invalid stuff".to_string()));
	}

	pub fn get_string(&self) -> String { self.0.clone() }
}

impl fmt::Debug for Cell {
	fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.to_string())
	}
}

impl fmt::Display for Cell {
	fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
		f.write_str(&self.to_string())
	}
}

impl Cell {
	pub fn by_string(string : &str) -> Result<Cell, SRLError> {
		use parse::*;
		use parse::assemble::*;
		use parse::tokenize::*;

		if string.contains('.') {
			return Err(SRLError("Cell::by_string".to_string(), "string contains '.'".to_string()));
		}

		match find_invalid_char(string) {
			Some(_) => return Err(SRLError("Cell::by_string".to_string(), "invalid char".to_string())),
			None => {}
		}
		let string : String = fix_whitespaces(string);
		let tokens = tokenize(string)?;
		if ! check_paren_correctness(tokens.clone()) {
			return Err(SRLError("Cell::by_string".to_string(), "parens incorrect".to_string()));
		}
		assemble(tokens)
	}

	pub fn is_constant(&self) -> bool {
		match &self {
			&&Cell::Simple { string : ref string_out } => return string_out.0.starts_with('\'') && string_out.0.ends_with('\''),
			_ => return false
		}
	}

	pub fn to_string(&self) -> String { // (equals a b); a
		return match &self {
			&&Cell::Complex {..} => {
				"(".to_string() + &self.to_unwrapped_string() + ")"
			},
			_ => self.to_unwrapped_string()
		}
	}

	pub fn to_unwrapped_string(&self) -> String { // equals a b | a
		match &self {
			&&Cell::Simple { string : ref string_out } => string_out.0.clone(),
			&&Cell::Complex { cells : ref cells_out } => {
				let mut string = String::new();
				string.push_str(&cells_out[0].to_string());
				for cell in cells_out.iter().skip(1) {
					string.push(' ');
					string.push_str(&cell.to_string());
				}
				return string;
			},
			&&Cell::Scope { id : ref id_out, body : ref body_out } => {
				let mut string = String::new();
				string.push('{');
				string.push_str(&id_out.to_string());
				string.push(' ');
				string.push_str(&body_out.to_string());
				string.push('}');
				string
			},
			&&Cell::Var { id : ref id_out } => {
				id_out.to_string()
			},
			&&Cell::Case { condition : ref condition_out, conclusion : ref conclusion_out } => {
				let mut string = String::new();
				string.push_str("[=> ");
				string.push_str(&condition_out.to_string());
				string.push(' ');
				string.push_str(&conclusion_out.to_string());
				string.push(']');
				string
			}
		}
	}

	pub fn to_rule_string(&self) -> String {
		self.to_unwrapped_string() + "."
	}

	pub fn count_subcells(&self) -> usize {
		return match &self {
			&&Cell::Simple {..} => 0,
			&&Cell::Complex { cells : ref cells_out } => cells_out.len(),
			&&Cell::Scope {..} => 1,
			&&Cell::Var {..} => 0,
			&&Cell::Case {..} => 2
		};
	}

	pub fn get_subcell(&self, index : usize) -> Cell {
		return match &self {
			&&Cell::Simple {..} => panic!("Cell::get_subcell(): Simple: no subcells"),
			&&Cell::Complex { cells : ref cells_out } => {
				if ! index_in_len(index, cells_out.len()) {
					panic!("Cell::get_subcell(): Complex: index out of range")
				}
				cells_out[index].clone()
			}
			&&Cell::Scope { body : ref body_out, .. } => {
				if index == 0 {
					body_out.as_ref().clone()
				} else {
					panic!("Cell::get_subcell(): Scope: index out of range")
				}
			},
			&&Cell::Var {..} => panic!("Cell::get_subcell(): Var: no subcells"),
			&&Cell::Case { condition : ref cond_out, conclusion : ref conc_out } => {
				if index == 0 {
					return cond_out.as_ref().clone();
				}
				if index == 1 {
					return conc_out.as_ref().clone();
				} 
				panic!("Cell::get_subcell(): Case: index out of range");
			}
		};
	}

	pub fn with_subcell(&self, cell : Cell, index : usize) -> Cell {
		if !index_in_len(index, self.count_subcells()) {
			panic!("Cell::with_subcell(): index not possible!");
		}
		match &self {
			&&Cell::Complex { cells : ref cells_out } => {
				let mut c : Vec<Cell> = cells_out.clone();
				c[index] = cell;
				return Cell::Complex { cells : c };
			},
			&&Cell::Scope { id : id_out, .. } => {
				return Cell::Scope { id : id_out, body : Box::new(cell) };
			},
			&&Cell::Case { condition : ref cond_out, conclusion : ref conc_out } => {
				if index == 0 {
					return Cell::Case { condition : Box::new(cell), conclusion : conc_out.clone() };
				} else if index == 1 {
					return Cell::Case { condition : cond_out.clone() , conclusion : Box::new(cell) };
				} else {
					panic!("Cell::with_subcell(): Case out of range");
				}
			},
			_ => panic!("Cell::with_subcell(): unacceptable!")
		}
	}

	pub fn recurse<T>(&self, mut t : T, lambda_expr : fn(&Cell, T) -> T) -> T {
		t = lambda_expr(&self, t);
		for index in 0..self.count_subcells() {
			t = self.get_subcell(index).recurse(t, lambda_expr);
		}
		t
	}

	pub fn get_type(&self) -> CellType {
		match self {
			&Cell::Scope{..} => CellType::Scope,
			&Cell::Simple{..} => CellType::Simple,
			&Cell::Complex{..} => CellType::Complex,
			&Cell::Case{..} => CellType::Case,
			&Cell::Var{..} => CellType::Var,
		}
	}

	// {0 = {1 (p 0 1) } {2 (p 0 2)}}.
	// the scopes 1 and 2 would `match`, but not be `equal`
	// == equals, when normalized (which you can't do because of vars out of scopes, as seen in example above)
	pub fn matches(&self, cell : &Cell) -> bool {
		self.matches_r(cell, vec![], vec![])
	}

	fn matches_r(&self, cell : &Cell, mut v1 : Vec<usize>, mut v2 : Vec<usize>) -> bool {
		if self.get_type() != cell.get_type() {
			return false;
		}
		match self {
			&Cell::Var { id : id_out } => {
				let id_out = &(id_out as usize);
				if let &Cell::Var { id : id_out2 } = cell {
					let id_out2  = &(id_out2 as usize);
					if v1.contains(id_out) != v2.contains(id_out2) {
						return false;
					}
					if v1.contains(id_out) {
						v1.iter().position(|x| x == id_out) == v2.iter().position(|x| x == id_out2);
					} else {
						return id_out == id_out2;
					}
					return true;
				} else { panic!("whoah!") }
			},
			&Cell::Scope { id : id_out, .. } => {
				if let &Cell::Scope { id : id_out2, .. } = cell {
					v1.push(id_out as usize);
					v2.push(id_out2 as usize);
				} else { panic!("whoah!") }
				for i in 0..self.count_subcells() {
					if !self.get_subcell(i).matches(&cell.get_subcell(i)) {
						return false;
					}
				}
				return true;
			},
			&Cell::Simple { string : ref string_out } => {
				if let &Cell::Simple { string : ref string2_out } = cell {
					return string_out == string2_out;
				} else { panic!("whoah!"); }
			},
			&Cell::Complex { cells : ref cells_out } => {
				if let &Cell::Complex { cells : ref cells2_out } = cell {
					for i in 0..cells_out.len() {
						if cells_out[i] != cells2_out[i] {
							return false;
						}
					}
					return true;
				} else { panic!("whoah!"); }
			},
			&Cell::Case { condition : ref cond_out, conclusion : ref conc_out } => {
				if let &Cell::Case { condition : ref cond2_out, conclusion : ref conc2_out } = cell {
					return cond_out.matches(cond2_out) && conc_out.matches(conc2_out);
				} else { panic!("whoah!"); }
			}
		}
	}

	pub fn replace_all(&self, pattern : Cell, replacement : Cell) -> Cell {
		let mut cell = self.clone();
		if cell == pattern { // XXX maybe cell.matches(pattern)
			return replacement;
		}
		for i in 0..cell.count_subcells() {
			cell = cell.with_subcell(cell.get_subcell(i).replace_all(pattern.clone(), replacement.clone()), i);
		}
		cell
	}
}

#[test]
fn test_get_subcell() {
	use gen::*;
	assert_eq!(complex(vec![true_cell(), false_cell()]).get_subcell(0), true_cell());
	assert_eq!(complex(vec![true_cell(), false_cell()]).get_subcell(1), false_cell());
}

#[test]
fn test_with_subcell() {
	use gen::*;
	assert_eq!(complex(vec![true_cell(), true_cell()]).with_subcell(false_cell(), 0), complex(vec![false_cell(), true_cell()]));
	assert_eq!(complex(vec![true_cell(), true_cell()]).with_subcell(false_cell(), 1), complex(vec![true_cell(), false_cell()]));
}

#[test]
fn test_to_string() {
	use gen::*;
	assert_eq!(&simple_by_str("a").to_string(), "a");
	assert_eq!(&complex(vec![simple_by_str("a"), simple_by_str("b")]).to_string(), "(a b)");
	assert_eq!(&scope(3, simple_by_str("b")).to_string(), "{3 b}");
}
