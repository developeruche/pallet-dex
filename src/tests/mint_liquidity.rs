use crate::{mock::*, AccountIdOf, Error, Event, LiquidityPools, Pallet, LiquidityTokens};
use frame_support::{assert_noop, assert_ok, storage::child::get};
use frame_system::RawOrigin;
use sp_core::H256;



#[test]
fn test_should_mint_liquidity() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let account: AccountIdOf<Test> = 4;
        let asset_a = 10;
        let asset_b = 20;
        let liquidity_token = 30;
        let origin = RawOrigin::Signed(account);
        let pool_pair = (asset_a, asset_b);
        let pool = Pallet::<Test>::create_liquidity_pool(origin.clone().into(), asset_a, asset_b, liquidity_token).unwrap();

        let pool = LiquidityPools::<Test>::get(pool_pair).unwrap();

        let mint = Pallet::<Test>::mint_liquidity(
            origin.into(),
            asset_a,
            asset_b,
            1000,
            1000,
            0
        ).unwrap();
    });
}

