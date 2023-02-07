use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use thread::JoinHandle;
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

    fn mem_and_resource_available(&self, mutex_process_data: &Processo)-> (bool,usize){
        let mut ram_lock = self.ram.lock().unwrap();
        let mut resources_lock = self.resources.lock().unwrap();

        let (mem_available, mem_index) = ram_lock.mem_available(&mutex_process_data.priority, &mutex_process_data.blocks);
        let resources_available = resources_lock.resources_available(&mutex_process_data);
        return (mem_available && resources_available, mem_index)
    }

    fn mem_and_resource_allocation(&self, mutex_process_data: &Processo, mem_index: usize){
        let mut ram_lock = self.ram.lock().unwrap();
        let mut resources_lock = self.resources.lock().unwrap();
        
        ram_lock.alloc_mem(&mutex_process_data.priority, &mutex_process_data.pid, &mutex_process_data.blocks, mem_index);
        resources_lock.alloc_resources(&mutex_process_data);
    }

    fn release_blocked_process(&self, lock: &Arc<T>) -> () {
        let (lock, cvar) = &*lock;
        let mut preempted = lock.lock().unwrap();
        *preempted = false;
        cvar.notify_one();
    }

    fn process_scaling(&mut self, queue_index: usize, process_index: usize, thread_handles: &mut Vec<JoinHandle<T>>, cond_variables: &mut Vec<T>) -> () {
        let mut process = self.pcb[queue_index][process_index];
        let mut lock = process.try_lock();

        // verifica se nao esta executando
        if let Ok(mut mutex_process_data) = lock {
            let available, where_available = self.mem_and_resource_available(&mutex_process_data);
            // verifica se pode executar
            if available{
                // verifica se thread já existe
                if mutex_process_data.state == 1 {
                    self.release_blocked_process(&cond_variables[queue_index][process_index]);
                } else {
                    println!("DISPATCHER => Criando processo {};", &mutex_process_data.pid);

                    self.mem_and_resource_allocation(&mutex_process_data, where_available);
                    std::mem::drop(mutex_process_data);

                    let thread_pair = Arc::clone(&cond_variables[queue_index][process_index]);

                    let handle = thread::spawn(|| {
                        let mut thread_process = process.lock().unwrap();
                        thread_process.execute(&self.filesystem, &thread_pair);
                    });
                    thread_handles.push(handle);
                }
            } else {
                if mutex_process_data.priority == 0{
                    return;
                } else {
                    // verificar possibilidade de preempção
                }
            }
        } else {
            return
        }
    }

    fn run(&mut self) -> (){
        // Concurrency structures
        let mut thread_handles = Vec::<std::thread::JoinHandle<()>>::new();
        let mut cond_variables = Vec::new();
        for i in 0..self.pcb.len() {
            cond_variables.push(vec![]);
            for j in 0..self.pcb[i].len(){
                cond_variables[i].push(Arc::new((Mutex::new(false), Condvar::new())))
            }
        }

        while self.pcb[0].len() > 0 || self.pcb[1].len() > 0 || self.pcb[2].len() > 0 || self.pcb[3].len() > 0  {
            for i in 0..4 {
                for j in 0..self.pcb[i].len(){
                    self.process_scaling(&i, &j, &thread_handles, &cond_variables);
                }
            }
        }
        
        for handle in thread_handles.into_iter() {
            handle.join().unwrap();
        }
    }

}