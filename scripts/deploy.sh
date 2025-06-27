#!/bin/bash

set -e

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Get the project root (parent of scripts directory)
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "🚀 Deploying Kryptic Journal to Kubernetes..."
echo "📁 Project root: $PROJECT_ROOT"

# Change to project root
cd "$PROJECT_ROOT"

# Build and tag Docker images
echo "📦 Building Docker images..."
docker build -t kryptic-journal-backend:latest .
docker build -f Dockerfile.migrator -t kryptic-journal-migrator:latest .

# Apply Kubernetes manifests in order
echo "🎯 Applying Kubernetes manifests..."

# Create namespace first
kubectl apply -f k8s/namespace.yaml

# Apply secrets and config
kubectl apply -f k8s/postgres-secret.yaml
kubectl apply -f k8s/postgres-configmap.yaml

# Apply storage
kubectl apply -f k8s/postgres-pvc.yaml

# Deploy database
kubectl apply -f k8s/postgres-deployment.yaml
kubectl apply -f k8s/postgres-service.yaml

# Wait for PostgreSQL to be ready
echo "⏳ Waiting for PostgreSQL to be ready..."
kubectl wait --for=condition=ready pod -l app=postgres -n kryptic-journal --timeout=300s

# Run migrations
echo "🗄️ Running database migrations..."
kubectl apply -f k8s/migration-job.yaml
kubectl wait --for=condition=complete job/kryptic-journal-migrations -n kryptic-journal --timeout=300s

# Deploy API
echo "🔌 Deploying API..."
kubectl apply -f k8s/api-deployment.yaml
kubectl apply -f k8s/api-service.yaml

# Apply ingress (optional)
if [ "$1" = "--with-ingress" ]; then
    echo "🌐 Setting up ingress..."
    kubectl apply -f k8s/api-ingress.yaml
fi

# Wait for API to be ready
echo "⏳ Waiting for API to be ready..."
kubectl wait --for=condition=ready pod -l app=kryptic-journal-api -n kryptic-journal --timeout=300s

echo "✅ Deployment complete!"
echo ""
echo "📊 Checking status..."
kubectl get pods -n kryptic-journal
echo ""
echo "🔗 To access your API:"
echo "   Local: kubectl port-forward svc/kryptic-journal-api-service -n kryptic-journal 3000:80"
echo "   Then visit: http://localhost:3000/health"