use error::SRLResult;
use cell::Cell;
use gen::*;

fn get_new_id(old_id : u32, scope_ids : &Vec<u32>, in_scope_vec : &Vec<bool>) -> SRLResult<u32> {
	for index in 0..scope_ids.len() {
		if old_id == scope_ids[index as usize] {
			if in_scope_vec[index] {
				return ok!(index as u32);
			} else {
				return err!("get_new_id(): already out of scope with id '{}'", old_id);
			}
		}
	}
	return err!("get_new_id(): id '{}' is not in scope_ids '{:?}'", old_id, scope_ids);
}

#[test]
fn test_get_normalized() {
	if complex(vec![var(0), scope(0, simple_by_str("ok"))]).get_normalized().is_ok() { panic!("test_get_normalized(): should not accept (0)"); }
	if complex(vec![scope(0, simple_by_str("ok")), var(0)]).get_normalized().is_ok() { panic!("test_get_normalized(): should not accept (1)"); }
	if scope(0, scope(0, simple_by_str("wow"))).get_normalized().is_ok() { panic!("test_get_normalized(): should not accept (2)"); }
	assert_eq!(scope(0, scope(1, var(1))).to_string(), scope(1, scope(2, var(2))).get_normalized().unwrap().to_string());
}


impl Cell {
       // creates new cell with normalized scopes
       // -- errors on var out of scope/multiple scopes with same id

	pub fn get_normalized(&self) -> SRLResult<Cell> {
		return self.get_normalized_from_r(&mut vec![], &mut vec![], 0);
	}

	pub fn get_normalized_from(&self, from : u32) -> SRLResult<Cell> {
		return self.get_normalized_from_r(&mut vec![], &mut vec![], from);
	}


	fn get_normalized_from_r(&self, vec : &mut Vec<u32>, in_scope_vec : &mut Vec<bool>, from : u32) -> SRLResult<Cell> {
		match &self {
			&&Cell::Simple { string : ref string_out } => {
				return ok!(x!( try_simple(string_out.get_string().to_string()) ));
			}
			&&Cell::Complex { cells : ref cells_out } => {
				let mut new_cells = Vec::new();
				for cell in cells_out {
					let norm = x!(cell.get_normalized_from_r(vec, in_scope_vec, from));
					new_cells.push(norm);
				}
				return ok!(complex(new_cells));
			}
			&&Cell::Scope { id : id_out, body : ref body_out } => {
				if vec.contains(&id_out) {
					return err!("get_normalized_from_r(): id '{}' used twice", id_out);
				}

				vec.push(id_out);
				in_scope_vec.push(true);
				let new_id = (vec.len() - 1) as u32 + from;
				let new_body = x!(body_out.get_normalized_from_r(vec, in_scope_vec, from));
				in_scope_vec.pop();
				in_scope_vec.push(false);
				return ok!(scope(new_id, new_body));
			}
			&&Cell::Var { id : id_out } => {
				let new_id = x!(get_new_id(id_out, vec, in_scope_vec));
				return ok!(Cell::Var { id : new_id + from });
			}
			&&Cell::Case { condition : ref condition_out, conclusion : ref conclusion_out } =>  {
				let condition_new = x!(condition_out.get_normalized_from_r(vec, in_scope_vec, from));
				let conclusion_new = x!(conclusion_out.get_normalized_from_r(vec, in_scope_vec, from));
				return ok!(case(condition_new, conclusion_new));
			}
		}
	}

	pub fn get_next_id(&self) -> usize {
		fn find_highest(cell : &Cell, i : i32) -> i32 {
			if let &Cell::Scope { id : x, ..} = cell {
				if x as i32 > i { x as i32 }
				else { i }
			} else {
				i
			}

		}
		(self.recurse::<i32>(-1, find_highest) + 1) as usize
	}
}
