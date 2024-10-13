use anchor_lang::prelude::*;

declare_id!("6TxBDM3BtmDm1HYE5dehsfVEUceBEWfm3VDEFCtXfYx2");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod fav_me {
    use super::*;

    pub fn set_favorites() -> Return<()> {}
}

#[derive(Accounts)]
pub struct Initialize {}
