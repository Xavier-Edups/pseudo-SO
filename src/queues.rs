use std::sync::Mutex;
use std::thread;

pub struct Dispatcher {
    pcb: Vec<Vec<Processo>>,
    running_process: Vec<usize>,
    ram: Mutex<RAM>,
    resources: Mutex<ResourceManager>,
    filesystem: Mutex<FileSystem>
}

// & pra emprestar
// mut pra mudar

impl Dispatcher {

    fn run(&mut self) -> (){
        let mut v = Vec::<std::thread::JoinHandle<()>>::new();

        while self.pcb[0].len() > 0 || self.pcb[1].len() > 0 || self.pcb[2].len() > 0 || self.pcb[3].len() > 0  {
            for i in 0..4 {
                for j in 0..self.pcb[i].len(){
                    let mut process = self.pcb[i][j];
                    // process already running;
                    if process.state == 2 {
                        continue
                    } else {
                        // dispatch process 
                        let (mem_available, mem_index) = self.ram.mem_available(&process.priority, &process.blocks);
                        let resources_available = self.resources.resources_available(&process);
                        if mem_available && resources_available {
                            
                            if process.state == 0 {
                                println!("DISPATCHER => Criando processo {};", &process.pid);
                            } 
                            self.ram.alloc_mem(&process.priority, &process.pid, &process.blocks, mem_index);
                            self.resources.alloc_resources(&process);

                            let handle = thread::spawn(|| {
                                // process.execute(&self.filesystem);

                                let mut tmp_ram = self.ram.lock().unwrap();
                                let mut tmp_resources = self.resources.lock().unwrap();
                                tmp_ram.dealloc_mem(&process.pid, &process.priority);
                                tmp_resources.resources.dealloc_resources(&process);

                                // process ended
                                if process.state == 3 {
                                    self.pcb[0].remove(i);
                                }
                            });
                            v.push(handle);
                        } else {
                            if process.priority == 0{
                                break;
                            } else {
                                // verificar possibilidade de preempção
                            }
                        }
                    }
                }
            }
        }
        
        for handle in v.into_iter() {
            handle.join().unwrap();
        }
    }

}