// Minimal Node backend to query domain accounts and send transactions (demo purposes)
const anchor = require('@project-serum/anchor');
const { Connection, PublicKey } = require('@solana/web3.js');

const RPC = process.env.RPC || 'http://127.0.0.1:8899';
const connection = new Connection(RPC, 'confirmed');

async function main() {
  console.log('Node backend running. RPC:', RPC);
  // In a real backend you'd load the IDL and interact with the deployed program
}

main().catch(err => {
  console.error(err);
  process.exit(1);
});
