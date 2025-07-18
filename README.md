# Solana Official Bootcamp – Projeto 1

Este repositório contém o primeiro projeto realizado durante o Bootcamp oficial da Solana, utilizando o **Solana Playground** e a framework **Anchor** para desenvolvimento de smart contracts (programas on-chain).

## 🧠 Conceitos Aprendidos

### 📦 Módulos em Rust
- Criamos um módulo `favorites` com uma função `set_favorite`.
- Usamos `super::*` para importar itens do escopo superior.

### 🧱 Structs
- **Favorites**: representa uma conta customizada (PDA) com dados do usuário.
- **SetFavorites**: estrutura de **contexto** que define as contas envolvidas na instrução.

```rust
#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,
    #[max_len(50)]
    pub color: String,
    #[max_len(5, 50)]
    pub hobbies: Vec<String>
}
