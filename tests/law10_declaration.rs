extern crate libsrl;
use libsrl::db::Database;
use libsrl::navi::CellID;

#[test]
fn test_declaration() {
	let mut db = match Database::by_string("= 'false' {0 (= 'false' (p 0))}.") {
		Ok(x) => x,
		Err(_) => panic!("panic!")
	};

	let cell_id = CellID::create(1, vec![]);

	match db.declaration(cell_id, "test_var") {
		Ok(x) => { assert_eq!(x.to_rule_string(), "p test_var."); },
		Err(_) => panic!("panic! (3)")
	}
}
