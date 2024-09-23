// Copyright 2020-2024 Trust Computing GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

bitflags! {
	/// Flags used to record the status of pool proposal
	#[derive(Encode, Decode, MaxEncodedLen)]
	pub struct ProposalStatusFlags: u8 {
		/// Whether the pool proposal passing the committee/democracy voting.
		///
		/// # Note
		///
		/// A valid pool must passing committee/public's audit procedure regarding legal files and other pool parameters.
		const PUBLIC_VOTE_PASSED = 0b0000_0001;
		/// Whether the minimum staked amount proposed by curator is satisfied.
		///
		/// # Note
		///
		/// Currently, a full size must be satisfied.
		///
		/// Once a pool is satisfied this requirement, all staked amount can no longer be withdrawed
		/// unless the pool is later denied passing by voting or until the end of pool maturity.
		///
		/// Otherwise, the pool will be refunded.
		const STAKE_AMOUNT_PASSED = 0b0000_0010;
		/// Whether the pool guardian has been selected
		///
		/// # Note
		///
		/// A valid pool must have guardian or a default one will be used (committee)
		const GUARDIAN_SELECTED = 0b0000_0100;
		/// Whether the proposal expired yet
		///
		/// # Note
		///
		/// Has nothing to do with pool. Only related to proposal expired time
		const PROPOSAL_EXPIRED = 0b0000_1000;
	}
}

#[derive(PartialEq, Eq, Clone, Encode, Debug, Decode, TypeInfo)]
pub struct PoolProposalStatus<BlockNumber> {
	pub pool_proposal_index: PoolProposalIndex,
	pub proposal_expire_time: BlockNumber,
}

#[derive(PartialEq, Eq, Clone, Encode, Debug, Decode, TypeInfo)]
pub struct PoolProposalInfo<InfoHash, Balance, BlockNumber, AccountId> {
	// Proposer/Curator
	pub proposer: AccountId,
	// Hash of pool info like legal files etc.
	pub pool_info_hash: InfoHash,
	// The maximum staking amount that the pool can handle
	pub max_pool_size: Balance,
	// If proposal passed, when the staking pool will start
	pub pool_start_time: BlockNumber,
	// If proposal passed, when the staking pool will end
	pub pool_end_time: BlockNumber,
	// estimated APR, but in percentage form
	// i.e. 100 => 100%
	pub estimated_epoch_reward: Balance,
	// Proposal status flags
	pub proposal_status_flags: ProposalStatusFlags,
}

#[derive(Clone, Encode, Debug, Decode, TypeInfo)]
pub struct Bond<Identity, BalanceType> {
	pub owner: Identity,
	pub amount: BalanceType,
}

impl<A: Decode, B: Default> Default for Bond<A, B> {
	fn default() -> Bond<A, B> {
		Bond {
			owner: A::decode(&mut sp_runtime::traits::TrailingZeroInput::zeroes())
				.expect("infinite length input; no invalid inputs for type; qed"),
			amount: B::default(),
		}
	}
}

impl<A, B: Default> Bond<A, B> {
	pub fn from_owner(owner: A) -> Self {
		Bond { owner, amount: B::default() }
	}
}

impl<Identity: Ord, Balance> Eq for Bond<Identity, Balance> {}

impl<Identity: Ord, Balance> Ord for Bond<Identity, Balance> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.owner.cmp(&other.owner)
	}
}

impl<Identity: Ord, Balance> PartialOrd for Bond<Identity, Balance> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl<Identity: Ord, Balance> PartialEq for Bond<Identity, Balance> {
	fn eq(&self, other: &Self) -> bool {
		self.owner == other.owner
	}
}

#[derive(Clone, Encode, Debug, Decode, TypeInfo)]
pub struct PoolProposalPreStaking<AccountId, Balance, BlockNumber, S: Get<u32>> {
	pub total_pre_staked_amount: Balance,
	// Ordered by bond owner AccountId
	pub pre_stakings: BoundedVec<Bond<AccountId, Balance>, S>,
	pub total_queued_amount: Balance,
	// Ordered by bond owner AccountId
	pub queued_pre_stakings: BoundedVec<(Bond<AccountId, Balance>, BlockNumber), S>,
}

