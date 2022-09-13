use anchor_lang::prelude::*;

declare_id!("C6p2jQDXrvbAsdEoebKwPGw5WM82jXBDpMbeh43Zobac");


#[program]
pub mod solana_twitter_app {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}


#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
}
const DISCRIMINATOR_LENGTH : usize = 8;
const PUBLIC_KEY_LENGTH : usize = 32;
