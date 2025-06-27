#!/bin/bash

set -e

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Get the project root (parent of scripts directory)
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "🚀 Deploying Kryptic Journal (Simple Docker Compose)..."
echo "📁 Project root: $PROJECT_ROOT"

# Change to project root
cd "$PROJECT_ROOT"

# Check if Docker is running
if ! docker info >/dev/null 2>&1; then
    echo "❌ Docker is not running or accessible"
    echo "💡 Try: sudo usermod -aG docker $USER && newgrp docker"
    exit 1
fi

# Create production environment file if it doesn't exist
if [ ! -f .env.prod ]; then
    echo "📝 Creating production environment file..."
    cp env.example .env.prod
    
    # Generate secure secrets
    JWT_SECRET=$(openssl rand -hex 32)
    ENCRYPTION_KEY=$(openssl rand -hex 32)
    
    # Update .env.prod with generated secrets
    sed -i "s/your-very-secure-jwt-secret-key-here-make-it-long-and-random/$JWT_SECRET/" .env.prod
    sed -i "s/your-64-character-hex-string-here-32-bytes-as-hex/$ENCRYPTION_KEY/" .env.prod
    
    echo "✅ Generated secure secrets in .env.prod"
    echo "⚠️  Please review and update DATABASE_URL in .env.prod if needed"
fi

# Build and start services
echo "📦 Building and starting services..."
docker-compose --env-file .env.prod up -d --build

# Wait for PostgreSQL to be ready
echo "⏳ Waiting for PostgreSQL to be ready..."
sleep 15

# Run migrations
echo "🗄️ Running database migrations..."
docker-compose --env-file .env.prod --profile migration run --rm migrator

# Show status
echo ""
echo "✅ Deployment complete!"
echo ""
echo "📊 Service status:"
docker-compose ps

echo ""
echo "🔗 Your API is available at:"
echo "   http://$(hostname -I | awk '{print $1}'):3000"
echo "   Health check: http://$(hostname -I | awk '{print $1}'):3000/health"
echo ""
echo "📋 Useful commands:"
echo "   View logs: docker-compose logs -f api"
echo "   Stop services: docker-compose down"
echo "   Update: ./scripts/deploy-simple.sh" 