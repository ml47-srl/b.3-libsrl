extern crate libsrl;
use libsrl::db::Database;

#[test]
fn test_core_rule() {
	let db = match Database::by_string("") {
		Ok(x) => x,
		Err(_) => panic!("panic!")
	};

	assert_eq!(db.get_rule(0).to_rule_string(), "{0 (= 0 0)}.");
	// woohoo
}
