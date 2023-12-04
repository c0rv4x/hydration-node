use super::*;

use frame_support::storage::with_transaction;
use hydradx_traits::registry::{AssetKind, Inspect};
use mock::Registry;

use pretty_assertions::assert_eq;
use sp_runtime::{DispatchResult, TransactionOutcome};

#[test]
fn is_sufficient_should_work() {
	let suff_asset_id = 1_u32;
	let insuff_asset_id = 2_u32;
	let non_existing_id = 3_u32;

	ExtBuilder::default()
		.with_assets(vec![
			(
				Some(suff_asset_id),
				Some(b"Suff".to_vec()),
				UNIT,
				None,
				None,
				None,
				true,
			),
			(
				Some(insuff_asset_id),
				Some(b"Insuff".to_vec()),
				UNIT,
				None,
				None,
				None,
				false,
			),
		])
		.build()
		.execute_with(|| {
			assert_eq!(<Registry as Inspect>::is_sufficient(suff_asset_id), true);

			assert_eq!(<Registry as Inspect>::is_sufficient(insuff_asset_id), false);

			assert_eq!(<Registry as Inspect>::is_sufficient(non_existing_id), false);
		});
}

#[test]
fn exists_should_work() {
	let asset_id = 2_u32;
	let non_existing_id = 3_u32;

	ExtBuilder::default()
		.with_assets(vec![(
			Some(asset_id),
			Some(b"Suff".to_vec()),
			UNIT,
			None,
			None,
			None,
			true,
		)])
		.build()
		.execute_with(|| {
			assert_eq!(<Registry as Inspect>::exists(asset_id), true);

			assert_eq!(<Registry as Inspect>::exists(non_existing_id), false);
		});
}

#[test]
fn decimals_should_work() {
	let non_existing_id = 543_u32;

	ExtBuilder::default()
		.with_assets(vec![
			(Some(1), Some(b"TKN1".to_vec()), UNIT, None, Some(5_u8), None, true),
			(Some(2), Some(b"TKN2".to_vec()), UNIT, None, Some(0_u8), None, true),
			(Some(3), Some(b"TKN3".to_vec()), UNIT, None, None, None, true),
		])
		.build()
		.execute_with(|| {
			assert_eq!(<Registry as Inspect>::decimals(1), Some(5));

			assert_eq!(<Registry as Inspect>::decimals(2), Some(0));

			assert_eq!(<Registry as Inspect>::decimals(3), None);

			assert_eq!(<Registry as Inspect>::decimals(non_existing_id), None);
		});
}

#[test]
fn asset_type_should_work() {
	let non_existing_id = 543_u32;

	ExtBuilder::default().build().execute_with(|| {
		let _ = with_transaction(|| {
			//Arrange
			let token_type_id =
				Registry::register_insufficient_asset(None, None, AssetKind::Token, None, None, None, None, None)
					.unwrap();
			let xyk_type_id =
				Registry::register_insufficient_asset(None, None, AssetKind::XYK, None, None, None, None, None)
					.unwrap();
			let stableswap_type_id =
				Registry::register_insufficient_asset(None, None, AssetKind::StableSwap, None, None, None, None, None)
					.unwrap();
			let bond_type_id =
				Registry::register_insufficient_asset(None, None, AssetKind::Bond, None, None, None, None, None)
					.unwrap();
			let external_type_id =
				Registry::register_insufficient_asset(None, None, AssetKind::External, None, None, None, None, None)
					.unwrap();

			//Assert
			assert_eq!(<Registry as Inspect>::asset_type(token_type_id), Some(AssetKind::Token));
			assert_eq!(<Registry as Inspect>::asset_type(xyk_type_id), Some(AssetKind::XYK));
			assert_eq!(
				<Registry as Inspect>::asset_type(stableswap_type_id),
				Some(AssetKind::StableSwap)
			);
			assert_eq!(<Registry as Inspect>::asset_type(bond_type_id), Some(AssetKind::Bond));
			assert_eq!(
				<Registry as Inspect>::asset_type(external_type_id),
				Some(AssetKind::External)
			);

			assert_eq!(<Registry as Inspect>::asset_type(non_existing_id), None);

			TransactionOutcome::Commit(DispatchResult::Ok(()))
		});
	});
}

#[test]
fn is_blacklisted_should_work() {
	ExtBuilder::default()
		.with_assets(vec![
			(Some(1), Some(b"Suff".to_vec()), UNIT, None, None, None, true),
			(Some(2), Some(b"Insuff".to_vec()), UNIT, None, None, None, false),
		])
		.build()
		.execute_with(|| {
			//Arrange
			//NOTE: update origin is set to ensure_signed in tests
			assert_ok!(Registry::blacklist_add(RuntimeOrigin::signed(ALICE), 1));

			//Act & assert
			assert_eq!(<Registry as Inspect>::is_blacklisted(1), true);

			assert_eq!(<Registry as Inspect>::is_blacklisted(2), false);
		});
}
