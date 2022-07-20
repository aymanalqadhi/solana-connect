# solana-connect
A custom Solana blockchain token implementation

## Available commands
This implementaion has four transaction types. Every transaction should pass the command identifier as the first byte in the attached data.
### 1. Create token
This transaction is used to initialize an existing account to be used as a token account.
***Arguments***
| # | Location | Name | Description | Notes |
| 0 | Accounts array | Token account | An account to be used as the token account | Must be a signer |
| 1 | Transaction data | Token | borsh-serialized token info (total, available, and initialize supply values) ||
### 2. Create account
This transaction is used to create accounts to hold tokens
***Arguments***
| # | Location | Name | Description | Notes |
| 0 | Accounts array | Owner account | The owner of the account ||
| 1 | Accounts array  | Token account | The token account to create the account for ||
| 2 | Accounts array | New account | The account which to link to the token | Must be a signer|
### 3. Mint
This transaction is used to mint tokens from a token account into another account
***Arguments***
| # | Location | Name | Description | Notes |
| 0 | Accounts array | Token account | The account to mint tokens from | Must be a signer |
| 1 | Accounts array | Destination account | The account to mint tokens into ||
| 2 | Transaction data | Amount | The amount of tokens to mint | Big-endian encoded 64-bit unsigned integer |
### 4. Transfer
This transaction is used to transfer a number of tokens from an account to another
***Arguments***
| # | Location | Name | Description | Notes |
| 0 | Accounts array | Token account | The token account ||
| 1 | Accounts array | From account | The source account | Must be signed |
| 2 | Accounts array | To account | The destination account ||
| 3 | Transaction data | Amount | The amount to be transferred | Big-endian encoded 64-bit unsigned integer |
