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
//! DATE: 2023-10-06, STEPS: 10, REPEAT: 30, LOW RANGE: [], HIGH RANGE: []
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
// --output=route-executor.rs
// --extrinsic=*

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_route_executor.
pub trait WeightInfo {
	fn calculate_and_execute_sell_in_lbp(c: u32, s: u32) -> Weight;
	fn calculate_and_execute_buy_in_lbp(c: u32, b: u32) -> Weight;
}

/// Weights for pallet_route_executor using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:1 w:1)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	/// The range of component `c` is `[0, 1]`.
	/// The range of component `s` is `[0, 1]`.
	fn calculate_and_execute_sell_in_lbp(c: u32, s: u32) -> Weight {
		// Minimum execution time: 74_851 nanoseconds.
		Weight::from_ref_time(26_266_260 as u64) // Standard Error: 251_240
			.saturating_add(Weight::from_ref_time(49_596_533 as u64).saturating_mul(c as u64))
			// Standard Error: 251_240
			.saturating_add(Weight::from_ref_time(252_604_739 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((5 as u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes((6 as u64).saturating_mul(s as u64)))
	}
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:1 w:1)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 2]`.
	/// The range of component `b` is `[0, 1]`.
	fn calculate_and_execute_buy_in_lbp(c: u32, b: u32) -> Weight {
		// Minimum execution time: 73_996 nanoseconds.
		Weight::from_ref_time(74_590_000 as u64) // Standard Error: 576_133
			.saturating_add(Weight::from_ref_time(2_213_808 as u64).saturating_mul(c as u64))
			// Standard Error: 1_264_777
			.saturating_add(Weight::from_ref_time(205_965_931 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((5 as u64).saturating_mul(b as u64)))
			.saturating_add(T::DbWeight::get().writes((6 as u64).saturating_mul(b as u64)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:1 w:1)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	/// The range of component `c` is `[0, 1]`.
	/// The range of component `s` is `[0, 1]`.
	fn calculate_and_execute_sell_in_lbp(c: u32, s: u32) -> Weight {
		// Minimum execution time: 74_851 nanoseconds.
		Weight::from_ref_time(26_266_260 as u64) // Standard Error: 251_240
			.saturating_add(Weight::from_ref_time(49_596_533 as u64).saturating_mul(c as u64))
			// Standard Error: 251_240
			.saturating_add(Weight::from_ref_time(252_604_739 as u64).saturating_mul(s as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().reads((5 as u64).saturating_mul(s as u64)))
			.saturating_add(RocksDbWeight::get().writes((6 as u64).saturating_mul(s as u64)))
	}
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:3)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Balances Locks (r:1 w:1)
	// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 2]`.
	/// The range of component `b` is `[0, 1]`.
	fn calculate_and_execute_buy_in_lbp(c: u32, b: u32) -> Weight {
		// Minimum execution time: 73_996 nanoseconds.
		Weight::from_ref_time(74_590_000 as u64) // Standard Error: 576_133
			.saturating_add(Weight::from_ref_time(2_213_808 as u64).saturating_mul(c as u64))
			// Standard Error: 1_264_777
			.saturating_add(Weight::from_ref_time(205_965_931 as u64).saturating_mul(b as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().reads((5 as u64).saturating_mul(b as u64)))
			.saturating_add(RocksDbWeight::get().writes((6 as u64).saturating_mul(b as u64)))
	}
}
