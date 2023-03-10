use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use thread::JoinHandle;
use crate::processo::State;
use crate::Processo;
use crate::RAM;
use crate::ResourceManager;
use crate::FileSystem;


pub struct Dispatcher {
    pcb: Vec<Vec<Mutex<Processo>>>,
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

    fn mem_and_resource_allocation(&self, mutex_process_data: &mut Processo, mem_index: usize){
        let mut ram_lock = self.ram.lock().unwrap();
        let mut resources_lock = self.resources.lock().unwrap();
        
        let offset = ram_lock.alloc_mem(&mutex_process_data.priority, &mutex_process_data.pid, &mutex_process_data.blocks, mem_index);
        mutex_process_data.offset = offset;
        resources_lock.alloc_resources(&mutex_process_data);
    }

    fn release_blocked_process(&self, lock: &Arc<(Mutex<bool>, Condvar)>) -> () {
        let (lock, cvar) = &*lock;
        let mut preempted = lock.lock().unwrap();
        *preempted = false;
        cvar.notify_one();
    }

    fn process_scaling(&mut self, queue_index: usize, process_index: usize, thread_handles: &mut Vec<JoinHandle<()>>, cond_variables: &mut Vec<Vec<Arc<(Mutex<bool>, Condvar)>>>) -> () {
        let mut process = self.pcb[queue_index][process_index];
        let mut lock = process.try_lock();

        // verifica se nao esta executando
        if let Ok(mut mutex_process_data) = lock {
            if mutex_process_data.state == State::Terminated{
                self.pcb[queue_index].remove(process_index);
                return;
            }
            let (available, where_available) = self.mem_and_resource_available(&mutex_process_data);
            // verifica se pode executar
            if available{
                // verifica se thread j?? existe
                if mutex_process_data.state == State::Waiting {
                    self.release_blocked_process(&cond_variables[queue_index][process_index]);
                } else {
                    mutex_process_data.print_process_create();

                    self.mem_and_resource_allocation(&mutex_process_data, where_available);
                    
                    // Porque n??o da pra transferir ownership do lock pra thread;
                    std::mem::drop(mutex_process_data);

                    let thread_pair = Arc::clone(&cond_variables[queue_index][process_index]);

                    let handle = thread::spawn(|| {
                        let mut clone_thread_pair = &thread_pair;
                        let mut clone_process = &process;
                        let mut return_type = 0;
                        while return_type > 0{
                            let mut thread_process = clone_process.lock().unwrap();
                            return_type = thread_process.execute(&self.filesystem, &clone_thread_pair);
                            std::mem::drop(thread_process);
                            if return_type > 0 {
                                let (lock, cvar) = &*clone_thread_pair;
                                let mut preempted = lock.lock().unwrap();
                                preempted = cvar.wait(preempted).unwrap();
                            }
                        }
                    });
                    thread_handles.push(handle);
                }
            } else {
                if mutex_process_data.priority == 0{
                    return;
                } else {
                    // verificar possibilidade de preemp????o
                    // verificar se tem processo de nivel menor em execu????o, que se retirado da pra botar o processo atual
                }
            }
        } else {
            return
        }
    }

    pub fn run(&mut self) -> (){
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
                // utilizado o while porque o vetor pode diminuir durante execu????o
                let mut j = 0;
                while j < self.pcb[i].len(){
                    self.process_scaling(&i, &j, &thread_handles, &cond_variables);
                }
            }
        }
        
        for handle in thread_handles.into_iter() {
            handle.join().unwrap();
        }
    }

}
