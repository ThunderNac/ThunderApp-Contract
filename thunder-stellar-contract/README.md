# 💫 Soroban Project

This repository contains multiple smart contracts developed for the **Soroban Smart Contract Platform** on Stellar. It follows the recommended structure for multi-contract Soroban projects.

## 📁 Project Structure

```text
.
├── contracts
│   ├── erc20_token
│   │   ├── src
│   │   │   ├── lib.rs
│   │   │   └── test.rs
│   │   └── Cargo.toml
│   ├── erc3643_compliance_token
│   │   ├── src
│   │   │   ├── lib.rs
│   │   │   └── test.rs
│   │   └── Cargo.toml
│   ├── nft_mockup_erc721
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml

├── Cargo.toml
└── README.md
```

### 📦 Available Contracts

#### ✅ `erc20_token`

A ERC20 token contract compliant with Soroban logic. Supports standard functions for futur prupose:

- `mint`, `burn`, `transfer`, `approve`, `allowance`.

#### ✅ `erc3643_compliance_token`

A prototype inspired by the **ERC-3643** standard with:

- Identity (KYC) management via allowlists.
- On-chain compliance logic with access control.
- Support for custom transfer rules.

> ✨ This contract is adapted for the Soroban environment, as there's no official ERC-3643 standard yet.

#### ✅ `nft_mockup_erc721`

A  mockup of ERC721-like behavior:

- NFT minting with URI.
- Read owner and URI.
- Simplified structure to explore NFT compatibility on Soroban.

---

## 🧪 Tests

Each contract includes a `test.rs` file with built-in unit tests.

To run all tests:

```bash
cargo test
```

---

## 🚀 Deployment

Contracts can be built and deployed using the Soroban CLI:

```bash
soroban build
soroban deploy ...
```

> 🔧 Make sure you configure `soroban-cli` and your network environment properly (sandbox/testnet/mainnet).

---

## 🧭 Coming Soon

- React frontend interface using `@stellar/soroban-client`
- CLI tools for batch mint / transfer
- Dynamic compliance layer for RWA support
