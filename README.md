# PureChain – A Private Blockchain Ecosystem

PureChain is a private blockchain ecosystem designed to address key challenges in decentralization, security, scalability, and cost efficiency. By integrating innovative consensus mechanisms and optimization techniques, PureChain operates without gas fees while ensuring robust performance and fault tolerance. This ecosystem is built with a multi-layer architecture and advanced enhancement algorithms to support secure and scalable operations.

---

## Table of Contents

- [Introduction](#introduction)
- [Architecture Overview](#architecture-overview)
    - [Enhancement Algorithms](#enhancement-algorithms)
    - [Consensus Mechanism – PoA2](#consensus-mechanism---poa2)
    - [Blockchain Layers](#blockchain-layers)
- [Core Modules](#core-modules)
- [Running PureChain](#running-purechain)
- [Testing](#testing)
- [Deployment on Ethereum](#deployment-on-ethereum)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

---

## Introduction

PureChain is a private, permissioned blockchain ecosystem that eliminates gas fees while maintaining high security and scalability. It leverages a hybrid architecture that integrates advanced consensus and mining algorithms with a layered network design. This design enables PureChain to deliver efficient transaction validation and state management for both enterprise and decentralized environments.

---

## Architecture Overview

PureChain’s architecture is built around two key enhancement algorithms and a multi-layer blockchain design:

### Enhancement Algorithms

- **Smart Auto Mining Plus (SAM+)**  
  SAM+ is an energy-efficient mining algorithm that activates only when there are pending transactions. It:
    - **Stops mining** during idle periods to conserve computational resources.
    - **Prioritizes transactions** based on size and urgency.
    - **Reduces storage and computational overhead**.

- **Proof of Authority and Association (PoA2)**  
  PoA2 is a private, permissioned consensus model where:
    - Validators are **pre-approved and registered** using digital signatures.
    - A **standby validator system** dynamically monitors and replaces inactive nodes.
    - An **automatic authority transfer mechanism** ensures continuous network integrity.
    - A **clique consensus protocol** enhances scalability.

### Blockchain Layers

PureChain is structured into three blockchain layers that optimize performance and interoperability:

- **NSL Layer 2 (NSL-L2) – ZK-Rollups Technology**  
  Leverages zero-knowledge proofs (ZK-SNARKS) to bundle transactions into compact rollups, significantly reducing transaction costs while enhancing privacy.

- **NSL Centralized Layer 2 (NSL-CL1)**  
  A permissioned Layer 2 solution optimized for enterprise applications. It employs smart contracts, JavaScript interfaces, and IPFS for high-performance processing.

- **Augmented Layer 1 Blockchain**  
  The foundational layer that integrates both SAM+ and PoA2. This layer is fully **EVM-compatible** and supports a staking mechanism for validator rewards, ensuring governance by trusted entities without block rewards.

---

## Core Modules

The PureChain Rust codebase is organized into several modules for clarity and extensibility:

- **Core Modules:**
    - **Blockchain (`core/blockchain.rs`)** – Manages block creation, storage, and validation.
    - **State Management (`core/state.rs`)** – Handles ledger states and account balances.
    - **Validators (`core/validator.rs`)** – Implements PoA2 and dynamic validator management.

- **Cryptography & Security:**
    - **Hashing Algorithms (`crypto/hash.rs`)** – Provides SHA-256 and Keccak256 functions.
    - **Digital Signatures (`crypto/signature.rs`)** – Implements ECDSA-based signing and verification.

- **Accounts:**
    - **Wallet (`accounts/wallet.rs`)** – Generates and manages ECDSA key pairs and derives addresses.
    - **Transactions (`accounts/transactions.rs`)** – Defines the structure for transactions.

- **Networking & P2P Communication:**
    - **Peer-to-Peer Networking (`p2p/network.rs`)** – Manages node discovery and block broadcasting.
    - **Network Identity (`internal/identity.rs`)** – Handles unique node identification.

- **RPC API:**
    - **RPC Server (`rpc/server.rs`)** – Provides HTTP endpoints for interacting with the blockchain (e.g., chain retrieval, transaction submission).

- **Mining & Consensus:**
    - **SAM+ Mining Algorithm (`miner/sam_plus.rs`)** – Implements dynamic mining based on transaction activity.
    - **Proof of Authority Engine (`pow/proof_of_authority.rs`)** – Governs validator selection and block validation.

- **Additional Functionalities:**
    - **Event Listener (`event/listener.rs`)** – Captures and processes blockchain events.
    - **Logger & Monitoring (`logger/log.rs`, `metrics/monitoring.rs`)** – Handles logging and system metrics.
    - **Merkle Patricia Trie (`trie/merkle.rs`)** – Ensures secure state storage using a simplified trie.

---

## Running PureChain

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Edition 2021)
- [Cargo](https://doc.rust-lang.org/cargo/)

### Building the Project

Clone the repository and build the project with Cargo:

```bash
git clone https://github.com/c0llinx/purchchain-rs
cd purechain-rs
cargo build --release
```
### Running the Node

Start the PureChain node (which launches the RPC server):
```bash
cargo run --release
``` 
The RPC server will start on port 8080 by default, exposing endpoints such as /chain and /transaction.

### Testing
PureChain includes a comprehensive test suite that covers all modules. To run the tests:
```bash
cargo test
```
This command runs unit and integration tests for blockchain functionality, cryptography, wallet operations, networking, mining, and more.