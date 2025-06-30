# Journaling Vault on NEAR

## Overview
The Journaling Vault is a smart contract project on the NEAR platform that allows users to submit and save text journal entries. Each entry can have optional tags and a privacy setting, enabling users to manage their personal reflections securely.

## Features
- **Submit Journal Entries**: Users can create journal entries with text content.
- **Tags**: Users can add tags to their entries for better organization.
- **Privacy Settings**: Each entry can be marked as private or public, allowing users to control who can see their entries.
- **Fetch Entries**: Users can retrieve their own entries or view public entries.

## Project Structure
```
journaling-vault-near
├── src
│   ├── lib.rs          # Entry point for the smart contract
│   ├── models.rs       # Data structures for journal entries
│   ├── contract.rs      # Core functionality of the smart contract
│   └── types.rs        # Additional types and enums
├── Cargo.toml          # Rust project configuration
└── README.md           # Project documentation
```

## Getting Started

### Prerequisites
- Rust installed on your machine
- NEAR CLI installed for deploying the contract

### Installation
1. Clone the repository:
   ```
   git clone <repository-url>
   cd journaling-vault-near
   ```

2. Build the project:
   ```
   cargo build --release
   ```

### Deployment
1. Deploy the contract to the NEAR testnet:
   ```
   near deploy --accountId <your-account-id> --wasmFile target/wasm32-unknown-unknown/release/journaling_vault.wasm
   ```

### Interacting with the Contract
- To add a journal entry:
  ```
  near call <contract-account-id> add_journal_entry '{"user": "<your-account-id>", "content": "<your-entry-content>", "tags": ["tag1", "tag2"], "is_private": false}' --accountId <your-account-id>
  ```

- To fetch your journal entries:
  ```
  near call <contract-account-id> get_user_entries '{"user": "<your-account-id>"}' --accountId <your-account-id>
  ```

- To fetch public entries:
  ```
  near call <contract-account-id> get_public_entries '{}' --accountId <your-account-id>
  ```

## License
This project is licensed under the MIT License. See the LICENSE file for more details.