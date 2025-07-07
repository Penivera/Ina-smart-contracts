
---

# 📝 UnchainHer Journal NFT Smart Contract

This smart contract powers the **UnchainHer Journal**, allowing users to document their thoughts as journal entries, manage privacy, and mint special entries as NFTs on the NEAR blockchain.

It extends NEAR's NFT standard with journaling capabilities, giving users control, privacy, and ownership of their digital expressions.

---

## ✨ Features

* **Journal Entries:**
  Users can add public or private journal entries with emotional tags.

* **NFT Minting:**
  Special journal entries can be tokenized as NFTs to ensure ownership and permanence.

* **Metadata Management:**
  The contract provides collection-level metadata (name, symbol, base\_uri) and per-token metadata for journal NFTs.

* **Minter Tracking:**
  Keeps track of which users have minted NFTs.

* **Privacy Controls:**
  Supports private entries visible only to the user.

---

## 📦 Contract Structure

### Storage

* `tokens: NonFungibleToken`
  Handles NFT logic and storage.

* `metadata: LazyOption<NFTContractMetadata>`
  Stores contract-level metadata (name, symbol, base\_uri).

* `minters: UnorderedSet<AccountId>`
  Tracks which accounts have minted NFTs.

* `journal_entries: Vector<JournalEntry>`
  Stores all journal entries.

---

## ⚙️ Initialization

### `initiate(media_url: String)`

Initializes the contract.

* `media_url`: IPFS or web URL for the journal NFT collection icon or media.

**Example:**

```bash
near call <contract> initiate '{"media_url": "https://ipfs.io/ipfs/<CID>"}' --accountId <your-account>
```

---

## 🛠 Methods

### ✍️ Journal Management

| Method                                         | Description                                                  |
| ---------------------------------------------- | ------------------------------------------------------------ |
| `add_journal_entry(content, tags, is_private)` | Adds a journal entry with tags and privacy flag.             |
| `get_all_entries()`                            | Returns all journal entries *(private view)*.                |
| `get_journal_by_id(entry_id)`                  | Fetch a specific journal entry by ID *(private view)*.       |
| `get_journal_by_token_id(token_id)`            | Fetch journal entry linked to a minted NFT *(private view)*. |
| `get_user_entries(user)`                       | Returns all entries by a specific user.                      |
| `get_private_entries(user)`                    | Returns private entries belonging to a user.                 |
| `get_public_entries()`                         | Returns all public journal entries.                          |

---

### 🎨 NFT Management

| Method                                    | Description                                              |
| ----------------------------------------- | -------------------------------------------------------- |
| `mint_nft_for_entry(entry_id, media_url)` | Tokenizes a journal entry as an NFT *(payable)*.         |
| `get_minted_entries()`                    | Returns all entries that have been minted as NFTs.       |
| `get_minters()`                           | Returns accounts that have minted NFTs *(private view)*. |
| `get_token_metadata(journal_id)`          | Returns metadata for a journal's NFT *(private view)*.   |
| `nft_metadata()`                          | Returns global NFT contract metadata *(private view)*.   |

---

### 🔎 Contract Metadata

| Method                                     | Description                                |
| ------------------------------------------ | ------------------------------------------ |
| `get_metadata()` *(Alias: `nft_metadata`)* | Returns contract-level metadata.           |
| `contract_source_metadata()`               | Returns build information of the contract. |

---

## 🏷 Allowed Tags for Journal Entries

* Happy
* Sad
* Angry
* Anxious
* Hopeful
* Grateful
* Lonely
* Confident
* Tired
* Overwhelmed

---

## 🖼️ Example NFT Metadata

```json
{
  "title": "UnchainHer Journal NFT",
  "description": "Tokenized Journal Entry from the UnchainHer Project",
  "media": "https://ipfs.io/ipfs/<CID>",
  "copies": 1,
  "issued_at": "<timestamp>"
}
```

---

## 🔗 Frontend Integration Guide

* **Add Entry:**
  Call `add_journal_entry()` for users to document thoughts.

* **Mint NFT:**
  Call `mint_nft_for_entry()` for eligible entries.

* **Display User Journal:**
  Use `get_user_entries()` to fetch user content, filtering private entries.

* **NFT Gallery:**
  Use `get_minted_entries()` to show all tokenized journal entries.

* **Collection Info:**
  Use `get_metadata()` or `nft_metadata()` for collection details.

---

## 🔒 Privacy Considerations

* **Public entries** are viewable by anyone.
* **Private entries** are only visible to the creator via `get_private_entries()` or `get_user_entries()`.

---

## 🧩 Notes

* **Token IDs:**
  Token IDs correspond to minted journal entries.

* **Ownership:**
  Minted NFTs give users verifiable ownership of specific entries.

* **Standard Compliance:**
  Built on NEAR's NFT standard for marketplace compatibility.

---

## 📄 License

MIT

---

**For more details, refer to the [NEAR NFT Standard Documentation](https://nomicon.io/Standards/NonFungibleToken/Core).**

---

