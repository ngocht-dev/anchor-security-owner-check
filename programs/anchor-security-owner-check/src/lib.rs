use anchor_lang::prelude::*;

declare_id!("3KcUTc3emDjXxe5M3u5zHXjNe4GxaVBrnifEsnioCPXr");

#[program]
pub mod anchor_security_owner_check {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
