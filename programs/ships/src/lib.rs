pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5eEyF9ibsMmiXwEL3z2CNmyn7oeVZQnmHUMkbvM1VdpD");

#[program]
pub mod ships {
    use anchor_lang::solana_program::blake3::hashv;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // hashv(vals)
        initialize::handler(ctx)
    }
}
