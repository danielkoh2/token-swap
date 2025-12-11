# token-swap

## Usage

This repository is using [Surfpool](https://surfpool.run) as a part of its development workflow.

Install from source:

```console
# Clone repo
git clone https://github.com/txtx/surfpool.git

# Set repo as current directory
cd surfpool

# Build
cargo surfpool-install
```

### Start a Surfnet

```console
surfpool start
```

`anchor build`

`surfpool start`

`solana airdrop 100 -k ./key/daniel_local.json`

`anchor test --skip-local-validator`

## Token swap example amm in anchor rust

**Automated Market Makers (AMM)** - Your Gateway to Effortless Trading!
Welcome to the world of Automated Market Makers (AMM), where seamless trading is made possible with the power of automation. The primary goal of AMMs is to act as automatic buyers and sellers, readily available whenever users wish to trade their assets.

**Advantages of AMMs:**

- Always Available Trading: Thanks to the algorithmic trading, AMMs are operational round-the-clock, ensuring you never miss a trading opportunity.

- Low Operational Costs: Embrace cheaper trades as AMMs eliminate the need for a market-making firm. Say goodbye to hefty fees! (In practice, MEV bots handle this role.)

Selecting the right algorithm for the AMM becomes the essential task. One fascinating development in blockchain AMMs is the Constant Function AMM (CFAMM), which permits trades that preserve a predefined condition on a constant function of the AMM's reserves, known as the Invariant. This enforcement compels the reserves to evolve along a remarkable Bonding Curve.

Meet the Constant Product AMM (CPAMM): Among the simplest CFAMMs and made popular by Uniswap V2, the CPAMM ensures the product of both reserves (xy) remains constant (K) for a given liquidity quantity. Simply put, if x denotes the reserve of token A and y denotes the reserve of token B, then xy = K, with K depending on the liquidity.

*Discover Diverse Bonding Curves:*

- Constant Sum AMM (CSAMM): The pool's invariant, x + y = K, maintains a constant price, but reserves for each asset can be emptied.

- Curve's Stableswap: A clever mix of CSAMM and CPAMM, the Stableswap brings unique properties to the AMM, depending on the token balance.

- Uniswap V3 Concentrated Liquidity AMM (CLAMM): Utilizing CPAMM, this model splits the curve into independent buckets, allowing liquidity provision to specific price buckets for efficient trading.

- Trader Joe CLAMM: Similar to UniV3 CLAMM, it divides the price range into buckets, where each bucket operates as a CSAMM instead of a CPAMM.

*The Undeniable Perks of CPAMMs:*

- Easier to Understand and Use: Unlike complex liquidity buckets, CPAMMs offer a single, user-friendly pool for straightforward trading.

- Memory Efficiency: With just one pool to maintain instead of multiple buckets, CPAMMs are incredibly memory-efficient, leading to lower memory usage and reduced costs.

For these reasons, we focus on implementing the CPAMM.

## Program Implementation

### Design

Let's go over the essential requirements for our smart contract design:

- Fee Distribution: Every pool must have a fee to reward Liquidity Providers (LPs). This fee is charged on trades and paid directly in the traded token. To maintain consistency across all pools, the fees will be shared.

- Single Pool per Asset Pair: Each asset pair will have precisely one pool. This approach avoids liquidity fragmentation and simplifies the process for developers to locate the appropriate pool.

- LPs Deposit Accounting: We need to keep track of LPs deposits in the smart contract.

To achieve an efficient and organized design, we can implement the following strategies:

- Shared Parameters: As pools can share certain parameters like the trading fee, we can create a single account to store these shared parameters for all pools. Additionally, each pool will have its separate account. This approach saves storage space, except when the configuration is smaller than 32 bytes due to the need to store the public key. In our case, we'll include an admin for the AMM to control fees, which exceeds the limit.

- Unique Pool Identification: To ensure each pool remains unique, we'll utilize seeds to generate a Program Derived Account (PDA). This helps avoid any ambiguity or confusion.

- SPL Token for Liquidity Accounting: We'll utilize the SPL token standard for liquidity accounting. This choice ensures easy composability and simplifies the handling of liquidity in the contract.

By implementing these strategies, we are creating a solana program that efficiently manages liquidity pools, rewards LPs, and maintains a seamless trading experience across various asset pairs.

## Principals

Here are some essential principles to consider when building on-chain programs in Solana:

- Store Keys in the Account: It's beneficial to store keys in the account when creating Program Derived Accounts (PDAs) using seeds. While this may increase account rent slightly, it offers significant advantages. By having all the necessary keys in the account, it becomes effortless to locate the account (since you can recreate its public key). Additionally, this approach works seamlessly with Anchor's has_one clause, streamlining the process.

- Simplicity in Seeds: When creating PDA seeds, prioritize simplicity. Using a straightforward logic for seeds makes it easier to remember and clarifies the relationship between accounts. A logical approach is to first include the seeds of the parent account and then use the current object's identifiers, preferably in alphabetical order. For example, in an AMM account storing configuration (with no parent), adding an identifier attribute, usually a pubkey, becomes necessary since the admin can change. For pools, which have the AMM as a parent and are uniquely defined by the tokens they facilitate trades for, it's advisable to use the AMM's pubkey as the seed, followed by token A's pubkey and then token B's.

- Minimize Instruction's Scope: Keeping each instruction's scope as small as possible is crucial for several reasons. It helps reduce transaction size by limiting the number of accounts touched simultaneously. Moreover, it enhances composability, readability, and security. However, a trade-off to consider is that it may lead to an increase in Lines Of Code (LOC).

- By following these principles, you can build on-chain programs in Solana that are efficient, well-organized, and conducive to seamless interactions, ensuring a robust foundation for your blockchain projects.

## Code Examples

```file structure
programs/token-swap/src/
├── constants.rs
├── errors.rs
├── instructions
│   ├── create_amm.rs
│   ├── create_pool.rs
│   ├── deposit_liquidity.rs
│   ├── mod.rs
│   ├── swap_exact_tokens_for_tokens.rs
│   └── withdraw_liquidity.rs
├── lib.rs
└── state.rs
```