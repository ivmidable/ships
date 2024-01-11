use anchor_lang::prelude::*;

#[account]
pub struct Board {
    pub ships: Vec<Vec<u8>>,
    //0 for not hits, 1 if missed, and 2 if hit
    pub hits: Vec<Vec<u8>>,
    pub player1: Pubkey,
    pub player2: Pubkey,
    pub player1_turn: bool,
}

#[account]
pub struct Match {
    pub bet_amount: u64,
    pub player_1_escrow: Pubkey,
    pub player_2_escrow: Pubkey,
    pub match_mint: Pubkey,
}

#[account]
pub struct player {}
