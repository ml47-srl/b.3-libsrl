extern crate libsrl;
use libsrl::error::*;
use libsrl::db::Database;
use libsrl::navi::CellID;

#[test]
fn test_inequal_constants() {
	let mut db = match Database::by_string("= p (= 'x' 'y').") {
		SRLResult::Ok(x) => x,
		SRLResult::Err(_) => panic!("panic!")
	};

	let cell_id = CellID::create(1, vec![2]);
	match db.inequal_constants(cell_id) {
		SRLResult::Ok(x) => { assert_eq!(x.to_rule_string(), "= p 'false'."); }
		SRLResult::Err(_) => panic!("panic! (2)")
	}
}
