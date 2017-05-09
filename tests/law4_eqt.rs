extern crate libsrl;
use libsrl::error::SRLResult;
use libsrl::db::Database;
use libsrl::navi::CellID;

#[test]
fn test_add_eqt() {
	let mut db = Database::by_string("{0 wow }.").unwrap();

	let cell_id = CellID::create(1, vec![0]);
	match db.add_eqt(cell_id) {
		SRLResult::Ok(x) => { assert_eq!(x.to_rule_string(), "{0 (= 'true' wow)}."); }
		SRLResult::Err(_) => panic!("panic! (2)")
	}
}

#[test]
fn test_rm_eqt() {
	let mut db = Database::by_string("{0 (= 'true' (= a b))}.").unwrap();

	let cell_id = CellID::create(1, vec![0, 2]);
	match db.rm_eqt(cell_id) {
		SRLResult::Ok(x) => { assert_eq!(x.to_rule_string(), "{0 (= a b)}."); }
		SRLResult::Err(_) => panic!("panic! (2)")
	}
}
