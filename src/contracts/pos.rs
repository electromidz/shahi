// use std::collections::HashMap;
//
// #[derive(Debug, Clone)]
// pub struct Validator {
//     pub address: String,
//     pub stake: u64, // Amount of tokens staked
// }
//
// #[derive(Debug)]
// pub struct ProofOfStake {
//     pub validators: HashMap<String, Validator>, // Address -> Validator
// }
//
// impl ProofOfStake {
//     pub fn new() -> Self {
//         ProofOfStake {
//             validators: HashMap::new(),
//         }
//     }
//
//     // Add stake to a validator
//     pub fn stake(&mut self, address: String, amount: u64) {
//         let entry = self
//             .validators
//             .entry(address.clone())
//             .or_insert(Validator { address, stake: 0 });
//         entry.stake += amount;
//     }
//
//     // Choose a validator based on stake weight
//     pub fn select_validator(&self) -> Option<&Validator> {
//         if self.validators.is_empty() {
//             return None;
//         }
//
//         // Create a weighted list of validators
//         let mut weighted_list = Vec::new();
//         for validator in self.validators.values() {
//             for _ in 0..validator.stake {
//                 weighted_list.push(validator);
//             }
//         }
//
//         // Randomly select a validator based on stake weight
//         let selected = weighted_list.choose(&mut rnd::thread_rng());
//         selected.cloned()
//     }
// }
