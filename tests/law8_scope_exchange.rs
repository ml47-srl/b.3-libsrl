extern crate libsrl;
use libsrl::db::Database;
use libsrl::navi::CellID;

#[test]
fn test_scope_exchange() {
	let mut db = match Database::by_string("{0 {1 (= 0 1) }}.") {
		Ok(x) => x,
		Err(_) => panic!("panic!")
	};

	let cell_id = CellID::create(1, vec![]);
	match db.scope_exchange(cell_id) {
		Ok(x) => { assert_eq!(x.to_rule_string(), "{0 {1 (= 1 0)}}."); }
		Err(srl_error) => panic!("panic! (3) err: {:?}", srl_error)
	}
}
