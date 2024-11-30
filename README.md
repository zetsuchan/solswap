# SolSwap 🌊

A specialized LST (Liquid Staking Token) aggregator and stableswap AMM built for the Solana ecosystem.

## Overview

SolSwap is a specialized DeFi protocol designed to optimize trading and management of Liquid Staking Tokens on Solana. It features:

- 📊 LST Aggregation Platform
- 💧 Specialized Stableswap AMM
- 📈 Yield Analytics
- 🔄 Smart Routing
- 💰 Enhanced Capital Efficiency

## Features

### LST Aggregation
- Single interface for managing multiple LSTs (JitoSOL, mSOL, bSOL, etc.)
- Automated yield optimization
- Real-time APY comparison
- One-click staking/unstaking

### Stableswap AMM
- Custom curve optimized for LST pairs
- Lower slippage compared to traditional AMMs
- Dynamic fees based on pool imbalance
- Concentrated liquidity for LST pairs

### Smart Routing
- Intelligent stake distribution
- MEV optimization through Jito integration
- Commission optimization
- Slippage optimization

## Technology Stack

- Solana Blockchain
- Anchor Framework
- React/Next.js Frontend
- TailwindCSS
- Python testing framework

## Getting Started

### Prerequisites

```bash
# Install Solana Tool Suite
sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"

# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest

# Install Node.js dependencies
yarn install
```

### Development Setup

```bash
# Clone the repository
git clone https://github.com/zetsuchan/solswap.git
cd solswap

# Install dependencies
yarn install

# Build the program
anchor build

# Run tests
anchor test

# Start local development
yarn dev
```

## Project Structure

```
solswap/
├── programs/          # Solana programs (smart contracts)
│   └── solswap/      # Main program logic
├── app/              # Frontend application
├── tests/            # Test suites
├── scripts/          # Deployment and utility scripts
└── sdk/              # TypeScript SDK
```

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
