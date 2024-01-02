// This file is part of CORD – https://cord.network

// Copyright (C) Dhiway Networks Pvt. Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// CORD is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// CORD is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with CORD. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_chain_space`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-01-02, STEPS: `5`, REPEAT: `2`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `pi.lan`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/cord
// benchmark
// pallet
// --chain=dev
// --steps=5
// --repeat=2
// --pallet=pallet_chain_space
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/chain-space/src/weights.rs
// --header=./HEADER-GPL3
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_chain_space`.
pub trait WeightInfo {
	fn add_delegate() -> Weight;
	fn add_admin_delegate() -> Weight;
	fn add_delegator() -> Weight;
	fn remove_delegate() -> Weight;
	fn create() -> Weight;
	fn approve() -> Weight;
	fn archive() -> Weight;
	fn restore() -> Weight;
	fn update_transaction_capacity() -> Weight;
	fn reset_transaction_count() -> Weight;
	fn approval_revoke() -> Weight;
	fn approval_restore() -> Weight;
}

/// Weights for `pallet_chain_space` using the CORD node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `ChainSpace::Authorizations` (r:2 w:1)
	/// Proof: `ChainSpace::Authorizations` (`max_values`: None, `max_size`: Some(184), added: 2659, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Delegates` (r:1 w:1)
	/// Proof: `ChainSpace::Delegates` (`max_values`: None, `max_size`: Some(320068), added: 322543, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn add_delegate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `746`
		//  Estimated: `323533`
		// Minimum execution time: 36_000_000 picoseconds.
		Weight::from_parts(37_000_000, 323533)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `ChainSpace::Authorizations` (r:2 w:1)
	/// Proof: `ChainSpace::Authorizations` (`max_values`: None, `max_size`: Some(184), added: 2659, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Delegates` (r:1 w:1)
	/// Proof: `ChainSpace::Delegates` (`max_values`: None, `max_size`: Some(320068), added: 322543, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn add_admin_delegate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `746`
		//  Estimated: `323533`
		// Minimum execution time: 35_000_000 picoseconds.
		Weight::from_parts(36_000_000, 323533)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `ChainSpace::Authorizations` (r:2 w:1)
	/// Proof: `ChainSpace::Authorizations` (`max_values`: None, `max_size`: Some(184), added: 2659, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Delegates` (r:1 w:1)
	/// Proof: `ChainSpace::Delegates` (`max_values`: None, `max_size`: Some(320068), added: 322543, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn add_delegator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `746`
		//  Estimated: `323533`
		// Minimum execution time: 35_000_000 picoseconds.
		Weight::from_parts(36_000_000, 323533)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `ChainSpace::Authorizations` (r:2 w:1)
	/// Proof: `ChainSpace::Authorizations` (`max_values`: None, `max_size`: Some(184), added: 2659, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Delegates` (r:1 w:1)
	/// Proof: `ChainSpace::Delegates` (`max_values`: None, `max_size`: Some(320068), added: 322543, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn remove_delegate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1003`
		//  Estimated: `323533`
		// Minimum execution time: 34_000_000 picoseconds.
		Weight::from_parts(34_000_000, 323533)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Delegates` (r:0 w:1)
	/// Proof: `ChainSpace::Delegates` (`max_values`: None, `max_size`: Some(320068), added: 322543, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Authorizations` (r:0 w:1)
	/// Proof: `ChainSpace::Authorizations` (`max_values`: None, `max_size`: Some(184), added: 2659, mode: `MaxEncodedLen`)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115`
		//  Estimated: `3613`
		// Minimum execution time: 26_000_000 picoseconds.
		Weight::from_parts(26_000_000, 3613)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn approve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `436`
		//  Estimated: `3613`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(20_000_000, 3613)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ChainSpace::Authorizations` (r:1 w:0)
	/// Proof: `ChainSpace::Authorizations` (`max_values`: None, `max_size`: Some(184), added: 2659, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn archive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `657`
		//  Estimated: `3649`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(26_000_000, 3649)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ChainSpace::Authorizations` (r:1 w:0)
	/// Proof: `ChainSpace::Authorizations` (`max_values`: None, `max_size`: Some(184), added: 2659, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn restore() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `666`
		//  Estimated: `3649`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(26_000_000, 3649)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn update_transaction_capacity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `445`
		//  Estimated: `3613`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(19_000_000, 3613)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn reset_transaction_count() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `445`
		//  Estimated: `3613`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(21_000_000, 3613)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn approval_revoke() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `445`
		//  Estimated: `3613`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(19_000_000, 3613)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn approval_restore() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `454`
		//  Estimated: `3613`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(18_000_000, 3613)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `ChainSpace::Authorizations` (r:2 w:1)
	/// Proof: `ChainSpace::Authorizations` (`max_values`: None, `max_size`: Some(184), added: 2659, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Delegates` (r:1 w:1)
	/// Proof: `ChainSpace::Delegates` (`max_values`: None, `max_size`: Some(320068), added: 322543, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn add_delegate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `746`
		//  Estimated: `323533`
		// Minimum execution time: 36_000_000 picoseconds.
		Weight::from_parts(37_000_000, 323533)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `ChainSpace::Authorizations` (r:2 w:1)
	/// Proof: `ChainSpace::Authorizations` (`max_values`: None, `max_size`: Some(184), added: 2659, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Delegates` (r:1 w:1)
	/// Proof: `ChainSpace::Delegates` (`max_values`: None, `max_size`: Some(320068), added: 322543, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn add_admin_delegate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `746`
		//  Estimated: `323533`
		// Minimum execution time: 35_000_000 picoseconds.
		Weight::from_parts(36_000_000, 323533)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `ChainSpace::Authorizations` (r:2 w:1)
	/// Proof: `ChainSpace::Authorizations` (`max_values`: None, `max_size`: Some(184), added: 2659, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Delegates` (r:1 w:1)
	/// Proof: `ChainSpace::Delegates` (`max_values`: None, `max_size`: Some(320068), added: 322543, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn add_delegator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `746`
		//  Estimated: `323533`
		// Minimum execution time: 35_000_000 picoseconds.
		Weight::from_parts(36_000_000, 323533)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `ChainSpace::Authorizations` (r:2 w:1)
	/// Proof: `ChainSpace::Authorizations` (`max_values`: None, `max_size`: Some(184), added: 2659, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Delegates` (r:1 w:1)
	/// Proof: `ChainSpace::Delegates` (`max_values`: None, `max_size`: Some(320068), added: 322543, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn remove_delegate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1003`
		//  Estimated: `323533`
		// Minimum execution time: 34_000_000 picoseconds.
		Weight::from_parts(34_000_000, 323533)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Delegates` (r:0 w:1)
	/// Proof: `ChainSpace::Delegates` (`max_values`: None, `max_size`: Some(320068), added: 322543, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Authorizations` (r:0 w:1)
	/// Proof: `ChainSpace::Authorizations` (`max_values`: None, `max_size`: Some(184), added: 2659, mode: `MaxEncodedLen`)
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115`
		//  Estimated: `3613`
		// Minimum execution time: 26_000_000 picoseconds.
		Weight::from_parts(26_000_000, 3613)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn approve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `436`
		//  Estimated: `3613`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(20_000_000, 3613)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `ChainSpace::Authorizations` (r:1 w:0)
	/// Proof: `ChainSpace::Authorizations` (`max_values`: None, `max_size`: Some(184), added: 2659, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn archive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `657`
		//  Estimated: `3649`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(26_000_000, 3649)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `ChainSpace::Authorizations` (r:1 w:0)
	/// Proof: `ChainSpace::Authorizations` (`max_values`: None, `max_size`: Some(184), added: 2659, mode: `MaxEncodedLen`)
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn restore() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `666`
		//  Estimated: `3649`
		// Minimum execution time: 25_000_000 picoseconds.
		Weight::from_parts(26_000_000, 3649)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn update_transaction_capacity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `445`
		//  Estimated: `3613`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(19_000_000, 3613)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn reset_transaction_count() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `445`
		//  Estimated: `3613`
		// Minimum execution time: 19_000_000 picoseconds.
		Weight::from_parts(21_000_000, 3613)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn approval_revoke() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `445`
		//  Estimated: `3613`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(19_000_000, 3613)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `ChainSpace::Spaces` (r:1 w:1)
	/// Proof: `ChainSpace::Spaces` (`max_values`: None, `max_size`: Some(148), added: 2623, mode: `MaxEncodedLen`)
	/// Storage: `Identifier::Identifiers` (r:1 w:1)
	/// Proof: `Identifier::Identifiers` (`max_values`: None, `max_size`: Some(4294967295), added: 2474, mode: `MaxEncodedLen`)
	fn approval_restore() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `454`
		//  Estimated: `3613`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(18_000_000, 3613)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
