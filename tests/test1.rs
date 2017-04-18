extern crate libsrl;

use libsrl::db::Database;

#[test]
fn test_to_string() {
	assert_eq!("(equals a b)", Database::by_string("equals a b.").unwrap().get_rule(1).to_string());
	assert_eq!("(equals a b)", Database::by_string("(equals a b).").unwrap().get_rule(1).to_string());
	assert_eq!("(equals a b)", Database::by_string("((equals a b)).").unwrap().get_rule(1).to_string());
	assert_eq!("a", Database::by_string("(a).").unwrap().get_rule(1).to_string());
	assert_eq!("(a b)", Database::by_string("(a) b.").unwrap().get_rule(1).to_string());
}

#[test]
fn test_to_unwrapped_string() {
	assert_eq!("equals a b", Database::by_string("equals a b.").unwrap().get_rule(1).to_unwrapped_string());
	assert_eq!("equals a b", Database::by_string("(equals a b).").unwrap().get_rule(1).to_unwrapped_string());
	assert_eq!("equals a b", Database::by_string("((equals a b)).").unwrap().get_rule(1).to_unwrapped_string());
	assert_eq!("a", Database::by_string("(a).").unwrap().get_rule(1).to_unwrapped_string());
	assert_eq!("a b", Database::by_string("(a) b.").unwrap().get_rule(1).to_unwrapped_string());
}

#[test]
fn test_to_rule_string() {
	assert_eq!("equals a b.", Database::by_string("equals a b.").unwrap().get_rule(1).to_rule_string());
	assert_eq!("equals a b.", Database::by_string("(equals a b).").unwrap().get_rule(1).to_rule_string());
	assert_eq!("equals a b.", Database::by_string("((equals a b)).").unwrap().get_rule(1).to_rule_string());
	assert_eq!("a.", Database::by_string("(a).").unwrap().get_rule(1).to_rule_string());
	assert_eq!("a b.", Database::by_string("(a) b.").unwrap().get_rule(1).to_rule_string());
}
