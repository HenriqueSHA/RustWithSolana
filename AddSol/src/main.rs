use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program,
    system_transaction,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{

    //configurando o cliente para a devnet

    let rpc_url = "https://api.devnet.solana.com";
    let rpc_client = RpcClient::new(rpc_url);

    //gerando keypair nova
    
    let keypair = Keypair::new();
    let public_key = keypair.pubkey();

    println!("Nova conta criada");
    println!("Endereço público: {}", public_key);

    //solicitando SOL da faucet na devnet
    let amount_in_lamports = 2_000_000_00; //1 SOL = 1_000_000_000 lamports   2_000_000_00
    rpc_client.request_airdrop(&public_key, amount_in_lamports)?;

    println!("Solicitção de {} lamports enviada para o endereço {}",
    amount_in_lamports, public_key);

    Ok(())
}
