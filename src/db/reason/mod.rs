mod wrapper;

use super::Database;
use cell::Cell;
use error::SRLError;
use gen::*;
use navi::CellID;
use navi::CellPath;

impl Database {
	fn add_rule(&mut self, rule : Cell) -> Result<Cell, SRLError> {
		let norm = rule.get_normalized()?;
		self.rules.push(norm.clone());
		Ok(norm)
	}

	// src_id = "The cell that has to be replaced" | `{0 (<p> 0)}.`
	// evidence_id = "the equals cell"		  | `{0 <(= p q)>}`
	pub fn equals_law(&mut self, src_id : CellID, evidence_id : CellID) -> Result<Cell, SRLError> {
		let src_path = src_id.get_path(&self.rules)?;
		let evidence_path = evidence_id.get_path(&self.rules)?;

		let wrapper = match evidence_path.get_wrapper() {
			Some(x) => x,
			None => return Err(SRLError("equals_law".to_string(), "evidence_id is not in wrapper".to_string()))
		};
		if !wrapper.is_nexq() {
			return Err(SRLError("equals_law".to_string(), "wrapper is no nexq-wrapper".to_string()));
		}
		let evi_cell = evidence_path.get_cell();
		let (a, b) = evi_cell.get_equals_cell_arguments()?;

		if !wrapper.is_around(&src_path) {
			return Err(SRLError("equals_law".to_string(), "src_id and evidence_id are not in the same wrapper".to_string()));
		}

		let src_cell = src_path.get_cell();

		let new : Cell;
		if a.matches(&src_cell) {
			new = b;
		} else if b.matches(&src_cell) {
			new = a;
		} else {
			return Err(SRLError("equals_law".to_string(), "replace cell does not occur in evidence".to_string()));
		}

		let rule = src_path.replace_by(new);
		self.add_rule(rule)
	}

	// src_id = "The cell that has to be replaced" | `{0 [=> (= p q) (<p> 0)]}.`
	// evidence_id = "the equals cell"		  | `{0 [=> <(= p q)> (p 0)]}`
	pub fn equals_law_impl(&mut self, src_id : CellID, evidence_id : CellID) -> Result<Cell, SRLError> {
		let src_path = src_id.get_path(&self.rules)?;
		let evidence_path = evidence_id.get_path(&self.rules)?;

		// check whether evidence_id is the condition of a case-cell
		if evidence_id.get_indices().last() != Some(&(0 as usize)) {
			return Err(SRLError("equals_law_impl".to_string(), "evidence_id can't be condition of case-cell".to_string()));
		}
		if let Cell::Case{..} = evidence_path.get_parent()?.get_cell() {} else {
			return Err(SRLError("equals_law_impl".to_string(), "evidence_id can't be condition of case-cell (2)".to_string()));
		}

		let rule_id = src_id.get_rule_id();
		if rule_id != evidence_id.get_rule_id() {
			return Err(SRLError("equals_law_impl".to_string(), "src_id and evidence_id are not in the same rule".to_string()));
		}
		let wrapper = match evidence_path.get_parent()?.get_child(1)?.get_wrapper() {
			Some(x) => x,
			None => return Err(SRLError("equals_law_impl".to_string(), "no wrapper!".to_string()))
		};
		if !wrapper.is_around(&src_path) {
			return Err(SRLError("equals_law_impl".to_string(), "evi-wrapper is not around src_id".to_string()));
		}

		let evi_cell = evidence_path.get_cell();
		let (a, b) = evi_cell.get_equals_cell_arguments()?;
		let src_cell = src_path.get_cell();

		let new : Cell;
		if a.matches(&src_cell) {
			new = b;
		} else if b.matches(&src_cell) {
			new = a;
		} else {
			return Err(SRLError("equals_law".to_string(), "replace cell does not occur in evidence".to_string()));
		}

		let rule = src_path.replace_by(new);
		self.add_rule(rule)
	}

