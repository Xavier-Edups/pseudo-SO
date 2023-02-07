use std::sync::Mutex;
use std::thread;

pub struct Dispatcher {
    pcb: Vec<Vec<Mutex<Processo>>>,
    running_process: Vec<usize>,
    ram: Mutex<RAM>,
    resources: Mutex<ResourceManager>,
    filesystem: Mutex<FileSystem>
}

// & pra emprestar
// mut pra mudar

impl Dispatcher {

    fn mem_and_resource_available(&self, mutex_process_data: &Processo)->bool{
        let mut ram_lock = self.ram.lock().unwrap();
        let mut resources_lock = self.resources.lock().unwrap();

        let (mem_available, mem_index) = ram_lock.mem_available(&mutex_process_data.priority, &mutex_process_data.blocks);
        let resources_available = resources_lock.resources_available(&mutex_process_data);
        return mem_available && resources_available
    }

    fn mem_and_resource_allocation(&self, mutex_process_data: &Processo){
        let mut ram_lock = self.ram.lock().unwrap();
        let mut resources_lock = self.resources.lock().unwrap();
        
        ram_lock.alloc_mem(&mutex_process_data.priority, &mutex_process_data.pid, &mutex_process_data.blocks, mem_index);
        resources_lock.alloc_resources(&mutex_process_data);
    }

    fn process_scaling(&mut self, thread_handles: &mut Vec<JoinHandle>) -> () {
        let mut process = self.pcb[i][j];
        let mut lock = process.try_lock();

        if let Ok(mut mutex_process_data) = lock {
            if  self.mem_and_resource_available(&mutex_process_data){
                if mutex_process_data.state == 0 {
                    println!("DISPATCHER => Criando processo {};", &mutex_process_data.pid);
                } 

                self.mem_and_resource_allocation(&mutex_process_data);

                let handle = thread::spawn(|| {
                    // process.execute(&self.filesystem);
                    // let mut tmp_ram = self.ram.lock().unwrap();
                    // let mut tmp_resources = self.resources.lock().unwrap();
                    // tmp_ram.dealloc_mem(&mutex_process_data.pid, &mutex_process_data.priority);
                    // tmp_resources.resources.dealloc_resources(&mutex_process_data);
                });
                thread_handles.push(handle);
            } else {
                if mutex_process_data.priority == 0{
                    break;
                } else {
                    // verificar possibilidade de preempção
                }
            }
        } else {
            // thread in possesion of lock;
            return
        }
    }

    fn run(&mut self) -> (){
        let mut v = Vec::<std::thread::JoinHandle<()>>::new();

        while self.pcb[0].len() > 0 || self.pcb[1].len() > 0 || self.pcb[2].len() > 0 || self.pcb[3].len() > 0  {
            for i in 0..4 {
                for j in 0..self.pcb[i].len(){
                    self.process_scaling(&v);
                }
            }
        }
        
        for handle in v.into_iter() {
            handle.join().unwrap();
        }
    }

}