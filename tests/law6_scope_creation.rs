extern crate libsrl;
use libsrl::db::Database;
use libsrl::navi::CellID;

#[test]
fn test_scope_creation() {
	let mut db = match Database::by_string("= 'false' (= 'true' x).") {
		Ok(x) => x,
		Err(_) => panic!("panic!")
	};

	let cell_id = CellID::create(1, vec![2]);

	match db.scope_creation(cell_id, vec![vec![2]]) {
		Ok(x) => { assert_eq!(x.to_rule_string(), "= 'false' {0 (= 'true' 0)}."); }
		Err(srl_error) => panic!("panic! (2) err: {:?}", srl_error)
	}
}
