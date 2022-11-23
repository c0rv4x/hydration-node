// This file is part of Hydra-node.

// Copyright (C) 2020-2021  Intergalactic, Limited (GIB).
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

//! Autogenerated weights for pallet_democracy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-30, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// --chain=local
// --steps=5
// --repeat=20
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template-no-back.hbs
// --pallet=pallet_democracy
// --output=democracy.rs
// --extrinsic=*
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_democracy::weights::WeightInfo;

pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	fn propose() -> Weight {
		Weight::from_ref_time(53_468_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn second(s: u32) -> Weight {
		Weight::from_ref_time(34_429_000 as u64) // Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(183_000 as u64).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn vote_new(r: u32) -> Weight {
		Weight::from_ref_time(41_810_000 as u64) // Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(197_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn vote_existing(r: u32) -> Weight {
		Weight::from_ref_time(41_736_000 as u64) // Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(200_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn emergency_cancel() -> Weight {
		Weight::from_ref_time(20_589_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	fn blacklist(p: u32) -> Weight {
		Weight::from_ref_time(48_500_000 as u64) // Standard Error: 48_000
			.saturating_add(Weight::from_ref_time(617_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	fn external_propose(v: u32) -> Weight {
		Weight::from_ref_time(9_935_000 as u64) // Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(80_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn external_propose_majority() -> Weight {
		Weight::from_ref_time(2_086_000 as u64).saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn external_propose_default() -> Weight {
		Weight::from_ref_time(1_984_000 as u64).saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn fast_track() -> Weight {
		Weight::from_ref_time(20_973_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn veto_external(v: u32) -> Weight {
		Weight::from_ref_time(21_466_000 as u64) // Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(100_000 as u64).saturating_mul(v as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	fn cancel_proposal(p: u32) -> Weight {
		Weight::from_ref_time(48_281_000 as u64) // Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(385_000 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	fn cancel_referendum() -> Weight {
		Weight::from_ref_time(13_026_000 as u64).saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn cancel_queued(r: u32) -> Weight {
		Weight::from_ref_time(25_113_000 as u64) // Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(1_813_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	fn on_initialize_base(r: u32) -> Weight {
		Weight::from_ref_time(3_860_000 as u64) // Standard Error: 18_000
			.saturating_add(Weight::from_ref_time(3_916_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn on_initialize_base_with_launch_period(r: u32) -> Weight {
		Weight::from_ref_time(10_189_000 as u64) // Standard Error: 16_000
			.saturating_add(Weight::from_ref_time(3_894_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn delegate(r: u32) -> Weight {
		Weight::from_ref_time(37_776_000 as u64) // Standard Error: 24_000
			.saturating_add(Weight::from_ref_time(5_068_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	fn undelegate(r: u32) -> Weight {
		Weight::from_ref_time(19_177_000 as u64) // Standard Error: 15_000
			.saturating_add(Weight::from_ref_time(5_034_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(r as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(r as u64)))
	}
	fn clear_public_proposals() -> Weight {
		Weight::from_ref_time(2_178_000 as u64).saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn note_preimage(b: u32) -> Weight {
		Weight::from_ref_time(24_989_000 as u64) // Standard Error: 0
			.saturating_add(Weight::from_ref_time(3_000 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn note_imminent_preimage(b: u32) -> Weight {
		Weight::from_ref_time(21_119_000 as u64) // Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn reap_preimage(b: u32) -> Weight {
		Weight::from_ref_time(27_541_000 as u64) // Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn unlock_remove(r: u32) -> Weight {
		Weight::from_ref_time(28_169_000 as u64) // Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(59_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn unlock_set(r: u32) -> Weight {
		Weight::from_ref_time(26_010_000 as u64) // Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(166_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	fn remove_vote(r: u32) -> Weight {
		Weight::from_ref_time(14_613_000 as u64) // Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(123_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	fn remove_other_vote(r: u32) -> Weight {
		Weight::from_ref_time(15_066_000 as u64) // Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(127_000 as u64).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}
