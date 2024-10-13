use anchor_lang::prelude::*;
use anchor_lang::system_program::System; // remove this if error throws

// This is your program's public key and it will update
// automatically when you build the project.

declare_id!("5g4MFuXhq8evZRDgxo1WZLbpX9Ex4MXitYMQ3TCZ2F4U");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod fav_prog {
    use super::*;

    pub fn set_favorites(
        context: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        let user_public_key = context.accounts.user.key;

        msg!("Greetings from {}", context.program_id);

        msg!("User {user_public_key}'s favorite number is {number}, favorite color is: {color}",);

        msg!("User's hobbies are: {:?}", hobbies);

        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init_if_needed, payer = user, space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE, seeds = [b"favorites", user.key().as_ref()], bump)]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}
