pub struct Dispatcher {
    pcb: Vec<Vec<Processo>>,
    running_process: Vec<usize>,
    ram: RAM,
    resources: ResourceManager
}

impl Dispatcher {

    fn run(&self) -> (){
        
        while pcb[0].len() > 0 || pcb[1].len() > 0 || pcb[2].len() > 0 || pcb[3].len() > 0  {
            if pcb[0].len() > 0 {
                for i in 0..pcb[0].len(){
                    let process = pcb[0][i];
                    if process.state == 1 {
                        continue
                    } else {
                        let mem_available, mem_index = self.ram.mem_available(process.priority, process.blocks);

                        if available {
                            println!("criando processo {} realtime", process.pid);
                            self.ram.alloc_mem(process.priority, process.pid, process.blocks, mem_index);

                        } else {
                            break;
                        }
                    }
                }
            }
        }

            // get processo da fila global
            
            // real time
                // checa recursos
                    // se ok 
                        // printa aloca e executa
                    // se não continua

            // user
                // checa recursos
                    // se ok
                        // printa aloca e executa
                    // se não
                        // checa se possivel preempção para execução
                            // se sim faz
    }

}