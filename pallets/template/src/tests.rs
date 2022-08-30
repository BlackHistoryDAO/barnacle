use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_adds_qualifier_should_work() {
	new_test_ext().execute_with(|| {
		//Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::add_qualifier(Origin::root(),1,b"abc".to_vec()));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::get_total_qualifiers(), 1);
	});
}

#[test]
fn it_adds_qualifier_should_fail() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::add_qualifier(Origin::root(),1,b"abc".to_vec()));
		assert_noop!(TemplateModule::add_qualifier(Origin::root(),1,b"abc".to_vec()), Error::<Test>::QualifierAlreadyExists);
	});
}
