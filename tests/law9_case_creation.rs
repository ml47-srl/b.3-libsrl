extern crate libsrl;
use libsrl::db::Database;
use libsrl::navi::CellID;
use libsrl::gen::*;

#[test]
fn test_case_creation() {
	let mut db = match Database::by_string("= 'true' y.") {
		Ok(x) => x,
		Err(_) => panic!("panic!")
	};

	let cell_id = CellID::create(1, vec![]);
	let cell = equals_cell(simple_by_str("'true'"), simple_by_str("x"));

	match db.case_creation(cell_id, cell) {
		Ok(x) => { assert_eq!(x.to_rule_string(), "[=> (= 'true' x) (= 'true' y)]."); }
		Err(srl_error) => panic!("panic! (3) err: {:?}", srl_error)
	}
}
