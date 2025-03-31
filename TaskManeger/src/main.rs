mod task;

use std::io;
use task::{Task, Status};

fn main() {
    let mut tarefas: Vec<Task> = Vec::new();
    let mut contador_id = 1; // Controla os IDs das tarefas

    loop {
        println!("\n--- Bem-vindo ao Task Manager ---");
        println!("1 - Adicionar uma nova tarefa");
        println!("2 - Listar todas as tarefas");
        println!("3 - Atualizar o status de uma tarefa pelo ID");
        println!("4 - Remover uma tarefa pelo ID");
        println!("5 - Sair");
        println!("---------------------------------");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Erro ao ler entrada");

        match opcao.trim() {
            "1" => adicionar_tarefa(&mut tarefas, &mut contador_id),
            "2" => listar_tarefas(&tarefas),
            "3" => atualizar_tarefa(&mut tarefas),
            "4" => remover_tarefa(&mut tarefas),
            "5" => {
                println!("Saindo do Task Manager...");
                break;
            }
            _ => println!("Opção inválida, tente novamente."),
        }
    }
}

// Função para adicionar uma nova tarefa
fn adicionar_tarefa(tarefas: &mut Vec<Task>, contador_id: &mut u32) {
    let mut descricao = String::new();
    println!("Digite a descrição da tarefa:");
    
    io::stdin().read_line(&mut descricao).expect("Erro ao ler entrada");
    let descricao = descricao.trim().to_string();

    let nova_tarefa = Task::new(*contador_id, descricao);
    tarefas.push(nova_tarefa);

    println!("Tarefa adicionada com sucesso! ID: {}", *contador_id);
    *contador_id += 1;
}

// Função para listar todas as tarefas
fn listar_tarefas(tarefas: &[Task]) {
    if tarefas.is_empty() {
        println!("Nenhuma tarefa cadastrada.");
        return;
    }

    println!("\n--- Lista de Tarefas ---");
    for tarefa in tarefas {
        println!(
            "ID: {} | Descrição: {} | Status: {}",
            tarefa.id, tarefa.descricao, tarefa.status.descricao()
        );
    }
}

// Função para atualizar o status de uma tarefa pelo ID
fn atualizar_tarefa(tarefas: &mut Vec<Task>) {
    let mut id_str = String::new();
    println!("Digite o ID da tarefa a ser atualizada:");

    io::stdin().read_line(&mut id_str).expect("Erro ao ler entrada");
    let id: u32 = match id_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ID inválido.");
            return;
        }
    };

    for tarefa in tarefas.iter_mut() {
        if tarefa.id == id {
            println!("Selecione o novo status:");
            println!("1 - Pendente");
            println!("2 - Em Andamento");
            println!("3 - Concluído");

            let mut status_str = String::new();
            io::stdin().read_line(&mut status_str).expect("Erro ao ler entrada");

            let novo_status = match status_str.trim() {
                "1" => Status::Pendente,
                "2" => Status::EmAndamento,
                "3" => Status::Concluido,
                _ => {
                    println!("Opção inválida.");
                    return;
                }
            };

            tarefa.atualizar_status(novo_status);
            println!("Status atualizado com sucesso!");
            return;
        }
    }

    println!("Tarefa não encontrada.");
}

// Função para remover uma tarefa pelo ID
fn remover_tarefa(tarefas: &mut Vec<Task>) {
    let mut id_str = String::new();
    println!("Digite o ID da tarefa a ser removida:");

    io::stdin().read_line(&mut id_str).expect("Erro ao ler entrada");
    let id: u32 = match id_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("ID inválido.");
            return;
        }
    };

    if let Some(pos) = tarefas.iter().position(|t| t.id == id) {
        tarefas.remove(pos);
        println!("Tarefa removida com sucesso!");
    } else {
        println!("Tarefa não encontrada.");
    }
}
