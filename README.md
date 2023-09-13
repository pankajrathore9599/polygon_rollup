# Polygon Rollup Transaction Simulation.

This project is a simulation of a simple blockchain-based transaction system. It consists of three main components: `Client`, `Aggregator`, and `Main Chain`. The client generates transactions, the aggregator collects and validates transactions, and the main chain records the transactions as blocks in the blockchain.

## Table of Contents.

- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Components](#components)
  - [Client](#client)
  - [Aggregator](#aggregator)
  - [Main Chain](#main-chain)

## Project Structure.

The project is organized into three main components:

- `client`: Simulates transaction generation and sends transactions to the aggregator.
- `aggregator`: Collects and validates transactions before sending them to the main chain.
- `main_chain`: Records transactions as blocks in the blockchain.

## Getting Started.

### Prerequisites.

Before running this project, make sure you have the following installed:

- Rust (https://www.rust-lang.org/tools/install)
- Install Cargo


### Setting up the project.
- Clone this repository.
- Navigate the repo.
- Install all the crates using ```cargo build``` in repo directory.

### Usage.
To simulate a blockchain-based transaction system, follow these steps:

1. Start the Main Chain Componet:
```cargo run main_chain``` from root directory. 

2. Start the Aggregator Component:
```cargo run aggregator``` from root directory.

These above 2 components with continously run and in sync with together now we have to send some trasactions to aggregator.

3. Run the Client Component for sending transaction everytime.
```cargo run client``` from root directory. {If you run one time the one trasaction will send.}
Note: The client sends one transaction each time you run it. To create a Rollup, the aggregator needs five transactions to send to the main chain.

## Components.

### Client
The Client component generates random transactions and sends them to the Aggregator. Each transaction includes sender and receiver information, an amount, a nonce, and a timestamp.

### Aggregator
The Aggregator collects incoming transactions from the Client, validates them, and forwards valid transactions to the Main Chain for recording. It performs basic checks such as signature verification and timestamp validation.

### Main Chain
The Main Chain records valid transactions as blocks in the blockchain. Each block contains multiple transactions and ensures the immutability and integrity of the transaction history. Every Rollup adds data to the Merkle tree for efficient verification.