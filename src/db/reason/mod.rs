mod wrapper;

use super::Database;
use cell::Cell;
use error::*;
use gen::*;
use navi::CellID;
use navi::CellPath;

impl Database {
	fn add_rule(&mut self, rule : Cell) -> SRLResult<Cell> {
		let norm = x!(rule.get_normalized());
		self.rules.push(norm.clone());
		ok!(norm)
	}

	// src_id = "The cell that has to be replaced" | `{0 (<p> 0)}.`
	// evidence_id = "the equals cell"		  | `{0 <(= p q)>}`
	pub fn equals_law(&mut self, src_id : CellID, evidence_id : CellID) -> SRLResult<Cell> {
		let src_path = x!(src_id.get_path(&self.rules));
		let evidence_path = x!(evidence_id.get_path(&self.rules));

		let wrapper = match evidence_path.get_wrapper() {
			Some(x) => x,
			None => return err!("equals_law(): evidence_id is not in wrapper")
		};
		if !wrapper.is_nexq() {
			return err!("equals_law(): wrapper is no nexq-wrapper")
		}
		let evi_cell = evidence_path.get_cell();
		let (a, b) = x!(evi_cell.get_equals_cell_arguments());

		if !wrapper.is_around(&src_path) {
			return err!("equals_law(): src_id and evidence_id are not in the same wrapper");
		}

		let src_cell = src_path.get_cell();

		let new : Cell;
		if a.matches(&src_cell) {
			new = b;
		} else if b.matches(&src_cell) {
			new = a;
		} else {
			return err!("equals_law(): replace cell does not occur in evidence");
		}

		let rule = src_path.replace_by(new);
		self.add_rule(rule)
	}

	// src_id = "The cell that has to be replaced" | `{0 [=> (= p q) (<p> 0)]}.`
	// evidence_id = "the equals cell"		  | `{0 [=> <(= p q)> (p 0)]}`
	pub fn equals_law_impl(&mut self, src_id : CellID, evidence_id : CellID) -> SRLResult<Cell> {
		let src_path = x!(src_id.get_path(&self.rules));
		let evidence_path = x!(evidence_id.get_path(&self.rules));

		// check whether evidence_id is the condition of a case-cell
		if evidence_id.get_indices().last() != Some(&(0 as usize)) {
			return err!("equals_law_impl(): evidence_id can't be condition of case-cell");
		}
		if let Cell::Case{..} = x!(evidence_path.get_parent()).get_cell() {} else {
			return err!("equals_law_impl(): evidence_id can't be condition of case-cell (2)")
		}

		let rule_id = src_id.get_rule_id();
		if rule_id != evidence_id.get_rule_id() {
			return err!("equals_law_impl(): src_id and evidence_id are not in the same rule")
		}
		let wrapper = match x!(x!(evidence_path.get_parent()).get_child(1)).get_wrapper() {
			Some(x) => x,
			None => return err!("equals_law_impl(): no wrapper!")
		};
		if !wrapper.is_around(&src_path) {
			return err!("equals_law_impl(): evi-wrapper is not around src_id")
		}

		let evi_cell = evidence_path.get_cell();
		let (a, b) = x!(evi_cell.get_equals_cell_arguments());
		let src_cell = src_path.get_cell();

		let new : Cell;
		if a.matches(&src_cell) {
			new = b;
		} else if b.matches(&src_cell) {
			new = a;
		} else {
			return err!("equals_law(): replace cell does not occur in evidence");
		}

		let rule = src_path.replace_by(new);
		self.add_rule(rule)
	}

	// id: `<(= 'ok' 'wow')>`
	pub fn inequal_constants(&mut self, id : CellID) -> SRLResult<Cell> {
		let path = x!(id.get_path(&self.rules));

		let cell = path.get_cell();
		let (x, y) = x!(cell.get_equals_cell_arguments());
		if !x.is_constant() {
			return err!("inequals_constants(): first arg not constant");
		}
		if !y.is_constant() {
			return err!("inequals_constants(): second arg is not constant");
		}
		if x == y {
			return err!("inequals_constants(): both args equal");
		}
		let rule = path.replace_by(false_cell());
		self.add_rule(rule)
	}

	// cell_id: <ok> => (= 'true' <ok>)
	pub fn add_eqt(&mut self, cell_id : CellID) -> SRLResult<Cell> {
		let cell_path = x!(cell_id.get_path(&self.rules));

		if !cell_path.is_bool() {
			return err!("add_eqt(): cell is not bool");
		}
		let cell = cell_path.get_cell();
		let rule = cell_path.replace_by(equals_cell(true_cell(), cell));
		self.add_rule(rule)
	}

