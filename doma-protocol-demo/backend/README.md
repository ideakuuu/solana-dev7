# doma-protocol-demo backend (Anchor program)

This folder contains a compact Anchor program that:
- Initializes a `Domain` account with name and metadata.
- Mints SPL tokens representing domain units.
- Creates simple lending positions (on-chain account storing collateral + loan).

Replace `declare_id!` with a real program id after `anchor deploy`.
