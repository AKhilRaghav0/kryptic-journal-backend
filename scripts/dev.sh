#!/bin/bash

set -e

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Get the project root (parent of scripts directory)
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "🔧 Starting Kryptic Journal in development mode..."
echo "📁 Project root: $PROJECT_ROOT"

# Change to project root
cd "$PROJECT_ROOT"

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
docker-compose --profile migration run --rm migrator

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