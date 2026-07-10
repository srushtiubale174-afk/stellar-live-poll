# 🗳️ Stellar Live Poll

A live polling dApp built on Stellar (Soroban smart contracts). Users connect their wallet, vote for their favorite blockchain, and see results update in real-time — all transactions are verifiable on-chain.

## 🌟 Features

- Multi-wallet support via **StellarWalletsKit** (Freighter, Albedo, xBull, and more)
- Smart contract deployed on **Stellar Testnet**
- Real-time vote results synced with contract state
- Transaction status tracking (pending / success / fail)
- Error handling for:
  - Wallet not found / not installed
  - Transaction rejected by user
  - Insufficient balance / contract execution errors

## 🛠️ Tech Stack

- **Smart Contract:** Rust + Soroban SDK
- **Frontend:** React (Vite)
- **Wallet Integration:** @creit.tech/stellar-wallets-kit
- **Blockchain SDK:** @stellar/stellar-sdk

## 📦 Project Structure