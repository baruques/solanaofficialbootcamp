# Solana Official Bootcamp - Projeto 1

Este repositório contém meu progresso no bootcamp oficial da Solana, utilizando o Anchor Framework para desenvolver programas no blockchain Solana.

## 📚 Aprendizados do dia (2025-07-18)

### 🧠 Conceitos estudados

- **Anchor Framework**: biblioteca que abstrai e facilita a construção de programas Solana.
- **#[program]**: macro que indica o módulo principal do programa.
- **#[account]**: define structs que representam contas a serem armazenadas no blockchain.
- **PDA (Program Derived Address)**: contas criadas programaticamente por um programa.
- **Signer**: garante que o usuário assinou a transação.
- **Structs de Contexto**: definem as contas necessárias para a execução de cada instrução.
- **Discriminador**: os 8 primeiros bytes de toda conta Anchor, usados para identificar a struct.
- **InitSpace + max_len**: macros que ajudam a definir e limitar o espaço de armazenamento das contas.
- **Seeds e Bump**: usadas para derivar endereços de contas de forma determinística.
- **set_inner()**: método que atualiza o conteúdo de uma conta já inicializada com novos dados.

### 🔨 Código construído

```rust
#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,
    #[max_len(50)]
    pub color: String,
    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

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
