module MyProject::CoinFlip {
    use std::signer;
    use std::vector;
    use aptos_std::type_info;
    use aptos_framework::coin;
    use aptos_framework::account;

    // Error codes
    const ENOT_ENOUGH_BALANCE: u64 = 1;
    const EINVALID_BET_AMOUNT: u64 = 2;
    const ETRANSFER_FAILED: u64 = 3;

    // Game outcomes
    const OUTCOME_HEADS: u64 = 0;
    const OUTCOME_TAILS: u64 = 1;

    struct BettingCoin has key {
        value: u64
    }

    struct GameResult has drop, store {
        player: address,
        bet_amount: u64,
        player_choice: u64,
        outcome: u64,
        won: bool,
        payout: u64
    }

    public entry fun initialize(account: &signer) {
        let account_addr = signer::address_of(account);
        if (!exists<BettingCoin>(account_addr)) {
            move_to(account, BettingCoin { value: 1000000 }); // Initial house bankroll
        }
    }

    /// Play a coin flip game
    /// choice: 0 for heads, 1 for tails
    public entry fun play(
        player: &signer,
        bet_amount: u64,
        choice: u64
    ) {
        // Validate inputs
        assert!(choice == OUTCOME_HEADS || choice == OUTCOME_TAILS, EINVALID_BET_AMOUNT);
        assert!(bet_amount > 0, EINVALID_BET_AMOUNT);

        let player_addr = signer::address_of(player);
        let house_addr = @MyProject;

        // Check player balance
        let player_balance = coin::balance<CoinType>(player_addr);
        assert!(player_balance >= bet_amount, ENOT_ENOUGH_BALANCE);

        // Check house balance
        let house_balance = borrow_global<BettingCoin>(house_addr).value;
        assert!(house_balance >= bet_amount * 2, ENOT_ENOUGH_BALANCE); // House needs to cover potential payout

        // Transfer bet amount from player to house
        coin::transfer<CoinType>(player, house_addr, bet_amount);

        // Generate random outcome (in a real contract, you'd use a proper randomness source)
        let outcome = generate_random_outcome(player_addr);

        // Calculate results
        let won = if (choice == outcome) { true } else { false };
        let payout = if (won) { bet_amount * 2 } else { 0 };

        // Process payout if player won
        if (won) {
            let house = borrow_global_mut<BettingCoin>(house_addr);
            house.value = house.value - payout;
            coin::transfer<CoinType>(&account::create_signer(house_addr), player_addr, payout);
        } else {
            let house = borrow_global_mut<BettingCoin>(house_addr);
            house.value = house.value + bet_amount;
        }

        // Emit game result event
        let game_result = GameResult {
            player: player_addr,
            bet_amount,
            player_choice: choice,
            outcome,
            won,
            payout
        };
        event::emit(game_result);
    }

    fun generate_random_outcome(player_addr: address): u64 {
        // In a real implementation, you would use a proper randomness source
        // This is a simple placeholder that's not truly random
        let addr_bytes = *&player_addr;
        let last_byte = vector::borrow(&addr_bytes, vector::length(&addr_bytes) - 1);
        if (*last_byte % 2 == 0) {
            OUTCOME_HEADS
        } else {
            OUTCOME_TAILS
        }
    }
}