	// id: `<(= 'ok' 'wow')>`
	pub fn inequal_constants(&mut self, id : CellID) -> Result<Cell, SRLError> {
		let path = id.get_path(&self.rules)?;

		let cell = path.get_cell();
		let (x, y) = cell.get_equals_cell_arguments()?;
		if !x.is_constant() {
			return Err(SRLError("inequals_constants".to_string(), "first arg not constant".to_string()));
		}
		if !y.is_constant() {
			return Err(SRLError("inequals_constants".to_string(), "second arg is not constant".to_string()));
		}
		if x == y {
			return Err(SRLError("inequals_constants".to_string(), "both args equal".to_string()));
		}
		let rule = path.replace_by(false_cell());
		self.add_rule(rule)
	}

	// cell_id: <ok> => (= 'true' <ok>)
	pub fn add_eqt(&mut self, cell_id : CellID) -> Result<Cell, SRLError> {
		let cell_path = cell_id.get_path(&self.rules)?;

		if !cell_path.is_bool() {
			return Err(SRLError("add_eqt".to_string(), "cell is not bool".to_string()));
		}
		let cell = cell_path.get_cell();
		let rule = cell_path.replace_by(equals_cell(true_cell(), cell));
		self.add_rule(rule)
	}

	// cell_id: (= 'true' <ok>) => <ok>
	pub fn rm_eqt(&mut self, cell_id : CellID) -> Result<Cell, SRLError> {
		let cell_path = cell_id.get_path(&self.rules)?;

		let cell = cell_path.get_cell();

		if cell_path.get_indices().is_empty() {
			return Err(SRLError("rm_eqt".to_string(), "cell has no parents".to_string()));
		}
		let parent_path = cell_path.get_parent()?;
		let parent_cell = parent_path.get_cell();
		let (a, b) : (Cell, Cell);
		if let Ok((x, y)) = parent_cell.get_equals_cell_arguments() {
			a = x; b = y;
		} else {
			return Err(SRLError("rm_eqt".to_string(), "not contained in equals cell".to_string()));
		}

		if a != true_cell() {
			return Err(SRLError("rm_eqt".to_string(), "first cell in equals is not 'true'".to_string()));
		}

		if b != cell {
			return Err(SRLError("rm_eqt".to_string(), "second cell in equals is not cell_id".to_string()));
		}

		let rule = parent_path.replace_by(cell);
		let tmp_cell_path = CellPath::create(rule.clone(), parent_path.get_indices())?;
		if !tmp_cell_path.is_bool() {
			return Err(SRLError("rm_eqt".to_string(), "result is no bool-cell".to_string()));
		}

		self.add_rule(rule)
	}

	pub fn scope_insertion(&mut self, scope_id : CellID, cell : Cell) -> Result<Cell, SRLError> {
		let scope_path = scope_id.get_path(&self.rules)?;

		let (id, body) : (u32, Cell) = match scope_path.get_cell() {
			Cell::Scope { id : x, body : y } => (x, *y),
			_ => return Err(SRLError("scope_insertion".to_string(), "scope_id does not represent scope".to_string()))
		};
		let child_path = scope_path.get_child(0)?;
		if !child_path.is_complete_bool() {
			return Err(SRLError("scope_insertion".to_string(), "body is no complete bool cell".to_string()));
		}
		let wrapper = match scope_path.get_wrapper() {
			Some(x) => x,
			None => return Err(SRLError("scope_insertion".to_string(), "no wrapper".to_string()))
		};
		if !wrapper.is_positive() {
			return Err(SRLError("scope_insertion".to_string(), "wrapper is not positive".to_string()));
		}

		let mut highest_id : i32 = scope_path.get_root_cell().get_next_id() as i32 - 1;
		let norm = cell.get_normalized()?;
		let id_amount : i32 = norm.get_next_id() as i32;

		let mut path = CellPath::create(body.clone(), vec![])?;

		loop {
			loop {
				match path.get_child(0) {
					Ok(x) => {
						path = x;
					},
					Err(_) => { break; }
				}
			}
			if let Cell::Var { id : id_out } = path.get_cell() {
				if id_out == id {
					let normalized = cell.get_normalized_from((highest_id + 1) as u32)?;
					let replaced = path.replace_by(normalized);
					path = CellPath::create(replaced, path.get_indices())?;
					highest_id += id_amount;
				}
			}
			loop {
				if path.get_indices().is_empty() {
					let rule = scope_path.replace_by(path.get_root_cell());
					return self.add_rule(rule);
				} else {
					match path.get_right_sibling() {
						Ok(x) => {
							path = x;
							break;
						},
						Err(_) => { path = path.get_parent().unwrap(); }
					}
				}
			}
		}
	}

