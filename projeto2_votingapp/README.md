
---

## ✅ Projeto 2: Voting App

Iniciado seguindo o vídeo oficial do bootcamp:

🔗 [Bootcamp Oficial - YouTube](https://www.youtube.com/watch?v=amAq-WHAFs8)

### 📦 Objetivo

Criar um programa em Solana que permite que usuários votem em propostas pré-definidas.  
O foco está em reforçar conceitos de:

- Estrutura de contas com Anchor (`#[account]`)
- Instruções com validações
- Contadores persistentes
- Gerenciamento de usuários e votos

### 🔨 Progresso

- Pasta criada com `npx create-solana-dapp` (scaffold)
- Projeto inicializado no terminal WSL no diretório `~/solana-projects/solanaofficialbootcamp/`
- Ambiente de desenvolvimento com:
  - Rust 1.79.0
  - Anchor 0.31.1 via AVM
  - Solana CLI instalado e atualizado
  - VSCode com Remote WSL configurado

---

## 📚 Conceitos abordados e estudados recentemente

### ⚙️ Ambiente e CLI

- `solana-test-validator`: simula localmente o ambiente blockchain para testes
- `anchor init`, `build`, `deploy`, `test`: comandos principais da Anchor CLI
- Integração com VSCode + WSL para desenvolvimento fluido

### ⛓ Conceitos de Blockchain (Solana):

- **Slot**: unidade de tempo fixa (~400ms); cada slot tem um líder de bloco
- **Bloco**: proposto por um único líder por slot
- **Skipped slots**: slots sem bloco; não afetam o relógio da rede, mas reduzem throughput
- **Proof of History (PoH)**: mecanismo de ordenação determinística das transações no tempo
- **Tower BFT**: algoritmo de consenso que empilha votos com lockouts crescentes
- **Leader Schedule**: cronograma determinístico de qual validador será o líder em cada slot
- **Double-voting**: tentativa maliciosa de votar em mais de um fork — detectável e penalizável
- **Sealevel**: motor de execução paralela que permite transações simultâneas na Solana

---

## 🔐 Discussões de arquitetura e resiliência

- Impacto dos skipped slots na performance e escalabilidade global
- Comparativo técnico entre Solana e Ethereum (PoS) em termos de liderança, paralelismo e segurança
- Pontos fortes e riscos do modelo de liderança único por slot
- Recuperação da Solana após a falha de 2022 e melhorias na rede

---

## 🚀 Próximos passos

- Implementar estrutura de conta de propostas e votos
- Criar instruções para votação
- Escrever testes de integração com Anchor
- Documentar endpoints e lógica de negócios

---

> Documentado com base nos estudos e avanços práticos no bootcamp Solana.  
> Autor: Gabriel Baruque
