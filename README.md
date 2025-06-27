# 🧠 Kryptic Journal Backend – Rust API for Encrypted Journaling

This is a journaling API written **entirely in Rust** using `axum`, `sqlx`, `jsonwebtoken`, and `ring`. It's built to securely store encrypted journal entries with JWT-based authentication.

## 🚀 Features

- ✅ **JWT Authentication** - Register and login with secure tokens
- ✅ **AES-256-GCM Encryption** - All journal content is encrypted at rest
- ✅ **PostgreSQL Database** - Robust data storage with migrations
- ✅ **Argon2 Password Hashing** - Secure password storage
- ✅ **RESTful API** - Clean REST endpoints for all operations
- ✅ **Structured Logging** - Built-in request tracing
- ✅ **User Isolation** - Users can only access their own entries

## 💻 Tech Stack

| Purpose            | Tool/Crate            |
| ------------------ | --------------------- |
| Web framework      | `axum`                |
| Database           | PostgreSQL via `sqlx` |
| Encryption         | `ring` (AES-256-GCM)  |
| Auth (JWT)         | `jsonwebtoken`        |
| Password hashing   | `argon2`              |
| Migrations         | `sqlx-cli`            |
| Environment config | `dotenvy`             |
| Error handling     | `thiserror`           |

## 📁 Project Structure

```
kryptic-journal-backend/
├── src/
│   ├── main.rs              # Application entry point
│   ├── routes/
│   │   ├── auth.rs          # Registration & login
│   │   └── journal.rs       # Journal CRUD operations
│   ├── db/
│   │   └── models.rs        # Database models & types
│   ├── auth/
│   │   └── jwt.rs           # JWT middleware & utils
│   └── utils/
│       └── encryption.rs    # AES encryption service
├── migrations/
│   ├── 001_create_users_table.sql
│   └── 002_create_journal_entries_table.sql
├── env.example              # Environment variables template
├── Cargo.toml
└── README.md
```

## 📌 API Endpoints

### 🔐 Authentication

| Method | Endpoint    | Description      | Auth Required |
|--------|-------------|------------------|---------------|
| POST   | `/register` | Register new user| No            |
| POST   | `/login`    | Login user       | No            |

### 📔 Journal Entries

| Method | Endpoint        | Description           | Auth Required |
|--------|-----------------|-----------------------|---------------|
| POST   | `/entries`      | Create new entry      | Yes           |
| GET    | `/entries`      | Get all user entries  | Yes           |
| GET    | `/entries/:id`  | Get specific entry    | Yes           |
| POST   | `/entries/:id`  | Update entry          | Yes           |
| DELETE | `/entries/:id`  | Delete entry          | Yes           |

### 📊 Health Check

| Method | Endpoint  | Description    | Auth Required |
|--------|-----------|----------------|---------------|
| GET    | `/health` | Service status | No            |

## 🛠️ Setup & Installation

### 🐳 Quick Start with Docker (Recommended)

**Prerequisites**: Docker and Docker Compose

```bash
# Clone and start everything
git clone <your-repo>
cd kryptic-journal-backend

# Start development environment
./scripts/dev.sh
```

The API will be available at `http://localhost:3000`

### 🚀 Production Deployment Options

#### Option 1: Docker Compose (Simple VPS)

```bash
# Production deployment
docker-compose -f docker-compose.yml up -d
```

#### Option 2: Kubernetes (Scalable VPS)

```bash
# Deploy to Kubernetes cluster
./scripts/deploy.sh

# With ingress (for external access)
./scripts/deploy.sh --with-ingress
```

### 🔧 Manual Installation (Development)

**Prerequisites**: Rust, PostgreSQL, sqlx-cli

1. **Setup environment**:
   ```bash
   cp env.example .env
   # Edit .env with your values
   ```

2. **Install dependencies**:
   ```bash
   cargo install sqlx-cli --no-default-features --features postgres
   ```

3. **Setup database**:
   ```bash
   createdb kryptic_journal
   sqlx migrate run
   ```

4. **Generate encryption key**:
   ```bash
   openssl rand -hex 32  # Copy to .env
   ```

5. **Run**:
   ```bash
   cargo run
   ```

## 🔒 Security Features

### Encryption
- **Algorithm**: AES-256-GCM with random nonces
- **Key Storage**: Environment variable (never in code)
- **Content Protection**: All journal content encrypted before database storage

### Authentication
- **JWT Tokens**: 24-hour expiration with secure secret
- **Password Hashing**: Argon2 with secure salt generation
- **Middleware Protection**: All journal routes require valid JWT

### Database Security
- **User Isolation**: Users can only access their own data
- **Prepared Statements**: All queries use sqlx parameterization
- **Foreign Key Constraints**: Ensures data integrity

## 📝 Example Usage

### Register a new user
```bash
curl -X POST http://localhost:3000/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "email": "john@example.com", 
    "password": "secure_password123"
  }'
```

### Login
```bash
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john@example.com",
    "password": "secure_password123"
  }'
```

### Create journal entry
```bash
curl -X POST http://localhost:3000/entries \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -d '{
    "title": "My First Entry",
    "content": "Today was a great day...",
    "mood_score": 8,
    "tags": ["happy", "productive"]
  }'
```

## 🐳 Docker & Kubernetes

### Docker Files Structure
```
📦 Docker Setup
├── Dockerfile              # Multi-stage production build
├── Dockerfile.migrator     # Database migration runner
├── docker-compose.yml      # Local development stack
├── .dockerignore           # Optimized builds
└── scripts/
    ├── dev.sh              # Local development
    └── deploy.sh           # Kubernetes deployment
```

### Kubernetes Manifests
```
📁 k8s/
├── namespace.yaml          # Isolated namespace
├── postgres-*.yaml         # Database deployment
├── api-*.yaml             # API deployment & service
├── api-ingress.yaml       # External access
└── migration-job.yaml     # Database migrations
```

### Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection | `postgresql://user:pass@host:5432/db` |
| `JWT_SECRET` | JWT signing key | `your-super-secure-secret` |
| `ENCRYPTION_KEY` | AES-256 key (64 hex chars) | `a1b2c3d4e5f6...` |
| `RUST_LOG` | Logging level | `info` |

### Deployment Commands

```bash
# Local development
./scripts/dev.sh

# View logs
docker-compose logs -f api

# Reset everything
docker-compose down -v

# Kubernetes deployment
./scripts/deploy.sh --with-ingress

# Check status
kubectl get pods -n kryptic-journal
```

## 🚀 Production Considerations

- **Security**: Update default secrets in `k8s/postgres-secret.yaml`
- **Scaling**: Adjust replicas in `k8s/api-deployment.yaml`
- **Storage**: Configure persistent storage class for your cluster
- **Ingress**: Update domain in `k8s/api-ingress.yaml`
- **SSL**: Set up cert-manager for automatic HTTPS certificates
- **Monitoring**: Add Prometheus metrics and health checks
- **Backup**: Configure PostgreSQL backup strategy

## 📄 License

MIT License - see LICENSE file for details 