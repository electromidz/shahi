use std::collections::HashMap;
use rand::Rng;

pub type Address = String;

pub struct FlipCoinGame {
    pub pot: u64,
    pub bets: HashMap<Address, u64>,
    pub players: Vec<Address>,
    pub token_balances: HashMap<Address, u64>,
}

impl FlipCoinGame {
    pub fn new() -> Self {
        Self {
            pot: 0,
            bets: HashMap::new(),
            players: vec![],
            token_balances: HashMap::new(),
        }
    }

    pub fn deposit_tokens(&mut self, player: Address, amount: u64) {
        let entry = self.token_balances.entry(player).or_insert(0);
        *entry += amount;
    }

    pub fn place_bet(&mut self, player: Address, amount: u64) -> Result<(), String> {
        let balance = self.token_balances.entry(player.clone()).or_insert(0);
        if *balance < amount {
            return Err("Insufficient balance".into());
        }

        *balance -= amount;
        self.pot += amount;
        self.bets.insert(player.clone(), amount);
        self.players.push(player);
        Ok(())
    }

    pub fn flip_coin(&mut self) -> Result<(Address, u64), String> {
        if self.players.len() != 2 {
            return Err("Need exactly 2 players".into());
        }

        let winner_index = rand::thread_rng().gen_range(0..2);
        let winner = self.players[winner_index].clone();
        let prize = self.pot;

        self.token_balances
            .entry(winner.clone())
            .and_modify(|bal| *bal += prize)
            .or_insert(prize);

        self.reset_game();
        Ok((winner, prize))
    }

    fn reset_game(&mut self) {
        self.pot = 0;
        self.bets.clear();
        self.players.clear();
    }

    pub fn get_balance(&self, player: &Address) -> u64 {
        *self.token_balances.get(player).unwrap_or(&0)
    }
}