# Encode T8 DEX

A decentralized exchange (DEX) built on Solana using the Anchor framework. This project implements an automated market maker (AMM) with constant product formula (x * y = k), similar to Uniswap V2.

## 🌟 Features

- **Pool Initialization**: Create liquidity pools for any SPL token pair
- **Add Liquidity**: Deposit tokens to provide liquidity and earn LP tokens
- **Remove Liquidity**: Burn LP tokens to withdraw your proportional share
- **Token Swaps**: Exchange tokens using the constant product formula with 0.1% fee
- **Slippage Protection**: Configurable minimum output amounts to prevent excessive slippage

## 📋 Table of Contents

- [Architecture](#architecture)
- [Program Structure](#program-structure)
- [Installation](#installation)
- [Usage](#usage)
- [Instructions](#instructions)
- [Testing](#testing)
- [Security Features](#security-features)
- [Technical Details](#technical-details)

## 🏗️ Architecture

### Core Components

#### Pool State
The pool stores essential information about the liquidity pool:
- Token A and Token B mint addresses
- Vault addresses for holding tokens
- LP (Liquidity Provider) token mint
- PDA bump seed for secure signing

#### Instructions
1. **Initialize Pool**: Sets up a new liquidity pool
2. **Add Liquidity**: Deposits tokens and mints LP tokens
3. **Remove Liquidity**: Burns LP tokens and withdraws proportional amounts
4. **Swap**: Exchanges one token for another with fee

## 📁 Program Structure

```
encode_t8_dex/
├── programs/
│   └── encode_t8_dex/
│       └── src/
│           ├── lib.rs                    # Program entry point
│           ├── errors.rs                 # Custom error definitions
│           ├── state/
│           │   ├── mod.rs
│           │   └── pool.rs              # Pool account structure
│           └── instructions/
│               ├── mod.rs
│               ├── initialize_pool.rs   # Pool creation logic
│               ├── add_liquidity.rs     # Liquidity provision
│               ├── remove_liquidity.rs  # Liquidity withdrawal
│               └── swap.rs              # Token swap logic
├── tests/
│   └── encode_t8_dex.ts                 # Comprehensive test suite
└── migrations/
    └── deploy.ts                         # Deployment script
```

## 🛠️ Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (v1.89.0)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.18+)
- [Anchor Framework](https://www.anchor-lang.com/docs/installation) (v0.32.1)
- [Node.js](https://nodejs.org/) (v18+)
- [Yarn](https://yarnpkg.com/)

### Setup

1. **Clone the repository**
```bash
git clone https://github.com/encodeclub/encode_t8_dex.git
cd encode_t8_dex
```

2. **Install dependencies**
```bash
yarn install
```

3. **Build the program**
```bash
anchor build
```

4. **Configure Solana for local development**
```bash
solana config set --url localhost
```

5. **Start local validator (in a separate terminal)**
```bash
solana-test-validator
```

6. **Deploy the program**
```bash
anchor deploy
```

## 🚀 Usage

### Running Tests

Execute the comprehensive test suite:

```bash
anchor test
```

Or with local validator:

```bash
yarn run ts-mocha -p ./tsconfig.json -t 1000000 "tests/**/*.ts"
```

### Test Coverage

The test suite includes:
- Pool initialization
- Initial liquidity provision (first depositor)
- Subsequent liquidity provision (ratio checking)
- Token swaps with fee calculation
- Liquidity removal with proportional withdrawal

## 📝 Instructions

### 1. Initialize Pool

Creates a new liquidity pool for a token pair.

**Accounts:**
- `pool`: Pool PDA account (created)
- `lp_mint`: LP token mint PDA (created)
- `mint_a`: First token mint
- `mint_b`: Second token mint
- `token_vault_a`: Vault for token A (created)
- `token_vault_b`: Vault for token B (created)
- `payer`: Transaction fee payer
- `token_program`: SPL Token program
- `system_program`: System program

**Seeds:**
- Pool PDA: `["pool", mint_a, mint_b]`
- LP Mint PDA: `["lp_mint", mint_a, mint_b]`

### 2. Add Liquidity

Deposits tokens into the pool and receives LP tokens.

**Parameters:**
- `amount_a`: Amount of token A to deposit
- `amount_b`: Amount of token B to deposit

**Logic:**
- **First deposit**: LP tokens = √(amount_a × amount_b)
- **Subsequent deposits**: Enforces ratio matching
  - `required_b = amount_a × vault_b / vault_a`
  - `lp_to_mint = amount_a × lp_supply / vault_a`

**Validations:**
- Non-zero amounts
- Correct ratio (for existing pools)

### 3. Remove Liquidity

Burns LP tokens to withdraw proportional amounts of both tokens.

**Parameters:**
- `lp_amount`: Amount of LP tokens to burn

**Logic:**
- `amount_a = lp_amount × vault_a / lp_supply`
- `amount_b = lp_amount × vault_b / lp_supply`

**Validations:**
- Non-zero LP amount
- Sufficient LP tokens
- Non-zero withdrawal amounts

### 4. Swap

Exchanges one token for another using the constant product formula.

**Parameters:**
- `amount_in`: Amount of input token
- `min_amount_out`: Minimum acceptable output (slippage protection)

**Logic:**
- Fee: 0.1% (1/1000) of input amount
- Formula: `amount_out = (vault_out × amount_in_after_fee) / (vault_in + amount_in_after_fee)`

**Validations:**
- Non-zero input amount
- Slippage check (output ≥ min_amount_out)

## 🔒 Security Features

### Error Handling

Custom error codes with descriptive messages:
- `ZeroAmount`: Prevents zero-value operations
- `InvalidRatio`: Ensures deposits maintain pool ratio
- `InsufficientLiquidity`: Guards against empty pool operations
- `InsufficientLpTokens`: Validates LP token ownership
- `CalculationOverflow`: Catches arithmetic overflows
- `ZeroWithdrawAmount`: Prevents dust withdrawals
- `SlippageExceeded`: Protects against unfavorable swaps
- `CalculationFailure`: Handles general calculation errors

### Account Validation

- PDA verification using seeds and bumps
- Mint address matching
- Token account ownership checks
- Authority validation
- Constraint-based account validation

### Safe Math

- Checked arithmetic operations
- Overflow protection
- Integer square root for LP token calculation
- U128 intermediate calculations to prevent overflow

## 🔧 Technical Details

### Token Decimals

- LP tokens: 6 decimals (hardcoded)
- Supports any decimal configuration for pool tokens

### Fee Structure

- Swap fee: 0.1% (1/1000)
- Fee remains in the pool, benefiting liquidity providers

### Constant Product Formula

The AMM uses the formula: `x × y = k`

Where:
- `x` = Token A reserves
- `y` = Token B reserves
- `k` = Constant product

For swaps:
```
amount_out = (reserve_out × amount_in_after_fee) / (reserve_in + amount_in_after_fee)
```

### Program ID

```
5rSQi9WaBc2g5eTHM28jBcdzSB21JzxAnHRs9srXVrBT
```

### Account Space

**Pool Account**: 169 bytes
- 8 bytes: Anchor discriminator
- 160 bytes: 5 Pubkeys (32 bytes each)
- 1 byte: Bump seed

## 📊 Test Results

The test suite validates:

1. **Pool Initialization**: Successfully creates pool with correct PDAs
2. **Initial Liquidity**: Mints √(100e6 × 100e6) = 100e6 LP tokens
3. **Subsequent Liquidity**: Enforces 1:1 ratio, mints proportional LP tokens
4. **Swap Calculation**: 
   - Input: 30e6 token A
   - Fee: 30,000 (0.1%)
   - Output: 24,979,163 token B
5. **Liquidity Removal**: Returns proportional amounts
   - Burns 50e6 LP tokens (1/3 of supply)
   - Receives 60e6 token A and 41,673,612 token B

## 🤝 Contributing

Contributions are welcome! Please follow these guidelines:

1. Fork the repository
2. Create a feature branch
3. Write tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## 📄 License

ISC License

## 🔗 Resources

- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana Documentation](https://docs.solana.com/)
- [SPL Token Program](https://spl.solana.com/token)
- [Uniswap V2 Whitepaper](https://uniswap.org/whitepaper.pdf)

## ⚠️ Disclaimer

This project is for educational purposes. It has not been audited and should not be used in production without proper security review and testing.

## 📞 Support

For questions and support, please open an issue in the repository.

---

Built with ❤️ using Anchor and Solana
