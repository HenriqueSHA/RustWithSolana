#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Pendente,
    EmAndamento,
    Concluido,
}

impl Status {
    pub fn descricao(&self) -> &str {
        match self {
            Status::Pendente => "Aguardando início.",
            Status::EmAndamento => "Tarefa em progresso.",
            Status::Concluido => "Tarefa finalizada!",
        }
    }
}

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub descricao: String,
    pub status: Status,
}

impl Task {
    pub fn new(id: u32, descricao: String) -> Self {
        Task {
            id,
            descricao,
            status: Status::Pendente,
        }
    }

    pub fn atualizar_status(&mut self, novo_status: Status) {
        self.status = novo_status;
    }
}
