#[derive(Debug)]
pub enum State {
    Ready,
    Running,
    Waiting,
    Terminated
}

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
    pub state: State
}

use crate::FileSystem;
use std::sync::{Arc, Mutex, Condvar};

impl Processo {
    fn pop_instruction(&mut self) {
        self.instructions.pop();
    }

    fn get_instruction(&self) -> &String {
        self.instructions.last().unwrap()
    }


    fn execute(&mut self, fs: &FileSystem, pair: &Arc<(Mutex<bool>, Condvar)>) {
        self.state = State::Ready;
        let mut p_counter = 0;
        loop {
            let (lock, cvar) = &**pair;
            let mut preempted = lock.lock().unwrap();
            if p_counter >= self.time {
                *preempted = true;   
            }
            while *preempted {
                preempted = cvar.wait(preempted).unwrap();
            }
            std::mem::drop(lock);
            
            if self.instructions.len() == 0 {
                return;
            } else {
                // pega instrução
                // executar instrução
            }

            p_counter += 1;
        }
    }

}
