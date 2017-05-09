use cell::{Cell, CellType};
use error::*;
use misc::*;
use gen::*;

#[derive(Clone, PartialEq)]
pub struct CellID {
	rule_id : usize,
	indices : Vec<usize>
}

#[derive(Clone, PartialEq)]
pub struct CellPath {
	root_cell : Cell,
	indices : Vec<usize>
}

impl CellID {
	pub fn create(rule_index : usize, indices : Vec<usize>) -> CellID {
		CellID { rule_id : rule_index, indices : indices }
	}

	pub fn get_path(&self, rules : &Vec<Cell>) -> SRLResult<CellPath> {
		if index_in_len(self.rule_id, rules.len()) {
			CellPath::create(rules[self.rule_id].clone(), self.indices.clone())
		} else {
			err!("CellID::to_path(): index of rule_id out of range")
		}
	}

	pub fn get_rule_id(&self) -> usize { self.rule_id.clone() }
	pub fn get_indices(&self) -> Vec<usize> { self.indices.clone() }

	pub fn get_parent(&self) -> SRLResult<CellID> {
		let mut vec = self.indices.clone();
		return match vec.pop() {
			Some(_) => ok!(CellID::create(self.rule_id, vec)),
			None => err!("CellID::get_parent(): no parent")
		}
	}

	pub fn get_child(&self, index : usize) -> SRLResult<CellID> {
		let mut vec = self.indices.clone();
		vec.push(index);
		ok!(CellID::create(self.rule_id, vec))
	}

	pub fn get_left_sibling(&self) -> SRLResult<CellID> {
		let mut vec = self.indices.clone();
		let index = match vec.pop() {
			Some(x) => x,
			None => return err!("CellID::get_left_sibling(): no parent")
		};
		if index == 0 {
			return err!("CellID::get_left_sibling(): no left sibling");
		}

		vec.push(index - 1);
		return ok!(CellID::create(self.rule_id, vec));
	}

	pub fn get_right_sibling(&self) -> SRLResult<CellID> {
		let mut vec = self.indices.clone();
		let index = match vec.pop() {
			Some(x) => x,
			None => return err!("CellID::get_right_sibling(): no parent")
		};
		vec.push(index + 1);
		return ok!(CellID::create(self.rule_id, vec));
	}

	pub fn is_valid(&self, rules : &Vec<Cell>) -> bool {
		if self.get_path(rules).is_ok() {
			return true;
		}
		return false;
	}
}

impl CellPath {
	pub fn create(root_cell : Cell, indices : Vec<usize>) -> SRLResult<CellPath> {
		// error test
		let mut cell = root_cell.clone();
		for index in indices.clone() {
			if index_in_len(index, cell.count_subcells()) {
				cell = cell.get_subcell(index);
			} else {
				return err!("CellPath::create(): index is unacceptable!")
			}
		}
		ok!(CellPath { root_cell : root_cell, indices : indices })
	}

	pub fn get_cell(&self) -> Cell {
		let mut cell = self.root_cell.clone();
		for index in self.indices.clone() {
			cell = cell.get_subcell(index);
		}
		cell
	}

	pub fn is_complete_bool(&self) -> bool {
		let my_cell = self.get_cell();
		if my_cell.get_equals_cell_arguments().is_ok() { return true; } // it's an equals cell
		if let Cell::Scope{..} = my_cell { return true; } // it's a scope
		if my_cell == true_cell() { return true; }
		if my_cell == false_cell() { return true; }
		false
	}

	pub fn is_bool(&self) -> bool {
		if self.is_complete_bool() {
			return true;
		}

		// positionals
		let parent = match self.get_parent() {
			SRLResult::Ok(x) => x,
			SRLResult::Err(_) => return true // no parent => rule
		};
		match parent.get_cell() {
			Cell::Scope{..} | Cell::Case{..} => return true,
			_ => {}
		}
		false
	}

	pub fn get_parent(&self) -> SRLResult<CellPath> {
		let mut vec = self.indices.clone();
		return match vec.pop() {
			Some(_) => CellPath::create(self.root_cell.clone(), vec),
			None => err!("CellPath::get_parent(): no parent")
		}
	}

	pub fn get_child(&self, index : usize) -> SRLResult<CellPath> {
		let mut vec = self.indices.clone();
		vec.push(index);
		CellPath::create(self.root_cell.clone(), vec)
	}

	pub fn get_left_sibling(&self) -> SRLResult<CellPath> {
		let mut vec = self.indices.clone();
		let index = match vec.pop() {
			Some(x) => x,
			None => return err!("CellPath::get_right_sibling(): no parent")
		};
		if index == 0 {
			return err!("CellPath::get_left_sibling(): no left sibling")
		}

		let parent = x!(self.get_parent());
		return parent.get_child(index - 1);
	}

	pub fn get_right_sibling(&self) -> SRLResult<CellPath> {
		let mut vec = self.indices.clone();
		let index = match vec.pop() {
			Some(x) => x,
			None => return err!("CellPath::get_right_sibling(): no parent")
		};
		let parent = x!(self.get_parent());
		return parent.get_child(index + 1);
	}

	pub fn replace_by(&self, mut cell : Cell) -> Cell {
		let mut indices = self.indices.clone();
		let mut last_index;

		while indices.len() > 0 {
			last_index = match indices.pop() {
				Some(x) => x,
				None => panic!("CellPath.replace_by: failure 1 - should not happen")
			};
			let cell_path = match CellPath::create(self.root_cell.clone(), indices.clone()) {
				SRLResult::Ok(x) => x,
				SRLResult::Err(srl_error) => panic!("CellPath::replace_by is invalid: {:?}", srl_error)
			};
			let tmp = cell_path.get_cell();
			cell = tmp.with_subcell(cell, last_index);
		}
		cell
	}

	pub fn get_type(&self) -> CellType {
		self.get_cell().get_type()
	}

	pub fn get_root_cell(&self) -> Cell { self.root_cell.clone() }
	pub fn get_indices(&self) -> Vec<usize> { self.indices.clone() }
}

#[test]
fn test_cell_id_and_cell_path() {
	let mut rules : Vec<Cell> = Vec::new();
	rules.push(simple_by_str("truth"));
	rules.push(complex(vec![simple_by_str("truth"), simple_by_str("wot")]));

	assert_eq!(
		CellPath { root_cell : rules[0].clone(), indices : Vec::new() }.get_cell(),
		simple_by_str("truth")
	);

	assert_eq!(
		CellPath { root_cell : rules[1].clone(), indices : vec![0] }.get_cell(),
		simple_by_str("truth")
	);

	assert_eq!(
		CellPath { root_cell : rules[1].clone(), indices : vec![1] }.get_cell(),
		simple_by_str("wot")
	);
}

#[test]
fn test_cell_path_replace_by() {
	let mut rules : Vec<Cell> = Vec::new();
	rules.push(simple_by_str("truth"));
	rules.push(complex(vec![simple_by_str("truth"), simple_by_str("wot")]));

	assert_eq!(
		CellPath { root_cell : rules[1].clone(), indices : vec![1] }.replace_by(simple_by_str("wow")),
		complex(vec![simple_by_str("truth"), simple_by_str("wow")])
	);
}
