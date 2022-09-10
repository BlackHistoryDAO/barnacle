use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_creates_a_collection_should_work() {
	new_test_ext().execute_with(|| {
        assert_eq!(Nft::get_total_collections(), 0);
		//Dispatch a signed extrinsic.
		assert_ok!(Nft::create_collection(Origin::root(),1,200,b"Qualifier".to_vec()));
		// Read pallet storage and assert an expected result.
		assert_eq!(Nft::get_total_collections(), 1);
	});
}

#[test]
fn it_mints_a_token_should_work() {
	new_test_ext().execute_with(|| {
        assert_eq!(Nft::get_total_collections(), 0);
		//Dispatch a signed extrinsic.
		assert_ok!(Nft::create_collection(Origin::root(),1,200,b"Qualifier".to_vec()));
        //Check # of tokens for collection 1
        assert_eq!(Nft::get_total_tokens(1u32),0u32);
		//Mint a token
        assert_ok!(Nft::mint(Origin::root(),1,1,b"Token1".to_vec()));
        // # of total tokens should increase by 1
        assert_eq!(Nft::get_total_tokens(1u32),1u32);

	});
}