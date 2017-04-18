extern crate libsrl;
use libsrl::db::Database;

#[test]
fn test_tricky() {
	let db = match Database::by_string("(a b) (c d).(a b) (c d) (e f).") {
		Ok(x) => x,
		Err(_) => panic!("panic!")
	};

	assert_eq!(db.get_rule(1).to_rule_string(), "(a b) (c d).");
	assert_eq!(db.get_rule(2).to_rule_string(), "(a b) (c d) (e f).");

	assert_eq!(db.get_rule(1).to_string(), "((a b) (c d))");
	assert_eq!(db.get_rule(2).to_string(), "((a b) (c d) (e f))");
}
