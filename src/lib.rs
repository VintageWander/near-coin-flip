use near_sdk::env::log_str;
use near_sdk::store::LookupMap;
use near_sdk::{env, near, AccountId, BorshStorageKey};

#[near(serializers = [borsh])]
#[derive(BorshStorageKey)]
enum StorageKey {
	Points,
}

#[near(contract_state)]
pub struct Contract {
	points: LookupMap<AccountId, u8>,
}

impl Default for Contract {
	fn default() -> Self {
		Self {
			points: LookupMap::new(StorageKey::Points),
		}
	}
}

#[near]
impl Contract {
	pub fn guess_coin_flip(&mut self, player_guess: String) -> String {
		let player = env::predecessor_account_id();
		log_str(&format!("{player} chose {player_guess}"));

		// Flip coin
		let coin_state: String = {
			let random_seed = *env::random_seed().first().unwrap() as i8;
			if let 0 = random_seed % 2 {
				"heads"
			} else {
				"tails"
			}
			.into()
		};

		let mut player_points = *self.points.get(&player).unwrap_or(&0);

		if coin_state.eq(&player_guess) {
			player_points += 1
		} else {
			player_points = player_points.saturating_sub(1u8);
		}

		log_str(&format!("Player points: {player_points}"));

		// Store new points
		self.points.insert(player, player_points);

		coin_state
	}

	pub fn points_of(&self, player: AccountId) -> u8 {
		let points: u8 = *self.points.get(&player).unwrap_or(&0);
		log_str(&format!("Points for player {player}: {points}"));
		points
	}
}
