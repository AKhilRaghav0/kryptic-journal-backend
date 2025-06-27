#!/bin/bash

set -e

echo "🔧 Starting Kryptic Journal in development mode..."

# Build the application image
echo "📦 Building application image..."
docker-compose build

# Start all services
echo "🚀 Starting services..."
docker-compose up -d postgres

# Wait for PostgreSQL to be ready
echo "⏳ Waiting for PostgreSQL to start..."
sleep 10

# Run migrations
echo "🗄️ Running database migrations..."
docker-compose run --rm migrator

# Start the API
echo "🔌 Starting API..."
docker-compose up -d api

echo "✅ Development environment is ready!"
echo ""
echo "🔗 Services:"
echo "   API: http://localhost:3000"
echo "   Health Check: http://localhost:3000/health"
echo "   PostgreSQL: localhost:5432"
echo ""
echo "📋 Useful commands:"
echo "   View logs: docker-compose logs -f api"
echo "   Stop all: docker-compose down"
echo "   Reset DB: docker-compose down -v && ./scripts/dev.sh" 