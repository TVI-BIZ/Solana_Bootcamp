//use anchor_lang::{prelude::*, solana_program::account_info::Account};
use anchor_lang::prelude::*;

declare_id!("62wpCWKSX87fLgkmeesnKspYctLnSvAzcfSjUgt6wtXe");

#[program]
pub mod notes {
    use super::*;

    pub fn create_note(ctx: Context<CreateNote>, content: String) -> Result<()> {
        let note = &mut ctx.accounts.note;
        let user = &ctx.accounts.user;

        note.content = content;
        note.user = user.key();

        Ok(())
    }
    pub fn delete_note(_ctx: Context<DeleteNote>) -> Result<()> {   
        Ok(())
    }
    // pub fn update_note(_ctx: Context<UpdateNote>) -> Result<()> {   
    //     Ok(())
    // }


}

#[derive(Accounts)]
pub struct CreateNote<'info> {
    #[account{
        init,
        payer = user,
        space = 2000
    }]
    pub note: Account<'info, Note>,

    #[account(mut)]
    pub user: Signer<'info>,    

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeleteNote<'info> {
    #[account{
        mut,
        has_one = user,
        close = user
    }]
    pub note: Account<'info, Note>,

    #[account(mut)]
    pub user: Signer<'info>,    

}
// #[derive(Accounts)]
// pub struct UpdateNote<'info> {
//     #[account{
//         mut,
//         has_one = user
//     }]
//     pub note: Account<'info, Note>,     

//     #[account(mut)]
//     pub user: Signer<'info>,

// }



#[account]
pub struct Note {
    pub content: String,
    pub user: Pubkey,

}
