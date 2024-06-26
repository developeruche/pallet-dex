use crate::{mock::*, AccountIdOf, LiquidityPools, Pallet};
use frame_system::RawOrigin;



#[test]
fn test_should_mint_liquidity() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);

        let account: AccountIdOf<Test> = 4;
        let origin = RawOrigin::Signed(account);
        let force_origin = RawOrigin::Root;



        let asset_a = 10;
        let asset_b = 20;
        let liquidity_token = 30;

        // creating this asset
        Assets::force_create(force_origin.clone().into(), asset_a.into(), account, true, 1).unwrap();
        Assets::force_create(force_origin.clone().into(), asset_b.into(), account, true, 1).unwrap();
        Assets::force_create(force_origin.clone().into(), liquidity_token.into(), account, true, 1).unwrap();


        // minting tokens to the liquidty provider
        Assets::mint(origin.clone().into(), asset_a.into(), account, 1000).unwrap();
        Assets::mint(origin.clone().into(), asset_b.into(), account, 1000).unwrap();



        let pool_pair = (asset_a, asset_b);
        Pallet::<Test>::create_liquidity_pool(origin.clone().into(), asset_a, asset_b, liquidity_token).unwrap();

        let mut pool = LiquidityPools::<Test>::get(pool_pair).unwrap();

        Pallet::<Test>::mint_liquidity(
            origin.clone().into(),
            asset_a,
            asset_b,
            100,
            100,
            0
        ).unwrap();


        let balance_a = Assets::balance(asset_a, account);
        let balance_b = Assets::balance(asset_b, account);
        let balance_lp = Assets::balance(liquidity_token, account);

        pool = LiquidityPools::<Test>::get(pool_pair).unwrap();
        let total_liquidity = pool.total_liquidity;


        assert_eq!(balance_a, 900);
        assert_eq!(balance_b, 900);
        assert_eq!(balance_lp, 100);
        assert_eq!(total_liquidity, 100);

        Pallet::<Test>::burn_liquidity(origin.into(), asset_a, asset_b, 50, 0, 0).unwrap();

        let balance_a = Assets::balance(asset_a, account);
        let balance_b = Assets::balance(asset_b, account);
        let balance_lp = Assets::balance(liquidity_token, account);

        pool = LiquidityPools::<Test>::get(pool_pair).unwrap();
        let total_liquidity = pool.total_liquidity;

        assert_eq!(balance_a, 950);
        assert_eq!(balance_b, 950);
        assert_eq!(balance_lp, 50);
        assert_eq!(total_liquidity, 50);
    });
}
