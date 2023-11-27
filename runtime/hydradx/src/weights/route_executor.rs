// This file is part of HydraDX.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_route_executor
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-24, STEPS: 10, REPEAT: 30, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --chain=dev
// --steps=10
// --repeat=30
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template-no-back.hbs
// --pallet=pallet-route-executor
// --output=route-executorp.rs
// --extrinsic=*

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_route_executor::weights::WeightInfo;

/// Weights for pallet_route_executor using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:5 w:5)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	/// The range of component `c` is `[0, 1]`.
	/// The range of component `s` is `[0, 1]`.
	fn calculate_and_execute_sell_in_lbp(c: u32, s: u32) -> Weight {
		// Minimum execution time: 76_217 nanoseconds.
		Weight::from_ref_time(25_542_379 as u64) // Standard Error: 108_648
			.saturating_add(Weight::from_ref_time(51_664_642 as u64).saturating_mul(c as u64))
			// Standard Error: 108_648
			.saturating_add(Weight::from_ref_time(299_167_877 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((9 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes((7 as u64).saturating_mul(s as u64)))
	}
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:5 w:5)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 2]`.
	/// The range of component `b` is `[0, 1]`.
	fn calculate_and_execute_buy_in_lbp(c: u32, b: u32) -> Weight {
		// Minimum execution time: 76_324 nanoseconds.
		Weight::from_ref_time(77_144_000 as u64) // Standard Error: 602_104
			.saturating_add(Weight::from_ref_time(2_491_556 as u64).saturating_mul(c as u64))
			// Standard Error: 1_321_790
			.saturating_add(Weight::from_ref_time(250_641_794 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((9 as u64).saturating_mul(b as u64)))
			.saturating_add(T::DbWeight::get().writes((7 as u64).saturating_mul(b as u64)))
	}
	// Storage: Router Routes (r:1 w:1)
	// Proof: Router Routes (max_values: None, max_size: Some(90), added: 2565, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:9 w:0)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:3 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: System Account (r:4 w:0)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:3 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	// Storage: XYK ShareToken (r:3 w:0)
	// Proof: XYK ShareToken (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: EmaOracle Accumulator (r:1 w:0)
	// Proof: EmaOracle Accumulator (max_values: Some(1), max_size: Some(5921), added: 6416, mode: MaxEncodedLen)
	fn set_route_for_xyk() -> Weight {
		// Minimum execution time: 651_581 nanoseconds.
		Weight::from_ref_time(655_822_000 as u64)
			.saturating_add(T::DbWeight::get().reads(26 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
}
