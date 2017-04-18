use cell::Cell;
use error::SRLError;
use cell::SimpleString;

pub fn true_cell() -> Cell {
	simple_by_str("'true'")
}

pub fn false_cell() -> Cell {
	simple_by_str("'false'")
}

pub fn equals_cell(cell1 : Cell, cell2 : Cell) -> Cell {
	complex(vec![simple_by_str("="), cell1, cell2])
}

pub fn try_simple(string_arg : String) -> Result<Cell, SRLError> {
	Ok(Cell::Simple { string : SimpleString::create(string_arg)? })
}

pub fn try_simple_by_str(string_arg : &str) -> Result<Cell, SRLError> {
	try_simple(string_arg.to_string())
}

pub fn simple(string_arg : String) -> Cell {
	try_simple(string_arg).unwrap()
}

pub fn simple_by_str(string_arg : &str) -> Cell {
	try_simple_by_str(string_arg).unwrap()
}

pub fn complex(cells_arg : Vec<Cell>) -> Cell {
	if cells_arg.len() < 2 {
		panic!("complex cell neeeds more than 1 argument");
	}
	Cell::Complex { cells : cells_arg }
}

pub fn scope(id : u32, body : Cell) -> Cell {
	return Cell::Scope { id : id, body : Box::new(body) };
}

pub fn var(id: u32) -> Cell {
	return Cell::Var { id : id };
}

pub fn case(condition : Cell, conclusion : Cell) -> Cell {
	return Cell::Case { condition : Box::new(condition), conclusion : Box::new(conclusion) };
}
