use cell::Cell;
use error::*;
use gen::*;

pub fn contains_only(string : String, list : String) -> bool {
	for thing in string.chars() {
		if !list.contains(thing) {
			return false;
		}
	}
	true
}

pub fn contains_some(string : String, list : String) -> bool {
	for thing in string.chars() {
		if list.contains(thing) {
			return true;
		}
	}
	false
}

pub fn index_in_len(index : usize, len : usize) -> bool {
	index < len
}

impl Cell {
	pub fn get_equals_cell_arguments(&self) -> SRLResult<(Cell, Cell)> {
		if let &Cell::Complex { cells : ref cells_out } = self {
			if cells_out.len() != 3 {
				return err!("get_equals_cell_arguments(): complex cell does not have 3 arguments");
			}
			if cells_out[0] != simple_by_str("=") {
				return err!("get_equals_cell_arguments(): first cell is not =");
			}
			return ok!((cells_out[1].clone(), cells_out[2].clone()));
		} else {
			return err!("get_equals_cell_arguments(): cell is not complex");
		}
	}
}
