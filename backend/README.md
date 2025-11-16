# BBF Backend API

REST API for interacting with the Solana product provenance program.

## Features

- Structured JSON logging with Pino (for observability scorecard)
- Security headers with Helmet
- Rate limiting
- CORS protection
- Solana blockchain integration
- Environment-based configuration

## API Endpoints

### Products

- `POST /api/products` - Create new product (manufacturer)
- `POST /api/products/:productId/transfer` - Transfer ownership
- `POST /api/products/:productId/repair` - Record repair
- `GET /api/products/:productId/history` - Get product history

### Health

- `GET /health` - Health check endpoint

## Environment Variables

Copy `.env.example` to `.env` and configure:

```bash
cp .env.example .env
```

**NEVER commit .env file to git!**

## Development

```bash
npm install
npm run dev
```

## Production

```bash
npm start
```

## Security

- All secrets must be in environment variables
- Use Kubernetes secrets or Vault in production
- Rate limiting enabled (100 req/15min per IP)
- Helmet security headers
- CORS whitelist configuration
