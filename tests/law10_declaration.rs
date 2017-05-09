extern crate libsrl;
use libsrl::error::*;
use libsrl::db::Database;
use libsrl::navi::CellID;

#[test]
fn test_declaration() {
	let mut db = Database::by_string("= 'false' {0 (= 'false' (p 0))}.").unwrap();

	let cell_id = CellID::create(1, vec![]);

	match db.declaration(cell_id, "test_var") {
		SRLResult::Ok(x) => { assert_eq!(x.to_rule_string(), "p test_var."); },
		SRLResult::Err(_) => panic!("panic! (3)")
	}
}
