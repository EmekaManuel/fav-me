use anchor_lang::prelude::*;
use anchor_lang::system_program::System;

declare_id!("FhnC7vwL3VV6n5kZn85GmRo1k5NeXLLRaHkUYzMh9eXJ");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod fav_me {
    use super::*;

    pub fn set_favorites(
        context: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greetings from {}", context.program_id);
        let user_public_key = context.accounts.user.key;

        // Log the user's favorite color and hobbies
        msg!("User {}'s favorite color is {}", user_public_key, color);
        msg!(
            "User {}'s favorite hobbies are: {:?}",
            user_public_key,
            hobbies
        );

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

    #[account(init_if_needed, payer = user, space = ANCHOR_DISCRIMINATOR_SIZE, seeds = [b"favorites", user.key().as_ref()], bump)]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}
