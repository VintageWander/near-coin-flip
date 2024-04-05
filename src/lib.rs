use near_sdk::{AccountId, BorshStorageKey, near};
use near_sdk::env;
use near_sdk::env::log_str;
use near_sdk::store::LookupMap;

#[near(serializers = [borsh])]
#[derive(BorshStorageKey)]
enum StorageKey {
	Points
}

pub fn coin_flip() -> String {
	let random_seed = *env::random_seed().first().expect("Cannot get random seed");
	if random_seed % 2 == 0 { "heads" } else { "tails" }.into()
}

// Contract structure
#[near(contract_state)]
pub struct Contract {
	points: LookupMap<AccountId, u8>,
}

impl Default for Contract {
	fn default() -> Self {
		Self {
			points: LookupMap::new(StorageKey::Points)
		}
	}
}

#[near]
impl Contract {
	pub fn guess_coin_flip(&mut self, player_guess: String) -> String {
		let player = env::predecessor_account_id();
		log_str(&format!("{player} chose {player_guess}"));
		
		// Flip the coin
		let coin_state = coin_flip();
		
		let mut player_points = *self.points.get(&player).expect("Cannot find player");
		
		if coin_state.eq(&player_guess) {
			player_points += 1;
		} else {
			player_points = player_points.saturating_sub(1);
		}
		
		log_str(&format!("Player points: {player_points}"));
		
		// Store new points
		self.points.insert(player, player_points);
		
		coin_state
	}
	
	// View points that a user has
	pub fn points_of(&self, player: AccountId) -> u8 {
		let points = *self.points.get(&player).expect("Cannot get player points");
		log_str(&format!("Points for {player}: {points}"));
		points
	}
}