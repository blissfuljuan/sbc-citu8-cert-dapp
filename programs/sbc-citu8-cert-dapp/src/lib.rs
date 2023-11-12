use anchor_lang::prelude::*;

declare_id!("uwWiGs51F6LeLHd84pLCTALydnyYJCT1kKmxmBVv7EV");

#[program]
pub mod sbc_citu8_cert_dapp {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
