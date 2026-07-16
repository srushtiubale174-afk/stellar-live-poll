\# Stellar Live Poll — Advanced dApp (Level 3)



A production-ready decentralized polling application built on Stellar using Soroban smart contracts, featuring inter-contract communication, a rewards system, and a fully tested CI/CD pipeline.



\## 🌟 Features



\- \*\*Live on-chain voting\*\* — Users vote using their Stellar wallet (Freighter)

\- \*\*Inter-contract communication\*\* — The poll contract automatically calls the rewards contract when a vote is cast

\- \*\*Reward points system\*\* — Voters earn 10 points per vote, tracked on-chain

\- \*\*Real-time updates\*\* — Vote counts and reward points update immediately after each transaction

\- \*\*Error handling \& loading states\*\* — Clear feedback during wallet connection, voting, and data fetching

\- \*\*Automated testing\*\* — 7 unit tests across both contracts

\- \*\*CI/CD pipeline\*\* — GitHub Actions automatically tests both contracts and builds the frontend on every push



\## 🏗 Architecture

\### Inter-Contract Communication



When a user votes on `poll-contract`, the contract internally calls `rewards-contract.add\_reward()` to award 10 points to the voter — all within a single blockchain transaction.



\## 🚀 Deployed Contracts (Stellar Testnet)



| Contract | Contract ID |

|---|---|

| Poll Contract | `CBFVVG4JCPACM24QDURZ3RAC3HI3OZEPUYFV6UCM5FKLRMTNTTSIUXVY` |

| Rewards Contract | `CAGRVBNENLGRGAFJV46IZQTDQRONZ2LFFLVOF6CEBJKJQ2S23AQWR2WX` |



\*\*Deployer Wallet:\*\* `GAMKDB6GPDBTGE7TWIOCTFN6S4MCG6FVG25FTW6TOCEK6DI4QGHEPTRY`



\### Transaction Hashes



| Action | Hash |

|---|---|

| Rewards Contract Deploy | `6c40b9a475c36131590209f43b77cf1ec5f21d58e8c011302f5275bd3da00c4d` |

| Poll Contract Deploy | `449e107c9be2f574ecb0ec9e1940cde19cd5e13286edea0b3bfcd7f31c44acd2` |

| Poll Initialize | `c81071d2d3e467a58de88ca4b706676e97d2614f563f4115420c3c9f7b6c4671` |



\## 🧪 Testing



Both contracts have unit tests covering core functionality.



\*\*Rewards Contract (4 tests):\*\*

\- `test\_add\_reward\_and\_get\_points`

\- `test\_multiple\_rewards\_accumulate`

\- `test\_get\_rewards\_list`

\- `test\_voter\_with\_no\_rewards\_has\_zero\_points`



\*\*Poll Contract (3 tests):\*\*

\- `test\_initialize\_and\_get\_options`

\- `test\_vote\_increases\_count`

\- `test\_get\_results\_starts\_at\_zero`



\## ⚙️ CI/CD



GitHub Actions runs automatically on every push to `main`:

1\. Builds and tests `rewards-contract`

2\. Builds and tests `poll-contract` (using the rewards contract's compiled wasm)

3\. Installs dependencies and builds the frontend



See `.github/workflows/ci.yml` for details.



\## 🖥 Running Locally



\### Prerequisites

\- Rust + Soroban CLI (`stellar` CLI)

\- Node.js 18+

\- Freighter wallet browser extension



\### Smart Contracts

```bash

\# Build rewards contract

cd rewards-contract

cargo build --target wasm32v1-none --release

cargo test



\# Build poll contract

cd ../poll-contract/contracts/poll

cargo build --target wasm32v1-none --release

cargo test

```




## 📹 Demo Video

Watch the full demo here: [https://youtu.be/EoYr87WA-8k]