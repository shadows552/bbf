#!/bin/bash

# BBF - Blockchain-Based Product Provenance
# Startup script for complete security stack

set -e

echo "🚀 Starting BBF - Blockchain-Based Product Provenance"
echo "=================================================="
echo ""

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check Docker
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed. Please install Docker first."
    exit 1
fi

if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi

echo "✅ Docker and Docker Compose found"
echo ""

# Check if .env files exist
if [ ! -f backend/.env ]; then
    echo "${YELLOW}⚠️  backend/.env not found. Copying from .env.example...${NC}"
    cp backend/.env.example backend/.env
fi

if [ ! -f frontend/.env ]; then
    echo "${YELLOW}⚠️  frontend/.env not found. Copying from .env.example...${NC}"
    cp frontend/.env.example frontend/.env
fi

echo "${GREEN}✅ Environment files ready${NC}"
echo ""

# Start services
echo "🐳 Starting Docker Compose services..."
docker-compose up -d

echo ""
echo "⏳ Waiting for services to be healthy (30 seconds)..."
sleep 30

echo ""
echo "${GREEN}✅ BBF Stack is starting up!${NC}"
echo ""
echo "=================================================="
echo "📡 Service URLs:"
echo "=================================================="
echo "🌐 Frontend:          http://localhost:8080"
echo "🔌 Backend API:       http://localhost:3000"
echo "📊 Grafana:           http://localhost:3001"
echo "   └─ User:           admin"
echo "   └─ Password:       admin123"
echo "🔐 OPA Policy:        http://localhost:8181"
echo "📝 Loki (Logs):       http://localhost:3100"
echo ""
echo "=================================================="
echo "📈 Grafana Dashboards:"
echo "=================================================="
echo "Application Logs:     http://localhost:3001/d/bbf-logs"
echo "Security Monitoring:  http://localhost:3001/d/bbf-security"
echo ""
echo "=================================================="
echo "🧪 Quick Security Tests:"
echo "=================================================="
echo "1. Trigger Falco alert:"
echo "   docker exec -it bbf-backend sh"
echo ""
echo "2. Test WAF (SQL injection):"
echo "   curl \"http://localhost/api/products?id=1' OR '1'='1\""
echo ""
echo "3. Test rate limiting:"
echo "   for i in {1..150}; do curl http://localhost/api/products & done"
echo ""
echo "=================================================="
echo "📋 Useful Commands:"
echo "=================================================="
echo "View logs:            docker-compose logs -f"
echo "Stop services:        docker-compose down"
echo "Restart service:      docker-compose restart <service>"
echo "Service status:       docker-compose ps"
echo ""
echo "${GREEN}🎉 Setup complete! Happy hacking!${NC}"
