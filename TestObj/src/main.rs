use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Signature};
use solana_transaction_status::UiTransactionEncoding;
use std::io::{self, Write};
use std::str::FromStr;

fn main() {
    let rpc_url = "https://api.mainnet-beta.solana.com"; // RPC da Solana
    let client = RpcClient::new(rpc_url.to_string());

    // Solicita ao usuário um ID para analisar
    print!("🔍 Digite um ID da Solana para analisar: ");
    io::stdout().flush().unwrap(); // Garante que a mensagem aparece antes da entrada do usuário
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim(); // Remove espaços extras

    // 1️⃣ Verifica se é um PublicKey válido (Conta ou PDA)
    if let Ok(pubkey) = Pubkey::from_str(input) {
        println!("✅ O ID é um PublicKey válido: {}", pubkey);

        // 2️⃣ Verifica se a conta existe na blockchain
        match client.get_account(&pubkey) {
            Ok(account) => {
                println!("✅ Essa conta existe na Solana!");
                println!("📄 Dados da conta: {:?}", account);
                return;
            }
            Err(_) => println!("❌ Essa conta NÃO foi encontrada na blockchain."),
        }

        // 3️⃣ Verifica se é um PDA (Program Derived Address)
        let possible_programs = [
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA", // Token Program (Tokens SPL)
            "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s", // Metaplex (NFTs)
        ];

        for program_id in possible_programs.iter() {
            let program_pubkey = Pubkey::from_str(program_id).unwrap();
            let (derived_pda, _bump) = Pubkey::find_program_address(&[input.as_bytes()], &program_pubkey);

            if derived_pda == pubkey {
                println!("✅ Esse ID é um PDA derivado do programa {}", program_id);
                return;
            }
        }
    } else {
        println!("❌ O ID não é um PublicKey válido.");
    }

    // 4️⃣ Verifica se é uma assinatura de transação (Transaction Signature)
    if let Ok(signature) = Signature::from_str(input) {
        match client.get_transaction(&signature, UiTransactionEncoding::Json) {
            Ok(tx) => {
                println!("✅ O ID é uma assinatura de transação válida!");
                println!("📜 Dados da transação: {:?}", tx);
                return;
            }
            Err(_) => println!("❌ O ID não é uma assinatura de transação válida."),
        }
    } else {
        println!("❌ O ID fornecido não é uma assinatura válida.");
    }

    // 5️⃣ Verifica se existem transações associadas ao ID
    match Pubkey::from_str(input) {
        Ok(pubkey) => match client.get_signatures_for_address(&pubkey) {
            Ok(signatures) => {
                if !signatures.is_empty() {
                    println!("✅ Foram encontradas transações associadas a esse ID:");
                    for sig in &signatures {
                        println!("  🔹 Assinatura: {}", sig.signature);
                    }
                    return;
                } else {
                    println!("❌ Nenhuma transação associada a esse ID.");
                }
            }
            Err(_) => println!("❌ Não foi possível buscar transações para esse ID."),
        },
        Err(_) => println!("❌ O ID não é uma chave pública válida."),
    }

    println!("❓ O ID fornecido não se encaixa em nenhuma categoria conhecida.");
}
