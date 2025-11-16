# Blockchain-Based Product Provenance (BBF)

A secure cloud-native dApp for tracking product lifecycle on Solana blockchain.

## Architecture

```
┌─────────────┐     ┌──────────────┐     ┌─────────────────┐
│   Frontend  │────▶│   Backend    │────▶│ Solana Program  │
│   (React)   │     │   (Node.js)  │     │   (On-chain)    │
└─────────────┘     └──────────────┘     └─────────────────┘
```

## Components

- **solana-program/** - On-chain Solana program for product provenance
- **backend/** - REST API for interacting with Solana blockchain
- **frontend/** - Web UI for product registration and transfers
- **k8s/** - Kubernetes manifests for deployment

## Security Features

- ✅ Container vulnerability scanning with Trivy
- ✅ Dependency scanning in CI/CD
- ✅ Alpine Linux minimal base images
- ✅ Secrets management with Kubernetes secrets
- ✅ Structured logging with JSON format
- ✅ Runtime monitoring with Falco
- ✅ WAF protection with ModSecurity

## Use Cases

- Prove legitimate ownership for second-hand resale
- Verify authorized repair history
- Combat theft and counterfeit goods
- Reduce return fraud for retailers

## Getting Started

See component-specific READMEs:
- [Solana Program](./solana-program/README.md)
- [Backend API](./backend/README.md)
- [Frontend](./frontend/README.md)
