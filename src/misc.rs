use cell::Cell;
use error::SRLError;
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
	pub fn get_equals_cell_arguments(&self) -> Result<(Cell, Cell), SRLError> {
		if let &Cell::Complex { cells : ref cells_out } = self {
			if cells_out.len() != 3 {
				return Err(SRLError("get_equals_cell_arguments".to_string(), "complex cell does not have 3 arguments".to_string()));
			}
			if cells_out[0] != simple_by_str("=") {
				return Err(SRLError("get_equals_cell_arguments".to_string(), "first cell is not =".to_string()));
			}
			return Ok((cells_out[1].clone(), cells_out[2].clone()));
		} else {
			return Err(SRLError("get_equals_cell_arguments".to_string(), "cell is not complex".to_string()));
		}
	}
}
