#[derive(Debug)]
pub struct Processo {
    pub pid: u16,
    pub offset: u32,
    pub init_time: u32,
    pub priority: usize,
    pub time: u32,
    pub blocks: u32,
    pub printer: usize,
    pub scanner: bool,
    pub modem: bool,
    pub drive: usize,
    pub instructions: Vec<String>,
    pub state: u8
}

// state
// 0 -> criado, só pode ir para 1
// 1 -> pronto, só pode ir para 2
// 2 -> executando, pode ir para 1 ou 3
// 3 -> finalizado, não pode assumir nenhum outro valor

impl Processo {
    fn pop_instruction(&mut self) {
        self.instructions.pop();
    }

    fn get_instruction(&self) -> &String {
        self.instructions.last().unwrap()
    }

    // fn execute(&self, fs: &FileSystem) -> () {
    //     // enquanto dentro do quantum, tiver instruções, nao for preemptado (estado de execução)
    //     while instructions.len() > 0  && self.state == 1 {
    //         
    //     }
    // }
}