	// cell_id: (= 'true' <ok>) => <ok>
	pub fn rm_eqt(&mut self, cell_id : CellID) -> SRLResult<Cell> {
		let cell_path = x!(cell_id.get_path(&self.rules));

		let cell = cell_path.get_cell();

		if cell_path.get_indices().is_empty() {
			return err!("rm_eqt(): cell has no parents");
		}
		let parent_path = x!(cell_path.get_parent());
		let parent_cell = parent_path.get_cell();
		let (a, b) : (Cell, Cell);
		if let SRLResult::Ok((x, y)) = parent_cell.get_equals_cell_arguments() {
			a = x; b = y;
		} else {
			return err!("rm_eqt(): not contained in equals cell");
		}

		if a != true_cell() {
			return err!("rm_eqt(): first cell in equals is not 'true'");
		}

		if b != cell {
			return err!("rm_eqt(): second cell in equals is not cell_id");
		}

		let rule = parent_path.replace_by(cell);
		let tmp_cell_path = x!(CellPath::create(rule.clone(), parent_path.get_indices()));
		if !tmp_cell_path.is_bool() {
			return err!("rm_eqt(): result is no bool-cell");
		}

		self.add_rule(rule)
	}

	pub fn scope_insertion(&mut self, scope_id : CellID, cell : Cell) -> SRLResult<Cell> {
		let scope_path = x!(scope_id.get_path(&self.rules));

		let (id, body) : (u32, Cell) = match scope_path.get_cell() {
			Cell::Scope { id : x, body : y } => (x, *y),
			_ => return err!("scope_insertion(): scope_id does not represent scope")
		};
		let child_path = x!(scope_path.get_child(0));
		if !child_path.is_complete_bool() {
			return err!("scope_insertion(): body is no complete bool cell");
		}
		let wrapper = match scope_path.get_wrapper() {
			Some(x) => x,
			None => return err!("scope_insertion(): no wrapper")
		};
		if !wrapper.is_positive() {
			return err!("scope_insertion(): wrapper is not positive");
		}

		let mut highest_id : i32 = scope_path.get_root_cell().get_next_id() as i32 - 1;
		let norm = x!(cell.get_normalized());
		let id_amount : i32 = norm.get_next_id() as i32;

		let mut path = x!(CellPath::create(body.clone(), vec![]));

		loop {
			loop {
				match path.get_child(0) {
					SRLResult::Ok(x) => {
						path = x;
					},
					SRLResult::Err(_) => { break; }
				}
			}
			if let Cell::Var { id : id_out } = path.get_cell() {
				if id_out == id {
					let normalized = x!(cell.get_normalized_from((highest_id + 1) as u32));
					let replaced = path.replace_by(normalized);
					path = x!(CellPath::create(replaced, path.get_indices()));
					highest_id += id_amount;
				}
			}
			loop {
				if path.get_indices().is_empty() {
					let rule = scope_path.replace_by(path.get_root_cell());
					return self.add_rule(rule);
				} else {
					match path.get_right_sibling() {
						SRLResult::Ok(x) => {
							path = x;
							break;
						},
						SRLResult::Err(_) => { path = path.get_parent().unwrap(); }
					}
				}
			}
		}
	}

	// = 'false' (= 'true' x).
	//                    <x>   => indices = vec![vec![2]] // indices relative to the scope_id
	//          <(= 'true' x)>  => scope_id
	pub fn scope_creation(&mut self, scope_id : CellID, indices : Vec<Vec<usize>>) -> SRLResult<Cell> {
		let mut scope_path = x!(scope_id.get_path(&self.rules));

		if !scope_path.is_complete_bool() {
			return err!("scope_creation(): scope_id does not contain a complete bool-cell");
		}

		let wrapper = match scope_path.get_wrapper() {
			Some(x) => x,
			None => return err!("scope_creation(): no wrapper")
		};
		if wrapper.is_positive() {
			return err!("scope_creation(): wrapper is positive");
		}

		let cell = scope_path.get_cell();
		for i in 0..indices.len()-1 {
			let cell1 = x!(CellPath::create(cell.clone(), indices[i].clone())).get_cell();
			let cell2 = x!(CellPath::create(cell.clone(), indices[i+1].clone())).get_cell(); // optimizable by not calculating everything twice
			if !cell1.matches(&cell2) {
				return err!("scope_creation(): indices do not represent the same cells");
			}
		}

		let new_id : u32 = scope_path.get_root_cell().get_next_id() as u32;
		let replaced = scope_path.replace_by(scope(new_id, cell));
		scope_path = x!(x!(CellPath::create(replaced, scope_path.get_indices())).get_child(0));

		for mut index in indices {
			let mut correct_index = scope_path.get_indices();
			correct_index.append(&mut index);

			let tmp_path = x!(CellPath::create(scope_path.get_root_cell(), correct_index));
			let new_cell = tmp_path.replace_by(var(new_id));
			scope_path = x!(CellPath::create(new_cell, scope_path.get_indices()));
		}

		self.add_rule(scope_path.get_root_cell())
	}

