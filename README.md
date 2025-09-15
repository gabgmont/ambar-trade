# Ambar-Trade

Ambar-Trade é uma plataforma de *tokenização de energia*, criada para o Hackathon Meridian da Stellar.  
O objetivo é entregar um sistema onde tokens representam energia elétrica, com lastro no **PLD (Preço de Liquidação das Diferenças)** definido pela **CCEE (Câmara de Comercialização de Energia Elétrica)**.  

O projeto inclui contratos inteligentes desenvolvidos em **Rust** e compilados para **WebAssembly (WASM)**, um **frontend em React**, e um **oráculo** responsável por atualizar o preço do token com base no PLD.

---

## 📌 Índice

- [Visão Geral](#visão-geral)  
- [Motivação](#motivação)  
- [Arquitetura](#arquitetura)  
- [Funcionalidades](#funcionalidades)  
- [Tecnologias](#tecnologias)  
- [Instalação & Como Executar](#instalação--como-executar)  
- [Contrato & Oráculo](#contrato--oráculo)  
- [Frontend](#frontend)  
- [Desenvolvimento](#desenvolvimento)  
- [Testes](#testes)  
- [Contribuição](#contribuição)  
- [Licença](#licença)  
- [Autores](#autores)

---

## 🔎 Visão Geral

A Ambar-Trade permite:

- Tokenizar energia elétrica, possibilitando negociação de forma digital.  
- Garantir que o valor do token esteja sempre **atrelado ao PLD** definido pela CCEE.  
- Proporcionar transparência, segurança e automação por meio de **contratos inteligentes**.  

---

## 🎯 Motivação

- Falta de liquidez e transparência nos preços do mercado de energia.  
- Necessidade de ativos digitais que reflitam valores reais e regulamentados.  
- Exploração das possibilidades de **tokenização de ativos reais** no ecossistema Web3.  

---

## 🏗 Arquitetura

1. **Contratos Inteligentes**  
   - Desenvolvidos em Rust, compilados para WebAssembly, rodando na **rede Stellar/Soroban**.  

2. **Oráculo de Preços**  
   - Coleta o PLD da CCEE periodicamente e atualiza o contrato com o valor mais recente.  

3. **Frontend em React**  
   - Interface para interação do usuário: emissão, consulta de saldo e preços, transferências.  

4. **Scripts de Deploy / Inicialização**  
   - Auxiliam no deploy dos contratos e configuração do oráculo.  

---

## ⚡ Funcionalidades

- Emissão de tokens lastreados no PLD.  
- Transferência de tokens entre usuários.  
- Consulta em tempo real ao preço do token.  
- Oráculo automatizado para atualização do preço.  
- Interface web amigável para interação.  

---

## 🛠 Tecnologias

| Camada | Tecnologias |
|--------|-------------|
| Smart Contracts | Rust, WebAssembly, Soroban/Stellar |
| Oráculo | Scripts Shell, integração com dados do PLD |
| Frontend | React, HTML, CSS, JS |
| Infraestrutura | Node.js, Cargo, Stellar SDK |

---

## 🚀 Instalação & Como Executar

### Pré-requisitos

- [Rust](https://www.rust-lang.org/)  
- [Cargo](https://doc.rust-lang.org/cargo/)  
- [Node.js](https://nodejs.org/) (npm ou yarn)  
- Conta/configuração da **Stellar / Soroban Testnet**  
- Acesso a fonte de dados do **PLD da CCEE**  

### Passos

1. Clone o repositório:
   ```bash
   git clone https://github.com/gabgmont/ambar-trade.git
   cd ambar-trade
   ```

2. Instale as dependências do frontend:
   ```bash
   cd frontend
   npm install
   ```

3. Compile os contratos:
   ```bash
   cd contracts
   cargo build --target wasm32-unknown-unknown --release
   ```

4. Faça o deploy dos contratos e configure o oráculo:
   ```bash
   ./scripts/deploy_oracle.sh
   ./scripts/init_oracle.sh
   ```

5. Inicie o frontend:
   ```bash
   cd frontend
   npm start
   ```

---

## 📡 Contrato & Oráculo

- Os contratos armazenam e gerenciam a lógica do token atrelado ao PLD.  
- O oráculo coleta o valor do PLD da CCEE e envia atualizações ao contrato.  
- Há tratamento para cenários de falha ou indisponibilidade da fonte de dados.  

---

## 💻 Frontend

- Interface web para:  
  - Visualizar saldo de tokens.  
  - Emitir e transferir tokens.  
  - Acompanhar o histórico e preço atualizado.  
- Integração com carteiras Stellar (rede Soroban).  

---

## 🧪 Testes

- **Contratos**: testes unitários em Rust.  
- **Oráculo**: simulações de entrada de dados.  
- **Frontend**: testes de interface e integração.  

---

## 🤝 Contribuição

1. Abra uma *issue* com sugestões ou bugs.  
2. Fork o repositório.  
3. Crie uma branch (`feature/minha-feature`).  
4. Faça commits claros e objetivos.  
5. Abra um Pull Request explicando suas alterações.  

---

## 📜 Licença

Este projeto está sob a licença **MIT**.  
Consulte o arquivo [LICENSE](LICENSE) para mais informações.  

---

## 👤 Autores

- Gabriel Monteiro - [@gabgmont](https://github.com/gabgmont)  
- Beto Rocha - [@beto-rocha-blockchain](https://github.com/beto-rocha-blockchain)  
- Patricia Sirvarolli - [@psirvarolli](https://github.com/psirvarolli)
---
