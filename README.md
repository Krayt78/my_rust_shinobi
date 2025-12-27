<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# My Rust Shinobi

A Leptos web application with Polkadot wallet integration, built with [Leptos](https://github.com/leptos-rs/leptos) and [Axum](https://github.com/tokio-rs/axum).

## Features

- **Polkadot Wallet Connection** - Connect to browser wallet extensions (Polkadot.js, Talisman, SubWallet)
- **PostgreSQL Database** - Persistent storage with sqlx and automatic migrations
- **Server-Side Rendering** - Full SSR support with Axum
- **Hot Reloading** - Fast development with cargo-leptos watch mode

---

## 🚀 Quick Start (TL;DR)

```bash
# 1. Start PostgreSQL with Docker Compose
docker compose up -d

# 2. Install dependencies
npm install

# 3. Run the full stack (frontend + backend + auto-migrations)
npm run dev

# 4. Open http://127.0.0.1:3000
```

> **Note:** Use `docker-compose up -d` if you have older Docker version.

---

## Prerequisites

Make sure you have the following installed:

| Tool | Installation | Purpose |
|------|--------------|---------|
| **Rust nightly** | `rustup toolchain install nightly --allow-downgrade` | Compile Rust code |
| **WASM target** | `rustup target add wasm32-unknown-unknown` | Compile to WebAssembly |
| **cargo-leptos** | `cargo install cargo-leptos --locked` | Build tool for Leptos |
| **Node.js & npm** | [nodejs.org](https://nodejs.org/) | Bundle wallet JavaScript |
| **Docker** | [docker.com](https://docker.com/) | Run PostgreSQL (recommended) |

## Full Setup Guide

### Step 1: Clone and Install Dependencies

```bash
# Clone the repository
git clone <your-repo-url>
cd my_rust_shinobi

# Install npm dependencies (for wallet bundling)
npm install
```

### Step 2: Set Up PostgreSQL Database

#### Option A: Using Docker Compose (Recommended)

```bash
# Start PostgreSQL (data persisted in Docker volume)
docker compose up -d

# Verify it's running
docker compose ps
```

**Docker Compose commands:**
```bash
docker compose up -d      # Start database in background
docker compose down       # Stop database (data preserved)
docker compose down -v    # Stop and delete all data
docker compose logs -f    # View logs
```

#### Option A (alt): Using Docker Run

```bash
# Start PostgreSQL container (without compose)
docker run -d \
  --name shinobi-postgres \
  -e POSTGRES_USER=shinobi \
  -e POSTGRES_PASSWORD=shinobi \
  -e POSTGRES_DB=my_rust_shinobi \
  -p 5432:5432 \
  postgres:16

# Verify it's running
docker ps | grep shinobi-postgres
```

#### Option B: Using Local PostgreSQL

```bash
# Install PostgreSQL
sudo apt install postgresql postgresql-contrib

# Start service
sudo systemctl start postgresql

# Create user and database
sudo -u postgres psql << EOF
CREATE USER shinobi WITH PASSWORD 'shinobi';
CREATE DATABASE my_rust_shinobi OWNER shinobi;
GRANT ALL PRIVILEGES ON DATABASE my_rust_shinobi TO shinobi;
EOF
```

### Step 3: Configure Environment Variables

Create a `.env` file (or copy from `.env.example`):

```bash
cp .env.example .env
```

The `.env` file should contain:

```env
# Database connection string
DATABASE_URL=postgres://shinobi:shinobi@localhost:5432/my_rust_shinobi

# Auto-run migrations on server startup
RUN_MIGRATIONS=true
```

### Step 4: Run the Application

#### Development Mode (Recommended)

```bash
npm run dev
```

This command:
1. ✅ Bundles the Polkadot wallet JavaScript (`public/wallet.js`)
2. ✅ Compiles the Rust backend (SSR)
3. ✅ Compiles the Rust frontend (WASM)
4. ✅ Connects to PostgreSQL
5. ✅ Runs database migrations
6. ✅ Starts the server with hot-reloading

#### Manual Steps (if needed)

```bash
# Step 1: Bundle wallet JavaScript
npm run build:wallet

# Step 2: Start Leptos dev server
cargo leptos watch
```

### Step 5: Open the Application

Open your browser and navigate to:

**http://127.0.0.1:3000**

You should see:
- ✅ "Welcome to Leptos!" heading
- ✅ "Connect Wallet" button
- ✅ Server logs showing: `Database connected successfully!`

---

## Available Scripts

| Command | Description |
|---------|-------------|
| `npm run dev` | Build wallet JS + start dev server with hot-reload |
| `npm run build` | Build wallet JS + production Leptos build |
| `npm run build:wallet` | Only bundle the wallet JavaScript |
| `cargo leptos watch` | Start dev server (requires wallet.js built) |
| `cargo leptos build --release` | Production build |

---

## Troubleshooting

### Database Connection Failed

```
Failed to connect to database. Make sure PostgreSQL is running and DATABASE_URL is correct.
```

**Solutions:**
1. Check if PostgreSQL is running: `docker ps` or `sudo systemctl status postgresql`
2. Verify the connection string in `.env`
3. Test connection manually: `psql postgres://shinobi:shinobi@localhost:5432/my_rust_shinobi`

### Port Already in Use

```
Reload TCP port 127.0.0.1:3001 already in use
```

**Solution:** Kill existing processes:
```bash
pkill -f "cargo-leptos"
pkill -f "my_rust_shinobi"
```

### wallet.js Not Found

**Solution:** Build the wallet bundle:
```bash
npm run build:wallet
```

---

## Project Structure

```
.
├── docker-compose.yml  # PostgreSQL database setup
├── .env                # Environment variables (DATABASE_URL, etc.)
├── .env.example        # Example environment file
├── Cargo.toml          # Rust dependencies
├── package.json        # npm scripts and JS dependencies
│
├── src/
│   ├── app.rs          # Main app component and routes
│   ├── lib.rs          # Library entry point (WASM hydration)
│   ├── main.rs         # Server entry point (Axum + database init)
│   ├── db/
│   │   ├── mod.rs      # Database module exports
│   │   ├── pool.rs     # Connection pool management
│   │   ├── models.rs   # Database models (Player, Character, etc.)
│   │   └── queries.rs  # Database queries
│   └── wallet/
│       ├── mod.rs      # Wallet bindings (Rust <-> JavaScript)
│       ├── context.rs  # Wallet state management
│       ├── components.rs   # Wallet UI components
│       └── polkadot_wallet.ts  # Polkadot.js integration
│
├── migrations/
│   └── 20241227000001_initial_schema.sql  # Database schema
│
├── public/
│   └── wallet.js       # Bundled wallet JavaScript (generated)
│
└── style/
    └── main.scss       # Global styles
```

## Wallet Connection

This project includes Polkadot wallet integration. To use it:

1. Install a Polkadot-compatible browser extension:
   - [Polkadot.js Extension](https://polkadot.js.org/extension/)
   - [Talisman](https://talisman.xyz/)
   - [SubWallet](https://subwallet.app/)

2. Click the "Connect Wallet" button in the app
3. Approve the connection in your wallet extension

### Wallet Development

The wallet JavaScript is bundled from `src/wallet/polkadot_wallet.ts` using esbuild. If you modify this file, rebuild it with:

```bash
npm run build:wallet
```

The bundled output goes to `public/wallet.js` and is served as a static asset.

## Database

The game uses PostgreSQL with sqlx for type-safe database queries.

### Database Schema

The initial migration creates these tables:

| Table | Description |
|-------|-------------|
| `players` | Player accounts linked to wallet addresses |
| `characters` | Ninja characters with stats, level, village |
| `items` | Game items (weapons, armor, consumables) |
| `inventory` | Items owned by characters |
| `skills` | Jutsu and abilities |
| `character_skills` | Skills learned by characters |
| `guilds` | Clans and organizations |
| `guild_members` | Guild membership |

### Running Migrations

Migrations run automatically on startup when `RUN_MIGRATIONS=true` is set in `.env`.

To run migrations manually:

```bash
# Using sqlx-cli
sqlx migrate run

# Or create a new migration
sqlx migrate add <migration_name>
```

### Database Models

Models are defined in `src/db/models.rs`. Example:

```rust
// Get a player by wallet address
let player = get_player_by_wallet(&pool, "5GrwvaEF...").await?;

// Create a new character
let character = create_character(&pool, &CreateCharacter {
    player_id: player.id,
    name: "Naruto".to_string(),
    village: Some("Konoha".to_string()),
}).await?;
```

## Compiling for Release

```bash
npm run build
```

Or manually:

```bash
npm run build:wallet
cargo leptos build --release
```

This will generate your server binary in `target/release` and your site package in `target/site`.

## Testing Your Project

First, install test dependencies:

```bash
cd end2end && npm install && cd ..
```

Then run the tests:

```bash
cargo leptos end-to-end
```

Or for release mode:

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.
Tests are located in `end2end/tests` directory.

## Executing a Server on a Remote Machine Without the Toolchain
After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
my_rust_shinobi
site/
```
Set the following environment variables (updating for your project as needed):
```sh
export LEPTOS_OUTPUT_NAME="my_rust_shinobi"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
export DATABASE_URL="postgres://user:password@localhost:5432/my_rust_shinobi"
export RUN_MIGRATIONS="true"
```
Finally, run the server binary.

## Licensing

This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.
