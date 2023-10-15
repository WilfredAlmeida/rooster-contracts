use anchor_lang::prelude::*;
use puppet::cpi::accounts::SetData;
use puppet::program::Puppet;
use puppet::{self, Data};


declare_id!("FwsAUMA78s66U2LSrEmRWeSP4zdqc3yuz1gH1zndZ6cq");


#[program]
mod puppet_master {
    use super::*;
    pub fn pull_strings(ctx: Context<PullStrings>, _data: u64) -> Result<()> {

        let label: String = "test".to_string();
        let data: String = "{title:'Hello HH BLR', msg:'Hire Me!!!'}".to_string();

        let _ = puppet::cpi::emit_event(ctx.accounts.set_data_ctx(), label, data);
        
        Ok(())
    }
}


#[derive(Accounts)]
pub struct PullStrings<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>,
    pub puppet_program: Program<'info, Puppet>,
}


impl<'info> PullStrings<'info> {
    pub fn set_data_ctx(&self) -> CpiContext<'_, '_, '_, 'info, SetData<'info>> {
        let cpi_program = self.puppet_program.to_account_info();
        let cpi_accounts = SetData {
            puppet: self.puppet.to_account_info()
        };
        CpiContext::new(cpi_program, cpi_accounts)
    }
}