	// = 'false' (= 'true' x).
	//                    <x>   => indices = vec![vec![2]] // indices relative to the scope_id
	//          <(= 'true' x)>  => scope_id
	pub fn scope_creation(&mut self, scope_id : CellID, indices : Vec<Vec<usize>>) -> Result<Cell, SRLError> {
		let mut scope_path = scope_id.get_path(&self.rules)?;

		if !scope_path.is_complete_bool() {
			return Err(SRLError("scope_creation".to_string(), "scope_id does not contain a complete bool-cell".to_string()));
		}

		let wrapper = match scope_path.get_wrapper() {
			Some(x) => x,
			None => return Err(SRLError("scope_creation".to_string(), "no wrapper".to_string()))
		};
		if wrapper.is_positive() {
			return Err(SRLError("scope_creation".to_string(), "wrapper is positive".to_string()));
		}

		let cell = scope_path.get_cell();
		for i in 0..indices.len()-1 {
			let cell1 = CellPath::create(cell.clone(), indices[i].clone())?.get_cell();
			let cell2 = CellPath::create(cell.clone(), indices[i+1].clone())?.get_cell(); // optimizable by not calculating everything twice
			if !cell1.matches(&cell2) {
				return Err(SRLError("scope_creation".to_string(), "indices do not represent the same cells".to_string()));
			}
		}

		let new_id : u32 = scope_path.get_root_cell().get_next_id() as u32;
		let replaced = scope_path.replace_by(scope(new_id, cell));
		scope_path = CellPath::create(replaced, scope_path.get_indices())?.get_child(0)?;

		for mut index in indices {
			let mut correct_index = scope_path.get_indices();
			correct_index.append(&mut index);

			let tmp_path = CellPath::create(scope_path.get_root_cell(), correct_index)?;
			let new_cell = tmp_path.replace_by(var(new_id));
			scope_path = CellPath::create(new_cell, scope_path.get_indices())?;
		}

		self.add_rule(scope_path.get_root_cell())
	}

	pub fn implications_derivation(&mut self, case_id : CellID, case_negation_id : CellID) -> Result<Cell, SRLError> {
		let case_path = case_id.get_path(&self.rules)?;
		let case_negation_path = case_negation_id.get_path(&self.rules)?;

		let case_cell = case_path.get_cell();
		let case_negation_cell = case_negation_path.get_cell();

		let (case_condition, case_conclusion) = match case_cell {
			Cell::Case { condition : x, conclusion : y} => (*x, *y),
			_ => return Err(SRLError("implications_derivation".to_string(), "case_id does not represent case-cell".to_string()))
		};
		let (case_negation_condition, case_negation_conclusion) = match case_negation_cell {
			Cell::Case { condition : x, conclusion : y} => (*x, *y),
			_ => return Err(SRLError("implications_derivation".to_string(), "case_negation_id does not represent case-cell".to_string()))
		};

		if case_conclusion != case_negation_conclusion {
			return Err(SRLError("implications_derivation".to_string(), "conclusions differ".to_string()));
		}

		if equals_cell(false_cell(), case_condition) != case_negation_condition {
			return Err(SRLError("implications_derivation".to_string(), "conditions are not correct".to_string()));
		}

		let case_wrapper = match case_path.get_wrapper() {
			Some(x) => x,
			None => return Err(SRLError("implications_derivation".to_string(), "no wrapper (1)".to_string()))
		};
		let case_negation_wrapper = match case_negation_path.get_wrapper() {
			Some(x) => x,
			None => return Err(SRLError("implications_derivation".to_string(), "no wrapper (2)".to_string()))
		};
		if case_wrapper != case_negation_wrapper {
			return Err(SRLError("implications_derivation".to_string(), "different wrappers".to_string()));
		}

		if !case_wrapper.is_nexq() {
			return Err(SRLError("implications_derivation".to_string(), "wrapper contains existance quantor".to_string()));
		}

		if !case_wrapper.is_positive() {
			return Err(SRLError("implications_derivation".to_string(), "wrapper is negative".to_string()));
		}
		self.add_rule(case_conclusion)
	}

