# BBF Frontend

React-based web interface for product provenance tracking.

## Features

- Solana wallet integration (Phantom, etc.)
- Product creation interface
- Ownership transfer
- Repair recording
- Product history viewer
- Responsive design
- Security headers (CSP)

## Development

```bash
npm install
npm run dev
```

Access at `http://localhost:8080`

## Build

```bash
npm run build
```

## Environment Variables

Copy `.env.example` to `.env`:

```bash
cp .env.example .env
```

## Wallet Setup

1. Install Phantom wallet browser extension
2. Create/import wallet
3. Switch to Devnet in wallet settings
4. Get devnet SOL from faucet

## Security

- CSP headers configured in nginx.conf
- No secrets in frontend code
- API calls use environment variables
- Wallet private keys never exposed
