use anchor_lang::prelude::*;

declare_id!("E6B6P5XHWmcCTHEStA7baftDtLQZbyGy847ykrZM4han");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favorites {
    use super::*;

    pub fn set_favorite(
        context: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greetings from {}", context.program_id);
        let user_public_key = context.accounts.user.key();

        msg!("User {user_public_key}'s favorite number is {number}, favorite color is {color} and their hobbies are {hobbies:?}"); // Aqui usamos :? em hobbies para que os
                                                                                                                                   // os itens no vetor expandam.
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
                             // É uma struct de contexto da função, por isso tem o mesmo nome. Ela vai criar contas necessárias (PDAs)
    #[account(mut)]          // para o funcionamento do programa na blockchain. A conta favorites associada a um usuário será criada
    pub user: Signer<'info>, // apenas caso já não exista uma conta deste programa associada a este user.

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}
