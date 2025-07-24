# Solana Official Bootcamp · Gabriel Baruque

Este repositório contém meu progresso completo no **Bootcamp Oficial da Solana**, com foco em desenvolvimento de smart contracts utilizando o framework **Anchor**, linguagem **Rust**, e a infraestrutura da **blockchain Solana**.

---

## 🎯 Finalidade do Bootcamp

O bootcamp tem como objetivo ensinar, na prática, o desenvolvimento de programas descentralizados (smart contracts) na blockchain Solana, utilizando ferramentas modernas como:

- Anchor Framework (para abstração do runtime Solana)
- Solana CLI
- Program Derived Addresses (PDAs)
- Testes locais com `solana-test-validator`
- Transações customizadas e gerenciamento de contas

---

## 📦 Projetos

| Projeto | Descrição |
|--------|-----------|
| [`projeto1_favorites`](./projeto1_favorites) | Smart contract para salvar preferências do usuário (cor favorita, número e hobbies). Primeiro contato com contas, PDAs e Anchor macros. |
| [`projeto2_votingapp`](./projeto2_votingapp) | Sistema de votação descentralizado. Desenvolvido com foco em instruções, contadores e estrutura de propostas com múltiplos votos. (em andamento) |

Mais projetos serão adicionados conforme o avanço no bootcamp.

---

## 📚 Conceitos abordados

### 🌐 Blockchain Solana
- Slots, blocos e skipped slots
- Proof of History (PoH) e Tower BFT
- Líderes de bloco e o Leader Schedule
- Transações, mempool e throughput
- Resiliência e escalabilidade na prática

### ⚙️ Anchor Framework & Rust
- `#[program]`, `#[account]` e contextos
- Inicialização e mutabilidade de contas
- Seeds, bump, e derivação de PDAs
- Alocação de espaço (`InitSpace`, `max_len`)
- `set_inner()` para atualização segura de contas

### 🧪 Testes e CLI
- Uso do `solana-test-validator` para testar localmente
- Deploy de programas Anchor
- Comandos de CLI para interação com contas e transações

---

## 🧰 Ambiente de desenvolvimento

- **Sistema:** Ubuntu via WSL (Windows Subsystem for Linux)
- **Editor:** VSCode com extensão Remote - WSL
- **Ferramentas:**
  - Rust 1.79.0
  - Anchor 0.31.1 (via AVM)
  - Solana CLI atualizado
  - Docker (opcional, para validação futura)

---

## ✏️ Autor

**Gabriel Baruque**  
https://github.com/baruques  
Estudante de Blockchain, Segurança e Sistemas Distribuídos
