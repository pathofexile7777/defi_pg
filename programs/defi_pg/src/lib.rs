use anchor_lang::prelude::*;

declare_id!("6uEtFvj1EtCD3oZvAhCR4wW3j1TQv2CaxgMoXyfksqaC");

#[program]
pub mod defi_pg {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {
    

}
