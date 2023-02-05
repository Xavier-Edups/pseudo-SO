struct RAMBlock {
    status: char,
    pid: i32,
    mem_index: i32,
    extension: i32,
}

struct RAM {
    realtime_mem: Vec<RAMBlock>,
    user_mem: Vec<RAMBlock>,
}

impl RAM {

    fn get_mem_index(&self, pid: i32, process_priority: i32) -> (bool, usize){
        let memory_blocks = get_mem(process_priority);
        for i in 0..memory_blocks.len() {
            if memory_blocks[i].pid == pid{
                return (true, i);
            }
        }
        return (false, 0);
    }

    fn mem_available(&self, process_priority: i32, process_b_size: i32) -> (bool, usize) {
        let memory_blocks;
        if process_priority > 0 {
            memory_blocks = self.user_mem;
        } else {
            memory_blocks = self.realtime_mem;
        }
        
        for i in 0..memory_blocks.len() {
            if memory_blocks[i].status == 'L' && memory_blocks[i].extension >= process_b_size{
                return (true, i);
            }
        }
        return (false, 0);
    }

    fn alloc_mem(&self, process_priority: i32, pid: i32, process_b_size: i32, available_mem_pos: usize) -> (){
        let memory_blocks;
        if process_priority > 0 {
            memory_blocks = self.user_mem;
        } else {
            memory_blocks = self.realtime_mem;
        }
        
        let block = memory_blocks[available_mem_pos];
        if block.extension == process_b_size {
            block.status = 'P';
            block.pid = pid;
        } else {
            let new_block = RAMBlock{
                status: 'P',
                pid: pid,
                mem_index: block.mem_index,
                extension: process_b_size,
            };

            // Assumindo que muda o objeto do vetor
            block.mem_index += process_b_size;
            block.extension -= process_b_size;

            memory_blocks.insert(available_mem_pos,new_block);
        }
    }

    fn get_mem(&self, process_priority: i32) {
        let memory_blocks;
        if process_priority > 0 {
            memory_blocks = self.user_mem;
        } else {
            memory_blocks = self.realtime_mem;
        }
        return memory_blocks;
    }

    fn dealloc_mem(&self, pid: i32, process_priority: i32) -> (){

        let found, index = get_mem_index(pid, process_priority);
        if !found {
            return
        }
        let memory_blocks = get_mem(process_priority);
        // unico bloco
        if memory_blocks.len() == 1 {
            memory_blocks[index].status = 'L';
            return;
        } else {
            // primeiro bloco com n blocos
            if index == 0 {
                if memory_blocks[index+1].status == 'L' {
                    self.merge_free_block(index, index+1, process_priority);
                }
            // ultimo bloco com n blocos
            } else if index == memory_blocks.len() - 1 {
                if memory_blocks[index-1].status == 'L' {
                    self.merge_free_block(index-1, index, process_priority);
                }
            // qualquer bloco de n blocos
            } else {
                if memory_blocks[index-1].status == 'L' && memory_blocks[index+1].status == 'L'{
                    self.merge_two_free_blocks(index-1,index+1,index, process_priority);
                } else if memory_blocks[index-1].status == 'L' {
                    self.merge_free_block(index-1, index, process_priority);
                } else if memory_blocks[index+1].status == 'L' {
                    self.merge_free_block(index, index+1, process_priority);
                } else {
                    memory_blocks[index].status = 'L';
                }
            }

            return;
        }
    }

    fn merge_free_block(&self, stay_block: usize, remove_block: usize, process_priority: i32) -> (){
        let memory_blocks = self.get_mem(process_priority);

        memory_blocks[stay_block].status = 'L';
        memory_blocks[stay_block].extension += memory_blocks[remove_block].extension;
        memory_blocks.remove(remove_block);
        return;
    }

    fn merge_two_free_blocks(&self, stay_block: usize, first_remove_block: usize, second_remove_block: usize, process_priority: i32) -> (){
        let memory_blocks = self.get_mem(process_priority);

        memory_blocks[stay_block].extension += memory_blocks[first_remove_block].extension;
        memory_blocks[stay_block].extension += memory_blocks[second_remove_block].extension;
        memory_blocks.remove(first_remove_block);
        memory_blocks.remove(second_remove_block);
        return;
    }
}