mod lib; // Importando modulo

use std::io;
use lib::SolanaApp;

fn main(){
    let rpc_url = "http://127.0.0.1:8899";
    let app = SolanaApp::new(rpc_url);

    loop {
        println!("\n--= Solana CLI =--");
        println!("1. Ver saldo da carteira");
        println!("2. Solicitar airdrop");
        println!("3. Efetuar uma transferência");
        println!("0. Sair");
        println!("Escolha uma das opções acima: ");
        println!("");
        println!("");


        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");
        let choice = input.trim();

        match choice{
            "0" => {
                println!("Saindo...");
                break;
            }
                "1" => {
                println!("Digite a chave pública da carteira:");
                let mut address = String::new();
                io::stdin().read_line(&mut address).expect("Falha ao ler entrada");
                let address =address.trim();
                app.get_balance(address);
            }
            
            
            "2" => {
                println!("Digite a chave pública da carteira que deseja para o airdrop");

                let mut address = String::new();
                io::stdin().read_line(&mut address).expect("Falha ao ler entrada");
                

                println!("Digite a quantidade de SOL que deseja solicitar:");

                let mut amount = String::new();
                io::stdin().read_line(&mut amount).expect("Falha ao ler entrada");
                let amount: f64 =amount.trim().parse().unwrap_or(1.0);
                app.airdrop(address.trim(), amount);
                
            }
            "3" => {

                let mut address = String::new();

                let mut destination = String::new();

                let mut amount = String::new();
                

                println!("Digite a carteira que irá transeferir:");
                io::stdin().read_line(&mut address).expect("Falha ao ler entrada");

                println!("Digite a carteira que irá receber a transferência:");
                io::stdin().read_line(&mut destination).expect("Falha ao ler entrada");

                println!("Digite o valor em SOL que deseja transferir transeferir:");
                io::stdin().read_line(&mut amount).expect("Falha ao ler entrada");

                let amount: f64 = amount.trim().parse().unwrap_or(1.0);

                app.transfer_sol(&address.trim(), &destination.trim(), amount);
                    
                
            }

        _   => println!("Opção inválida, tente novamente"),
        }
    }
}