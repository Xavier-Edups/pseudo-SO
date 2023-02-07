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

    fn execute(&self, fs: &FileSystem, pair: &Arc<T>) -> () {
        self.state = 1;
        while true {
            let (lock, cvar) = &*pair;
            let mut preempted = lock.lock().unwrap();
            if p_counter >= self.time {
                *preempted = true;   
            }
            while *preempted {
                preempted = cvar.wait(preempted).unwrap();
            }
            std::mem::drop(mutex_process_data);
            
            if self.instructions.len() == 0 {
                return;
            } else {
                // pega instrução
                // executar instrução
            }
        }
    }
}
