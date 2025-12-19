# Decentralized Reputation System

A Soroban smart contract on the **Stellar testnet** that enables users to build and query on-chain reputation through peer reviews.

## ✨ Features
- Submit reputation reviews with scores from **-10 to +10**
- Prevents self-reviews
- Authenticated transactions (address-based)
- Tracks total, positive, and negative reviews
- Returns a **normalized reputation score (0–100)**
- Fully on-chain and immutable

## 🛠 Tech Stack
- **Blockchain:** Stellar (Testnet)
- **Smart Contracts:** Soroban
- **Language:** Rust
- **CLI:** Stellar CLI

## 📦 Contract Functions
- `submit_review(reviewer, reviewee, score, comment)`
- `get_reputation(user) -> (total_score, positive, negative, total, normalized)`

## 🚀 Deployment
The contract is deployed on **Stellar Testnet**.

**Contract ID:**
CDPJYSJJUMQXMUVTURYYAQWBWVVKOD3OED6A6UTNYYB3LKH53KTLGRJA

## 🧪 Example

```bash
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source-account dev \
  --network testnet \
  -- get_reputation \
  --user user2 ```

## 🔮 Future Improvements

-Weighted reviews

-Review history & pagination

-Time-based reputation decay

-Category-based reputation