use crate::{mock::*, AccountIdOf, Error, Event, LiquidityPools, Pallet, LiquidityTokens};
use frame_support::{assert_noop, assert_ok, storage::child::get};
use frame_system::RawOrigin;
use sp_core::H256;



#[test]
fn test_should_create_liquidity_pool() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let account: AccountIdOf<Test> = 4;
        let asset_a = 10;
        let asset_b = 20;
        let liquidity_token = 30;
        let origin = RawOrigin::Signed(account);
        let pool = Pallet::<Test>::create_liquidity_pool(origin.into(), asset_a, asset_b, liquidity_token).unwrap();

        let pool_pair = (asset_a, asset_b);
        let pool = LiquidityPools::<Test>::get(pool_pair).unwrap();

        assert_eq!(pool.liquidity_token, liquidity_token);
        assert_eq!(pool.total_liquidity, 0);
        assert_eq!(pool.liquidity_token, 30);
        assert_eq!(pool.assets, pool_pair);
        assert_eq!(pool.reserves, (0, 0));

        let liquidity_token = LiquidityTokens::<Test>::get(liquidity_token);
        assert_eq!(liquidity_token, pool_pair);
    });
}


#[test]
fn test_should_not_create_liquidity_pool_with_same_assets() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let account: AccountIdOf<Test> = 4;
        let asset_a = 10;
        let asset_b = 10;
        let liquidity_token = 30;
        let origin = RawOrigin::Signed(account);
        let _ = Pallet::<Test>::create_liquidity_pool(origin.clone().into(), asset_a, asset_b, liquidity_token);
        let result = Pallet::<Test>::create_liquidity_pool(origin.into(), asset_a, asset_b, liquidity_token);

        assert_noop!(result, Error::<Test>::LiquidityPoolAlreadyExists);
    });
}

#[test]
fn ensure_only_signed_origin_can_create_liquidity_pool() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let account: AccountIdOf<Test> = 4;
        let asset_a = 10;
        let asset_b = 20;
        let liquidity_token = 30;
        let origin = RawOrigin::Root;
        let result = Pallet::<Test>::create_liquidity_pool(origin.into(), asset_a, asset_b, liquidity_token);

        assert_noop!(result, sp_runtime::DispatchError::BadOrigin);
    });
}


#[test]
fn ensure_event_was_emitted_on_pool_creation() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let account: AccountIdOf<Test> = 4;
        let asset_a = 10;
        let asset_b = 20;
        let liquidity_token = 30;
        let origin = RawOrigin::Signed(account);
        let _ = Pallet::<Test>::create_liquidity_pool(origin.into(), asset_a, asset_b, liquidity_token);
        let pool_pair = (asset_a, asset_b);



        let event = Event::LiquidityPoolCreated(account, pool_pair);
        assert!(System::events().iter().any(|a| a.event == RuntimeEvent::Dex(event.clone())));
    });
}