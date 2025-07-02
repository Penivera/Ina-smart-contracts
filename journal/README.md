# Journaling Vault Smart Contract

A NEAR smart contract for securely storing and retrieving user journal entries with privacy controls and tag support.

## Features

- **Add Journal Entry:**  
  Users can submit a journal entry with text content, tags (mood or cycle), and a privacy flag (`is_private`).

- **Get User Entries:**  
  Retrieve all journal entries for a specific user.

- **Get Public Entries:**  
  Retrieve all journal entries marked as public (for DAO voting or community features).

## Data Model

### JournalEntry

| Field       | Type      | Description                        |
|-------------|-----------|------------------------------------|
| user        | string    | NEAR account ID of the entry owner |
| content     | string    | The journal text                   |
| tags        | Tag[]     | Tags for mood or cycle             |
| is_private  | boolean   | Privacy flag (true = private)      |

### Tag

Supported values:
- Happy
- Sad
- Angry
- Anxious
- Hopeful
- Grateful
- Lonely
- Confident
- Tired
- Overwhelmed

## Contract Methods

### `add_journal_entry(content: String, tags: Vec<Tag>, is_private: bool)`

Add a new journal entry for the caller.

### `get_user_entries(user: AccountId) -> Option<JournalEntry>`

Fetch all journal entries for a given user.

### `get_public_entries() -> Vec<JournalEntry>`

Fetch all public journal entries.

### `new()`

Initialize the contract.

## Usage

1. **Deploy the contract** to NEAR.
2. **Call `add_journal_entry`** to add your journal entry.
3. **Call `get_user_entries`** to retrieve your entries.
4. **Call `get_public_entries`** to see all public entries.

## JSON Schema

See [`info.json`](journal/info.json) for the full contract interface and data schema.

---

**Project generated with [Cargo Near](https://github.com/near/cargo-near)**
