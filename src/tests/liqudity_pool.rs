use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};



#[test]
fn test_should_mint_lp() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        
    });
}
