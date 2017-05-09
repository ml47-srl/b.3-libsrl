extern crate libsrl;
use libsrl::error::*;
use libsrl::db::Database;
use libsrl::navi::CellID;

#[test]
fn test_scope_exchange() {
	let mut db = match Database::by_string("{0 {1 (= 0 1) }}.") {
		SRLResult::Ok(x) => x,
		SRLResult::Err(_) => panic!("panic!")
	};

	let cell_id = CellID::create(1, vec![]);
	match db.scope_exchange(cell_id) {
		SRLResult::Ok(x) => { assert_eq!(x.to_rule_string(), "{0 {1 (= 1 0)}}."); }
		SRLResult::Err(srl_error) => panic!("panic! (3) err: {:?}", srl_error)
	}
}
