extern crate libsrl;
use libsrl::db::Database;
use libsrl::navi::CellID;
use libsrl::gen::*;
use libsrl::error::SRLResult;

#[test]
fn test_equals_law_1() {
	let mut db = match Database::by_string("(= x y). {0 (p x)}. {0 (= (this 0) 0)}.") {
		SRLResult::Ok(x) => x,
		SRLResult::Err(_) => panic!("panic!")
	};

	assert_eq!(db.get_rule(1).to_rule_string(), "= x y.");
	assert_eq!(db.get_rule(2).to_rule_string(), "{0 (p x)}.");
	assert_eq!(db.get_rule(3).to_rule_string(), "{0 (= (this 0) 0)}.");
	let evi = CellID::create(1, vec![]);
	let src = CellID::create(2, vec![0, 1]);
	match db.equals_law(src, evi) {
		SRLResult::Ok(x) => { assert_eq!(x.to_rule_string(), "{0 (p y)}."); }
		SRLResult::Err(_) => panic!("failure!")
	};
	assert_eq!(db.get_rule(4).to_rule_string(), "{0 (p y)}.");

	let cell = complex(vec![simple_by_str("a"), simple_by_str("b")]);

	let blub = CellID::create(3, vec![]);
	match db.scope_insertion(blub, cell) {
		SRLResult::Ok(x) => println!("rule: {}", x.to_rule_string()),
		SRLResult::Err(srl_error) => panic!("panic: {:?}", srl_error)
	}
}

#[test]
fn test_equals_law_2() {
	let mut db = Database::by_string("{0 (= (this 0) 0)}. {0 (= (self 0) 0)}.").unwrap();

	let evi = CellID::create(1, vec![0]);
	let src = CellID::create(2, vec![0, 2]);
	match db.equals_law(src, evi) {
		SRLResult::Ok(x) => { assert_eq!(x.to_rule_string(), "{0 (= (self 0) (this 0))}."); }
		SRLResult::Err(_) => panic!("failure!")
	};
}

#[test]
fn test_equals_law_3() {
	let mut db = Database::by_string("(= a b). p b.").unwrap();
	let evi = CellID::create(1, vec![]);
	let src = CellID::create(2, vec![1]);
	match db.equals_law(src, evi) {
		SRLResult::Ok(x) => { assert_eq!(x.to_rule_string(), "p a."); }
		SRLResult::Err(_) => panic!("failure!")
	};
}
