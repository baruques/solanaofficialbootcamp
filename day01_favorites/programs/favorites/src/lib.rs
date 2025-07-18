use anchor_lang::prelude::*;

declare_id!("");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorite() -> Return <()> {
        // Preencher depois
    }
}
#[account]
#[derive(InitSpace)]
pub struct Favorites{
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>
}

pub struct SetFavorites<'info> {         // É uma struct de contexto da função, por isso tem o mesmo nome. Ela vai criar contas necessárias (PDAs)
    #[account(mut)]                      // para o funcionamento do programa na blockchain. A conta favorites associada a um usuário será criada
    pub user: Signer<'info>,             // apenas caso já não exista uma conta deste programa associada a este user. 

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>
}