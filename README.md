<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# My Rust Shinobi

A Leptos web application with Polkadot wallet integration, built with [Leptos](https://github.com/leptos-rs/leptos) and [Axum](https://github.com/tokio-rs/axum).

## Features

- **Polkadot Wallet Connection** - Connect to browser wallet extensions (Polkadot.js, Talisman, SubWallet)
- **Server-Side Rendering** - Full SSR support with Axum
- **Hot Reloading** - Fast development with cargo-leptos watch mode

## Prerequisites

Make sure you have the following installed:

1. **Rust nightly**: `rustup toolchain install nightly --allow-downgrade`
2. **WASM target**: `rustup target add wasm32-unknown-unknown`
3. **cargo-leptos**: `cargo install cargo-leptos --locked`
4. **Node.js & npm**: Required for bundling wallet JavaScript
5. **Sass** (optional): `npm install -g sass`

## Setup

Install dependencies:

```bash
npm install
```

## Running your project

The recommended way to run the project (builds wallet JS + starts Leptos):

```bash
npm run dev
```

Or manually:

```bash
npm run build:wallet  # Bundle the wallet JavaScript
cargo leptos watch    # Start the dev server
```

Then open http://127.0.0.1:3000 in your browser.

## Project Structure

```
src/
├── app.rs              # Main app component and routes
├── lib.rs              # Library entry point (WASM hydration)
├── main.rs             # Server entry point (Axum)
└── wallet/
    ├── mod.rs          # Wallet bindings (Rust <-> JavaScript)
    ├── context.rs      # Wallet state management
    ├── components.rs   # Wallet UI components
    └── polkadot_wallet.ts  # Polkadot.js integration (TypeScript)

public/
└── wallet.js           # Bundled wallet JavaScript (generated)

style/
└── main.scss           # Global styles
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
```
Finally, run the server binary.

## Licensing

This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.
