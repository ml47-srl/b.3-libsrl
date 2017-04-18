extern crate libsrl;
use libsrl::db::Database;
use libsrl::navi::CellID;

#[test]
fn test_add_eqt() {
	let mut db = match Database::by_string("{0 wow }.") {
		Ok(x) => x,
		Err(_) => panic!("panic!")
	};

	let cell_id = CellID::create(1, vec![0]);
	match db.add_eqt(cell_id) {
		Ok(x) => { assert_eq!(x.to_rule_string(), "{0 (= 'true' wow)}."); }
		Err(_) => panic!("panic! (2)")
	}
}

#[test]
fn test_rm_eqt() {
	let mut db = match Database::by_string("{0 (= 'true' (= a b))}.") {
		Ok(x) => x,
		Err(_) => panic!("panic!")
	};

	let cell_id = CellID::create(1, vec![0, 2]);
	match db.rm_eqt(cell_id) {
		Ok(x) => { assert_eq!(x.to_rule_string(), "{0 (= a b)}."); }
		Err(_) => panic!("panic! (2)")
	}
}
