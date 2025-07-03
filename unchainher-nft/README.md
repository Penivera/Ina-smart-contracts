# UnchainHer NFT Smart Contract

This contract implements a NEAR Non-Fungible Token (NFT) collection for the UnchainHer project, empowering women with digital ownership. It is based on the NEAR NFT standard and provides minting, metadata, and minter tracking functionality.

---

## Features

- **NFT Minting:**  
  Each account can mint one unique NFT. Minting is restricted to one per account.

- **Metadata Management:**  
  The contract stores and exposes collection-level metadata (name, symbol, icon, base_uri) and per-token metadata.

- **Minter Tracking:**  
  Keeps track of all accounts that have minted an NFT.

---

## Contract Structure

### Storage

- `tokens: NonFungibleToken`  
  Handles NFT logic and storage.

- `metadata: LazyOption<NFTContractMetadata>`  
  Stores contract-level metadata (name, symbol, icon, base_uri).

- `minters: UnorderedSet<AccountId>`  
  Tracks which accounts have minted an NFT.

---

## Initialization

### `new(media_url: String) -> Self`

Initializes the contract.  
- `media_url`: The IPFS or web URL for the NFT collection's icon/media.  
- The contract owner is set to the account that deploys the contract.

**Example:**
```bash
near call <contract> new '{"media_url": "https://ipfs.io/ipfs/<CID>"}' --accountId <your-account>
```

---

## Methods

### `mint_nft()`

Mints an NFT for the caller.  
- Only one NFT per account is allowed.
- The NFT metadata is set using the contract's icon/media and standard fields.

**Example:**
```bash
near call <contract> mint_nft '{}' --accountId <your-account>
```

### `get_metadata() -> NFTContractMetadata`

Returns the contract's metadata (name, symbol, icon, base_uri, etc).

**Example:**
```bash
near view <contract> get_metadata '{}'
```

### `get_minters() -> Vec<AccountId>`

Returns a list of all accounts that have minted an NFT.

**Example:**
```bash
near view <contract> get_minters '{}'
```

### `get_token_metadata() -> Option<TokenMetadata>`

Returns the metadata for the NFT owned by the caller (using the caller's account ID as the token ID).

**Example:**
```bash
near view <contract> get_token_metadata '{}'
```

---

## Frontend Integration Guide

- **Mint Button:**  
  Call `mint_nft()` for the connected wallet. Disable the button if the user is already a minter (check with `get_minters()`).

- **Display NFT:**  
  Use `get_token_metadata()` to fetch and display the user's NFT image and details.

- **Show Collection Info:**  
  Use `get_metadata()` to display the collection's name, symbol, and icon.

- **List Minters:**  
  Use `get_minters()` to show all accounts that have minted an NFT.

---

## Backend Integration Guide

- **Ownership Verification:**  
  Use `get_minters()` to verify if a user has minted an NFT.

- **Metadata Fetching:**  
  Use `get_metadata()` and `get_token_metadata()` for collection and token details.

- **Minting Automation:**  
  Call `mint_nft()` on behalf of users (requires their signature).

---

## Notes

- **Token ID:**  
  The token ID is set to the minter's account ID.

- **Media URL:**  
  The contract expects an IPFS or HTTP(S) URL for the collection icon/media.

- **Standard Compliance:**  
  Built on NEAR's NFT standard for compatibility with wallets and marketplaces.

---

## Example NFT Metadata

```json
{
  "title": "UnchainHer NFT",
  "description": "Empowering Women with Ownership",
  "media": "https://ipfs.io/ipfs/<CID>",
  "copies": 1,
  "issued_at": "<timestamp>"
}
```

---

## License

MIT

---

**For more information, see [NEAR NFT Standard Documentation](https://nomicon.io/Standards/NonFungibleToken/Core).**
