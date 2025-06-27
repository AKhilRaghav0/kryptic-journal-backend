#!/bin/bash

set -e

echo "🔧 Setting up VPS for Kryptic Journal deployment..."

# Check if running as root
if [ "$EUID" -eq 0 ]; then
  echo "❌ Don't run this script as root. Run as your regular user."
  exit 1
fi

# Update system
echo "📦 Updating system packages..."
sudo apt update && sudo apt upgrade -y

# Install Docker if not present
if ! command -v docker &> /dev/null; then
    echo "🐳 Installing Docker..."
    curl -fsSL https://get.docker.com -o get-docker.sh
    sudo sh get-docker.sh
    rm get-docker.sh
    
    # Add user to docker group
    sudo usermod -aG docker $USER
    echo "✅ Docker installed. You may need to log out and back in."
else
    echo "✅ Docker already installed"
fi

# Install Docker Compose if not present
if ! command -v docker-compose &> /dev/null; then
    echo "🐙 Installing Docker Compose..."
    sudo apt install -y docker-compose
else
    echo "✅ Docker Compose already installed"
fi

# Install kubectl if not present
if ! command -v kubectl &> /dev/null; then
    echo "☸️  Installing kubectl..."
    curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
    sudo install -o root -g root -m 0755 kubectl /usr/local/bin/kubectl
    rm kubectl
else
    echo "✅ kubectl already installed"
fi

# Check if user is in docker group
if groups $USER | grep &>/dev/null '\bdocker\b'; then
    echo "✅ User is in docker group"
else
    echo "⚠️  Adding user to docker group..."
    sudo usermod -aG docker $USER
    echo "🔄 Please log out and back in for docker group changes to take effect"
fi

echo ""
echo "🎉 VPS setup complete!"
echo ""
echo "📋 Next steps:"
echo "   1. Log out and back in (if docker group was just added)"
echo "   2. Test docker: docker --version"
echo "   3. Deploy app: ./scripts/deploy.sh"
echo ""
echo "💡 For Kubernetes, you'll also need a cluster (k3s, microk8s, or managed service)" 