use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_adds_qualifier_should_work() {
	new_test_ext().execute_with(|| {
		//Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::add_qualifier(Origin::root(),1));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::qualifiers_uid_count(), 1);
	});
}

#[test]
fn it_adds_qualifier_should_fail() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::add_qualifier(Origin::root(),1));
		assert_noop!(TemplateModule::add_qualifier(Origin::root(),1), Error::<Test>::QualifierAlreadyExists);
	});
}

#[test]
fn it_adds_contributor_should_work() {
	new_test_ext().execute_with(|| {
		//Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::add_contributor(Origin::root(),1));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::contributors_uid_count(), 1);
	});
}

#[test]
fn it_adds_contributor_should_fail() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::add_contributor(Origin::root(),1));
		assert_noop!(TemplateModule::add_contributor(Origin::root(),1), Error::<Test>::ContributorAlreadyExists);
	});
}

#[test]
fn it_adds_collector_should_work() {
	new_test_ext().execute_with(|| {
		//Dispatch a signed extrinsic.
		assert_ok!(TemplateModule::add_collector(Origin::root(),1));
		// Read pallet storage and assert an expected result.
		assert_eq!(TemplateModule::collectors_uid_count(), 1);
	});
}

#[test]
fn it_adds_collector_should_fail() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::add_collector(Origin::root(),1));
		assert_noop!(TemplateModule::add_collector(Origin::root(),1), Error::<Test>::CollectorAlreadyExists);
	});
}

#[test]
fn it_creates_document_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::add_contributor(Origin::root(),2));
		assert_ok!(TemplateModule::create_document(Origin::signed(2),b"Doc1".to_vec(),b"Test1".to_vec(),b"pdf".to_vec(),b"https://ipfs.hash".to_vec()));
		assert_eq!(TemplateModule::get_total_items(),1);
	});
	
}

#[test]
fn it_changes_qualification_voting_window_should_work() {
	new_test_ext().execute_with(|| {
		assert_eq!(TemplateModule::get_qualification_voting_window(),14400u32);
		assert_ok!(TemplateModule::set_qualification_voting_window(Origin::root(),1000u32));
		assert_eq!(TemplateModule::get_qualification_voting_window(),1000u32);
	});	
}

#[test]
fn it_changes_verification_voting_window_should_work() {
	new_test_ext().execute_with(|| {
		assert_eq!(TemplateModule::get_verification_voting_window(),14400u32);
		assert_ok!(TemplateModule::set_verification_voting_window(Origin::root(),1000u32));
		assert_eq!(TemplateModule::get_verification_voting_window(),1000u32);
	});	
}

#[test]
fn it_creates_qualification_voting_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(TemplateModule::add_contributor(Origin::root(),2));
		assert_ok!(TemplateModule::create_document(Origin::signed(2),b"Doc1".to_vec(),b"Test1".to_vec(),b"pdf".to_vec(),b"https://ipfs.hash".to_vec()));
		assert_eq!(TemplateModule::get_total_items(),1);
		assert_ok!(TemplateModule::create_qualification_voting(Origin::root(),1));
	});	
}