impl<AccountId, Balance: Default + CheckedAdd, S: Get<u32>>
	PoolProposalPreStaking<AccountId, Balance, BlockNumber, S>
{
	/// Create a new empty default
	pub fn new() -> Self {
		PoolProposalPreStaking {
			total_pre_staked_amount: Default::default(),
			pre_stakings: Default::default(),
			total_queued_amount: Default::default(),
			queued_pre_stakings: Default::default(),
		}
	}

	pub fn get_pre_staking(&self, account: AccountId) -> Option<(usize, Balance)> {
		match self.pre_stakings.binary_search(&Bond::from_owner(account)) {
			Ok(loc) => Some((loc, self.pre_stakings.index(loc))),
			Err(loc) => None,
		}
	}

	pub fn add_pre_staking<T: Config>(
		&mut self,
		account: AccountId,
		amount: Balance,
	) -> Result<(), DispatchError> {
		if let Some(existing) = self.get_pre_staking(account) {
			self.pre_stakings.remove(existing.0);
			let new_balance = existing.1.checked_add(&amount).ok_or(ArithmeticError::Overflow)?;
			let _ = self
				.pre_stakings
				.try_insert(existing.0, Bond { owner: account, amount: new_balance })
				.map_err(|_| Error::<T>::StakingPoolOversized)?;
		} else {
			let _ = self
				.pre_stakings
				.try_insert(existing.0, Bond { owner: account, amount })
				.map_err(|_| Error::<T>::StakingPoolOversized)?;
		}
		self::total_pre_staked_amount = self::total_pre_staked_amount
			.checked_add(&amount)
			.ok_or(ArithmeticError::Overflow)?;
		Ok(())
	}

	pub fn withdraw<T: Config>(
		&mut self,
		account: AccountId,
		amount: Balance,
	) -> Result<(), DispatchError> {
		// Withdraw Queued one if any
		if let Some(existing_q) = self.get_queued_staking(account) {
			if (existing_q.1 > amount) {
				// Existing queue is larger than target amount
				// Finish withdrawing and return early
				self.queued_pre_stakings.remove(existing_q.0);
				let new_balance_q =
					existing_q.1.checked_sub(&amount).ok_or(ArithmeticError::Overflow)?;
				self.queued_pre_stakings
					.try_insert(
						existing_q.0,
						(Bond { owner: account, amount: new_balance_q }, existing_q.2),
					)
					.map_err(|_| Error::<T>::StakingPoolOversized)?;

				self::total_queued_amount = self::total_queued_amount
					.checked_sub(&amount)
					.ok_or(ArithmeticError::Overflow)?;
				return Ok(());
			} else {
				// Totally remove queued
				self.queued_pre_stakings.remove(existing_q.0);
				self::total_queued_amount = self::total_queued_amount
					.checked_sub(&existing_q.1)
					.ok_or(ArithmeticError::Overflow)?;

				let left_amount = amount - existing_q.1;

				if let Some(existing_p) = self.get_pre_staking(account) {
					// Existing pre-staking is larger than left target amount
					// Finish withdrawing and return early
					if (existing_p.1 > left_amount) {
						self.pre_stakings.remove(existing_p.0);
						let new_balance_p = existing_p
							.1
							.checked_sub(&left_amount)
							.ok_or(ArithmeticError::Overflow)?;
						self.pre_stakings
							.try_insert(
								existing_q.0,
								Bond { owner: account, amount: new_balance_p },
							)
							.map_err(|_| Error::<T>::StakingPoolOversized)?;
						self::total_pre_staked_amount = self::total_pre_staked_amount
							.checked_sub(&left_amount)
							.ok_or(ArithmeticError::Overflow)?;
						return Ok(());
					} else if (existing_p.1 == left_amount) {
						// Exact amount to finish everything
						self.pre_stakings.remove(existing_p.0);
						self::total_pre_staked_amount = self::total_pre_staked_amount
							.checked_sub(&left_amount)
							.ok_or(ArithmeticError::Overflow)?;
						return Ok(());
					} else {
						// Not enough fund to finish operation
						return Err(Error::<T>::InsufficientPreStaking);
					}
				}
			}
		}
		// No pre-staking of all kinds
		return Err(Error::<T>::InsufficientPreStaking);
	}

	pub fn get_queued_staking(&self, account: AccountId) -> Option<(usize, Balance, BlockNumber)> {
		match self
			.queued_pre_stakings
			.binary_search_by(|p| p.0.cmp(&Bond::from_owner(account)))
		{
			Ok(loc) => Some((
				loc,
				self.queued_pre_stakings.index(loc).0.amount,
				self.queued_pre_stakings.index(loc).1,
			)),
			Err(loc) => None,
		}
	}

	pub fn add_queued_staking<T: Config>(
		&mut self,
		account: AccountId,
		amount: Balance,
		current_block: BlockNumber,
	) -> Result<(), DispatchError> {
		if let Some(existing) = self.get_queued_staking(account) {
			self.queued_pre_stakings.remove(existing.0);
			let new_balance = existing.1.checked_add(&amount).ok_or(ArithmeticError::Overflow)?;
			let _ = self
				.queued_pre_stakings
				.try_insert(
					existing.0,
					(Bond { owner: account, amount: new_balance }, current_block),
				)
				.map_err(|_| Error::<T>::StakingPoolOversized)?;
		} else {
			let _ = self
				.queued_pre_stakings
				.try_insert(existing.0, (Bond { owner: account, amount }, current_block))
				.map_err(|_| Error::<T>::StakingPoolOversized)?;
		}
		self::total_queued_amount = self::total_queued_amount
			.checked_add(&amount)
			.ok_or(ArithmeticError::Overflow)?;
		Ok(())
	}

	// Transfer queued amount into pre staking
	pub fn move_queued_to_pre_staking_until<T: Config>(
		&mut self,
		target_pre_staked_amount: Balance,
	) -> Result<Vec<Bond<AccountId, Balance>>, DispatchError> {
		let result: Vec<Bond<AccountId, Balance>> = Vec::new();
		// Make sure target transfer is possible
		ensure!(
			self.total_queued_amount
				>= target_pre_staked_amount
					.checked_sub(self.total_pre_staked_amount)
					.ok_or(ArithmeticError::Overflow)?,
			Error::<T>::InsufficientPreStaking
		);

		let mut v = self.queued_pre_stakings.into_inner().clone();
		// temp sorted by blocknumber
		v.sort_by(|p| p.2);

		for i in v.iter() {
			let transfer_amount = target_pre_staked_amount
				.checked_sub(self.total_pre_staked_amount)
				.ok_or(ArithmeticError::Overflow)?;
			if (i.0.amount >= transfer_amount) {
				let _ = self.withdraw(i.0.owner, transfer_amount)?;
				self.add_pre_staking(i.0.owner, transfer_amount)?;
				result.push(Bond { owner: i.0.owner, amount: transfer_amount });
				break;
			} else {
				let _ = self.withdraw(i.0.owner, i.0.amount)?;
				self.add_pre_staking(i.0.owner, i.0.amount)?;
				result.push(Bond { owner: i.0.owner, amount: i.0.amount });
			}
		}

		Ok(result)
	}
}
