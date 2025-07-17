# 🔐 ZKP Chaum-Pedersen Protocol in Rust

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

> Zero-Knowledge Proof implementation of the Chaum-Pedersen protocol using pure Rust.

## 📚 Overview

This project demonstrates a basic implementation of the **Chaum-Pedersen Zero-Knowledge Proof Protocol** in Rust. It serves as an educational and experimental tool for learning and understanding how non-interactive proofs of knowledge can be constructed over discrete log equality.

> 💡 The Chaum-Pedersen protocol proves that two discrete logarithms are equal **without revealing the secret** — a core concept in Zero-Knowledge Proofs (ZKP).

## ⚙️ Features

- [x] Generate public/private key pairs
- [x] Create and verify Chaum-Pedersen proofs
- [x] Modular design with separation of logic
- [ ] gRPC support via Tonic (WIP)
- [ ] CLI or Web interface (planned)

## 🛠️ Tech Stack

- **Rust** — main programming language
- **num-bigint** — for big integer operations
- **rand** — for cryptographically secure randomness
- **tonic** — to build authentication via gRPC
- **protobuf** — interface definition for gRPC

## 🚀 Running the Project

### Prerequisites

- Rust (latest stable version recommended)
- [Protobuf Compiler (`protoc`)](https://grpc.io/docs/protoc-installation/) for gRPC support
- Clone this repo:

```bash
git clone https://github.com/ParesSensei/ZKP-protocol-in-rust.git
cd ZKP-protocol-in-rust
cargo build
cargo run --bin server