	pub fn implications_derivation(&mut self, case_id : CellID, case_negation_id : CellID) -> SRLResult<Cell> {
		let case_path = x!(case_id.get_path(&self.rules));
		let case_negation_path = x!(case_negation_id.get_path(&self.rules));

		let case_cell = case_path.get_cell();
		let case_negation_cell = case_negation_path.get_cell();

		let (case_condition, case_conclusion) = match case_cell {
			Cell::Case { condition : x, conclusion : y} => (*x, *y),
			_ => return err!("implications_derivation(): case_id does not represent case-cell")
		};
		let (case_negation_condition, case_negation_conclusion) = match case_negation_cell {
			Cell::Case { condition : x, conclusion : y} => (*x, *y),
			_ => return err!("implications_derivation(): case_negation_id does not represent case-cell")
		};

		if case_conclusion != case_negation_conclusion {
			return err!("implications_derivation(): conclusions differ");
		}

		if equals_cell(false_cell(), case_condition) != case_negation_condition {
			return err!("implications_derivation(): conditions are not correct");
		}

		let case_wrapper = match case_path.get_wrapper() {
			Some(x) => x,
			None => return err!("implications_derivation(): no wrapper (1)")
		};
		let case_negation_wrapper = match case_negation_path.get_wrapper() {
			Some(x) => x,
			None => return err!("implications_derivation(): no wrapper (2)")
		};
		if case_wrapper != case_negation_wrapper {
			return err!("implications_derivation(): different wrappers");
		}

		if !case_wrapper.is_nexq() {
			return err!("implications_derivation(): wrapper contains existance quantor");
		}

		if !case_wrapper.is_positive() {
			return err!("implications_derivation(): wrapper is negative");
		}
		self.add_rule(case_conclusion)
	}

	pub fn scope_exchange(&mut self, outer_scope_id : CellID) -> SRLResult<Cell> {
		let outer_scope_path = x!(outer_scope_id.get_path(&self.rules));

		let inner_scope_path = x!(outer_scope_path.get_child(0));
		let outer_id = match outer_scope_path.get_cell() {
			Cell::Scope { id : x, ..} => x,
			_ => return err!("scope_exchange(): outer cell is no scope")
		};
		let (inner_id, body) = match inner_scope_path.get_cell() {
			Cell::Scope { id : x, body : y } => (x, *y),
			_ => return err!("scope_exchange(): inner cell is no scope")
		};

		let rule = outer_scope_path.replace_by(scope(inner_id, scope(outer_id, body)));
		self.add_rule(rule)
	}

	pub fn case_creation(&mut self, cell_id : CellID, arg_cell : Cell) -> SRLResult<Cell> {
		let path = x!(cell_id.get_path(&self.rules));
		let wrapper = match path.get_wrapper() {
			Some(x) => x,
			None => return err!("case_creation(): no wrapper")
		};
		if !wrapper.is_positive() {
			return err!("case_creation(): wrapper is not positive");
		}
		let cell = path.get_cell();
		let rule = path.replace_by(case(arg_cell, cell));
		self.add_rule(rule)
	}

	// <(= 'false' {0 (= 'false' (p 0 1))})>
	pub fn declaration(&mut self, cell_id : CellID, string : &str) -> SRLResult<Cell> {
		// occurence checks
		if self.contains_cellname(string) {
			return err!("declaration(): string does already occur");
		}

		// wrapper checks
		let cell_path = x!(cell_id.get_path(&self.rules));
		let wrapper = match cell_path.get_wrapper() {
			Some(x) => x,
			None => return err!("declaration(): no wrapper")
		};
		if !wrapper.is_positive() {
			return err!("declaration(): wrapper is negative");
		}
		if !wrapper.is_nallq() {
			return err!("declaration(): wrapper contains all quantor");
		}

		// check (= 'false' {0 (= 'false' * )}) pattern
		let (x, y) = x!(cell_path.get_cell().get_equals_cell_arguments());
		if x != false_cell() {
			return err!("declaration(): first arg of equals cell is not 'false'");
		}
		let (id, body) = match y {
			Cell::Scope { id : x, body : y } => (x, y),
			_ => return err!("declaration(): second arg is no scope")
		};
		let (a, b) = x!(body.get_equals_cell_arguments());
		if a != false_cell() {
			return err!("declaration(): scope does not contain (= 'false' *)");
		}

		let new = b.replace_all(var(id), x!(try_simple(string.to_string())) );
		let rule = cell_path.replace_by(new);
		self.add_rule(rule)
	}
}
