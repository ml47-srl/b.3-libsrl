extern crate libsrl;
use libsrl::db::Database;
use libsrl::navi::CellID;

#[test]
fn test_inequal_constants() {
	let mut db = match Database::by_string("= p (= 'x' 'y').") {
		Ok(x) => x,
		Err(_) => panic!("panic!")
	};

	let cell_id = CellID::create(1, vec![2]);
	match db.inequal_constants(cell_id) {
		Ok(x) => { assert_eq!(x.to_rule_string(), "= p 'false'."); }
		Err(_) => panic!("panic! (2)")
	}
}
