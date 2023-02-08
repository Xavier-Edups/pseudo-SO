#[derive(Debug, PartialEq, Eq)]
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
        &self.instructions[0]
    }

    fn remove_instruction(&mut self) -> () {
        self.instructions.remove(0);
    }

    fn do_instruction(&self, fs: &Mutex<FileSystem>, instruction: &String) -> (){
        let t: String = instruction.chars().filter(|c| !c.is_whitespace()).collect();
        let res: Vec<String> = t.split(",").map(|s| s.to_string()).collect();
        let mut filesystem_lock = fs.lock().unwrap();
        
        // tipo da operação
        if res[1] == "0"{
            let (available, vec_index) = filesystem_lock.storage_available(res[3].parse::<i32>().unwrap());
            if available {

                filesystem_lock.create_file(&self.pid, &res[2], &res[3].parse::<i32>().unwrap(), &vec_index);
                filesystem_lock.print_block(&res[3], &vec_index);
            } else {
                return
            }
        } else {
            let deleted = filesystem_lock.delete_file(&self.pid, &self.priority, &res[2]);
            if deleted {
                filesystem_lock.print_remove_block(&res[2]);
            }
        }
    }

    fn execute(&mut self, fs: &Mutex<FileSystem>, pair: &Arc<(Mutex<bool>, Condvar)>) -> i32{
        self.state = State::Running;
        let mut p_counter = 0;
        loop {
            let (lock, cvar) = &**pair;
            let mut preempted = lock.lock().unwrap();
            if p_counter >= self.time {
                *preempted = true;   
            }
            if *preempted {
                self.state = State::Waiting;
                return 1;
            }
            std::mem::drop(preempted);
            
            if self.instructions.len() == 0 {
                self.state = State::Terminated;
                return -1;
            } else {
                println!("{} Execuntando instrução {}",self.pid, p_counter);
                let instruction = self.get_instruction();
                self.do_instruction(fs, instruction);
                self.remove_instruction();
            }

            p_counter += 1;
        }
    }

    fn print_process_create(&self) -> () {
        println!("DISPATCHER => ");
        println!("PID: {}", self.pid);
        println!("offset: {}", self.offset);
        println!("blocks: {}", self.blocks);
        println!("priority: {}", self.priority);
        println!("time: {}", self.time);
        println!("printers: {}", self.printer);
        println!("scanners: {}", self.scanner);
        println!("modems: {}", self.modem);
        println!("drives: {}", self.drive);
    }
}
