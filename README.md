# senior-solana-portfolio
Senior-level Solana portfolio demonstrating domain tokenization, SPL tokens, and on-chain lending primitives.

## Structure
See the repository tree in the interview prompt. Contains three main projects:
- `doma-protocol-demo` — Anchor smart contract + Node backend + React frontend demoing DomainFi.
- `spl-token-examples` — CLI Rust examples to mint and transfer SPL tokens.
- `defi-domain-lending` — Minimal lending protocol demo wiring on-chain logic to frontend.

## Quick start (local dev)
Prereqs:
- Rust + Cargo
- Solana CLI (>=1.14)
- Anchor (Anchor CLI)
- Node.js (>=18) + npm/yarn
- Yarn recommended for frontend (but npm works)

1. Start Solana localnet:
   ```bash
   solana-test-validator
   ```
2. In `doma-protocol-demo/backend` build and deploy Anchor program:
   ```bash
   cd doma-protocol-demo/backend
   anchor build
   anchor deploy
   ```
3. Install and run Node backends and frontends:
   ```bash
   # backend
   cd doma-protocol-demo/backend
   npm install
   npm run start

   # frontend
   cd ../../doma-protocol-demo/frontend
   npm install
   npm run start
   ```
4. See sample data in `doma-protocol-demo/sample-data/domains.json`.

## Notes for interview
- The Anchor program demonstrates domain tokenization accounts and a simple lending escrow.
- Frontends use React and placeholder wallet integration (Phantom) to show interactions; replace RPC and program IDs after Anchor deploy.
- This repo is intentionally compact but demonstrates architecture, code structure, and integration points.
