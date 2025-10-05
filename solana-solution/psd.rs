// use anchor_lang::prelude::*;

// declare_id!("BZWJgmW8XFhHpCY6RPK1UkAVPCJGmoc5pcYq6uhT18me"); // temporary, will be replaced after deploy

// #[program]
// pub mod authevo_identity {
//     use super::*;

//     // Create a new identity account
//     pub fn create_identity(ctx: Context<CreateIdentity>, pubkey: Pubkey) -> Result<()> {
//         let identity = &mut ctx.accounts.identity;
//         identity.owner = pubkey;
//         identity.status = true; // active
//         identity.version = 1;
//         identity.bump = ctx.bumps.identity;
//         identity.created_at = Clock::get()?.unix_timestamp;
//         Ok(())
//     }

//     // Rotate identity
//     pub fn rotate_identity(ctx: Context<UpdateIdentity>, new_pubkey: Pubkey) -> Result<()> {
//         let identity = &mut ctx.accounts.identity;
//         require!(identity.status == true, IdentityError::Revoked);
//         identity.owner = new_pubkey;
//         identity.version = identity.version.checked_add(1).unwrap();
//         identity.last_rotation_at = Some(Clock::get()?.unix_timestamp);
//         Ok(())
//     }

//     // Revoke identity
//     pub fn revoke_identity(ctx: Context<UpdateIdentity>) -> Result<()> {
//         let identity = &mut ctx.accounts.identity;
//         identity.status = false;
//         Ok(())
//     }
// }

// #[account]
// pub struct IdentityAccount {
//     pub owner: Pubkey,      // 32
//     pub status: bool,       // 1
//     pub version: u64,       // 8
//     pub bump: u8,           // 1
//     pub created_at: i64,    // 8
//     pub last_rotation_at: Option<i64>, // 9
// }

// #[derive(Accounts)]
// #[instruction(pubkey: Pubkey)]
// pub struct CreateIdentity<'info> {
//     #[account(
//         init,
//         payer = user,
//         space = 8 + 64,
//         seeds = [b"identity", user.key.as_ref()],
//         bump
//     )]
//     pub identity: Account<'info, IdentityAccount>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct UpdateIdentity<'info> {
//     #[account(mut, seeds = [b"identity", user.key.as_ref()], bump = identity.bump)]
//     pub identity: Account<'info, IdentityAccount>,
//     pub user: Signer<'info>,
// }

// #[error_code]
// pub enum IdentityError {
//     #[msg("This identity has been revoked.")]
//     Revoked,
// }