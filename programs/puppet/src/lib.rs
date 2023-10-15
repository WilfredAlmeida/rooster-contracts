use anchor_lang::prelude::*;


declare_id!("BdTgodfbn4PBz5zgj2EoGU6b7owYWXY17y6ZK4DM12qR");


#[program]
pub mod puppet {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn emit_event(_ctx: Context<SetData>, label: String, data: String) -> Result<()> {

        let json_string = format!(
            r#"{{"label":"{}","data":"{}"}}"#,
            label, data
        );

        emit!(RoosterEvent{
            data: json_string
        });

        msg!("Event Emitted");

        Ok(())
    }

}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub puppet: Account<'info, Data>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>,
}


#[account]
pub struct Data {
    pub data: u64,
}

#[event]
pub struct RoosterEvent{
    // pub label: String,
    pub data: String
}