	pub fn scope_exchange(&mut self, outer_scope_id : CellID) -> Result<Cell, SRLError> {
		let outer_scope_path = outer_scope_id.get_path(&self.rules)?;

		let inner_scope_path = outer_scope_path.get_child(0)?;
		let outer_id = match outer_scope_path.get_cell() {
			Cell::Scope { id : x, ..} => x,
			_ => return Err(SRLError("scope_exchange".to_string(), "outer cell is no scope".to_string()))
		};
		let (inner_id, body) = match inner_scope_path.get_cell() {
			Cell::Scope { id : x, body : y } => (x, *y),
			_ => return Err(SRLError("scope_exchange".to_string(), "inner cell is no scope".to_string()))
		};

		let rule = outer_scope_path.replace_by(scope(inner_id, scope(outer_id, body)));
		self.add_rule(rule)
	}

	pub fn case_creation(&mut self, cell_id : CellID, arg_cell : Cell) -> Result<Cell, SRLError> {
		let path = cell_id.get_path(&self.rules)?;
		let wrapper = match path.get_wrapper() {
			Some(x) => x,
			None => return Err(SRLError("case_creation".to_string(), "no wrapper".to_string()))
		};
		if !wrapper.is_positive() {
			return Err(SRLError("case_creation".to_string(), "wrapper is not positive".to_string()));
		}
		let cell = path.get_cell();
		let rule = path.replace_by(case(arg_cell, cell));
		self.add_rule(rule)
	}

	// <(= 'false' {0 (= 'false' (p 0 1))})>
	pub fn declaration(&mut self, cell_id : CellID, string : &str) -> Result<Cell, SRLError> {
		// occurence checks
		if self.contains_cellname(string) {
			return Err(SRLError("declaration".to_string(), "string does already occur".to_string()));
		}

		// wrapper checks
		let cell_path = cell_id.get_path(&self.rules)?;
		let wrapper = match cell_path.get_wrapper() {
			Some(x) => x,
			None => return Err(SRLError("declaration".to_string(), "no wrapper".to_string()))
		};
		if !wrapper.is_positive() {
			return Err(SRLError("declaration".to_string(), "wrapper is negative".to_string()));
		}
		if !wrapper.is_nallq() {
			return Err(SRLError("declaration".to_string(), "wrapper contains all quantor".to_string()));
		}

		// check (= 'false' {0 (= 'false' * )}) pattern
		let (x, y) = cell_path.get_cell().get_equals_cell_arguments()?;
		if x != false_cell() {
			return Err(SRLError("declaration".to_string(), "first arg of equals cell is not 'false'".to_string()));
		}
		let (id, body) = match y {
			Cell::Scope { id : x, body : y } => (x, y),
			_ => return Err(SRLError("declaration".to_string(), "second arg is no scope".to_string()))
		};
		let (a, b) = body.get_equals_cell_arguments()?;
		if a != false_cell() {
			return Err(SRLError("declaration".to_string(), "scope does not contain (= 'false' *)".to_string()));
		}

		let new = b.replace_all(var(id), try_simple(string.to_string())?);
		let rule = cell_path.replace_by(new);
		self.add_rule(rule)
	}
}
