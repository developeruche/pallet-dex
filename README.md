# DEX Pallet
--------------

Custom DEX pallet using the Polkadot SDK based of UniswapV2 implementation.



### Usage

Configuring pallet-dex v2 to a solo-chain or a parachain is a simple process.

First, add the following to your `Cargo.toml`:

```toml
pallet-dex-v2 = { version = "0.0.3", default-features = false}
# if you don't use pallet-assets in  your runtime before, you need to add it to your runtime
pallet-assets = { version = "32.0.0", default-features = false }
```

Then, add the following to your runtime's `lib.rs`:

```rust
pub use pallet_dex_v2 as dex;

// declare pallet id for dex
parameter_types! {
    // pallet ID
    pub const DexPallet: PalletId = PalletId(*b"DCitadel");
}

// declare types used in asset pallet
parameter_types! {
	pub const AssetDeposit: Balance = 100;
	pub const ApprovalDeposit: Balance = 1;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: Balance = 10;
	pub const MetadataDepositPerByte: Balance = 1;
}

/// Configure the assets pallet
impl pallet_assets::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type AssetId = u32;
	type AssetIdParameter = codec::Compact<u32>;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = AssetDeposit;
	type AssetAccountDeposit = ConstU128<1>;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
	type RemoveItemsLimit = ConstU32<1000>;
	type CallbackHandle = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

/// Configure the dex pallet
impl pallet_dex::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type NativeBalance = Balances;
    type Fungibles = Assets;
    type PalletId = DexPallet;
}

```


### License


### Contributing
