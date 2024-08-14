use anchor_lang::prelude::*;

declare_id!("BfAmwtCMs8otQ72caxXmuomDMY2Ni2dfYDXfQq2nqxKf");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
