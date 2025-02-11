use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signer::{keypair::Keypair, Signer},  // Mantenha o `Signer` se for usar `Signer` diretamente
    system_transaction,
    signature::read_keypair_file, // Importa a função para ler chave do arquivo
};

pub struct SolanaApp {
    pub client: RpcClient,
}

impl SolanaApp {
    /// Inicializa a conexão com a blockchain Solana (Localhost, Devnet, Testnet ou Mainnet)
    pub fn new(rpc_url: &str) -> Self {
        SolanaApp {
            client: RpcClient::new(rpc_url.to_string()),
        }
    }

    /// Consulta o saldo de uma carteira
    pub fn get_balance(&self, wallet_address: &str) {
        let pubkey = match wallet_address.parse::<Pubkey>() {
            Ok(pk) => pk,
            Err(_) => {
                println!("❌ Chave pública inválida.");
                return;
            }
        };

        match self.client.get_balance(&pubkey) {
            Ok(balance) => println!("✅ Saldo da carteira {}: {:.6} SOL", wallet_address, balance as f64 / 1_000_000_000.0),
            Err(err) => println!("❌ Erro ao obter saldo: {:?}", err),
        }
    }

    /// Solicita um airdrop de SOL para uma carteira
    pub fn airdrop(&self, wallet_address: &str, amount_sol: f64) {
        let pubkey = match wallet_address.parse::<Pubkey>() {
            Ok(pk) => pk,
            Err(_) => {
                println!("❌ Chave pública inválida.");
                return;
            }
        };

        let amount_lamports = (amount_sol * 1_000_000_000.0) as u64;

        match self.client.request_airdrop(&pubkey, amount_lamports) {
            Ok(signature) => println!("✅ Airdrop solicitado com sucesso! Assinatura: {:?}", signature),
            Err(err) => println!("❌ Erro ao solicitar airdrop: {:?}", err),
        }
    }

    /// Transferência de SOL entre contas
    pub fn transfer_sol(&self, from_keypair_path: &str, to_pubkey: &str, amount_sol: f64) {
        // Usando `read_keypair_file` para carregar a chave privada do arquivo
        let from_keypair = read_keypair_file(from_keypair_path).expect("❌ Erro ao carregar keypair do remetente");

        let to_pubkey = to_pubkey.parse::<Pubkey>().expect("❌ Chave pública do destinatário inválida");

        let amount_lamports = (amount_sol * 1_000_000_000.0) as u64;
        let recent_blockhash = self.client.get_latest_blockhash().expect("❌ Erro ao obter blockhash");

        let tx = system_transaction::transfer(&from_keypair, &to_pubkey, amount_lamports, recent_blockhash);

        match self.client.send_and_confirm_transaction(&tx) {
            Ok(signature) => println!("✅ Transferência bem-sucedida! Assinatura da transação: {:?}", signature),
            Err(err) => println!("❌ Erro ao enviar transação: {:?}", err),
        }
    }
}
