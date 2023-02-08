#[derive(Debug)]
pub struct RAMBlock {
    status: char,
    pid: i32,
    mem_index: i32,
    extension: i32
}

pub struct RAM {
    pub realtime_mem: Vec<RAMBlock>,
    pub user_mem: Vec<RAMBlock>
}

impl RAM {

    fn get_rt_mem_index(&mut self, pid: i32) -> (bool, usize){
        for i in 0..self.realtime_mem.len() {
            if self.realtime_mem[i].pid == pid{
                return (true, i);
            }
        }
        return (false, 0);
    }

    fn get_user_mem_index(&mut self, pid: i32) -> (bool, usize){
        for i in 0..self.user_mem.len() {
            if self.user_mem[i].pid == pid{
                return (true, i);
            }
        }
        return (false, 0);
    }

    fn get_mem_index(&mut self, pid: i32, process_priority: i32) -> (bool, usize){
        if process_priority > 0 {
            return self.get_user_mem_index(pid);
        } else {
            return self.get_rt_mem_index(pid);
        }
    }

    fn rt_mem_available(&mut self, process_b_size: i32) -> (bool, usize) {
        for i in 0..self.realtime_mem.len() {
            if self.realtime_mem[i].status == 'L' && self.realtime_mem[i].extension >= process_b_size{
                return (true, i);
            }
        }
        return (false, 0);
    }

    fn user_mem_available(&mut self, process_b_size: i32) -> (bool, usize) {
        for i in 0..self.user_mem.len() {
            if self.user_mem[i].status == 'L' && self.user_mem[i].extension >= process_b_size{
                return (true, i);
            }
        }
        return (false, 0);
    }

    fn mem_available(&mut self, process_priority: i32, process_b_size: i32) -> (bool, usize) {
        if process_priority > 0 {
            return self.user_mem_available(process_b_size);
        } else {
            return self.rt_mem_available(process_b_size);
        }
    }

    fn alloc_mem(&mut self, process_priority: i32, pid: i32, process_b_size: i32, available_mem_pos: usize) -> i32{
        if process_priority > 0 {
            return self.alloc_user_mem(pid, process_b_size, available_mem_pos);
        } else {
            return self.alloc_rt_mem(pid, process_b_size, available_mem_pos);
        }
    }

    fn alloc_rt_mem(&mut self, pid: i32, process_b_size: i32, available_mem_pos: usize) -> i32{

        if self.realtime_mem[available_mem_pos].extension == process_b_size {
            self.realtime_mem[available_mem_pos].status = 'P';
            self.realtime_mem[available_mem_pos].pid = pid;
            return self.realtime_mem[available_mem_pos].mem_index;
        } else {
            let new_block = RAMBlock{
                status: 'P',
                pid: pid,
                mem_index: self.realtime_mem[available_mem_pos].mem_index,
                extension: process_b_size,
            };

            // Assumindo que muda o objeto do vetor
            self.realtime_mem[available_mem_pos].mem_index += process_b_size;
            self.realtime_mem[available_mem_pos].extension -= process_b_size;
            
            let resultado = new_block.mem_index.clone();

            self.realtime_mem.insert(available_mem_pos, new_block);
            
            return resultado;
        }
    }

    fn alloc_user_mem(&mut self, pid: i32, process_b_size: i32, available_mem_pos: usize) -> i32{

        if self.user_mem[available_mem_pos].extension == process_b_size {
            self.user_mem[available_mem_pos].status = 'P';
            self.user_mem[available_mem_pos].pid = pid;
            return self.user_mem[available_mem_pos].mem_index;
        } else {
            let new_block = RAMBlock{
                status: 'P',
                pid: pid,
                mem_index: self.user_mem[available_mem_pos].mem_index,
                extension: process_b_size,
            };

            // Assumindo que muda o objeto do vetor
            self.user_mem[available_mem_pos].mem_index += process_b_size;
            self.user_mem[available_mem_pos].extension -= process_b_size;

            let resultado = new_block.mem_index.clone();

            self.user_mem.insert(available_mem_pos,new_block);

            return resultado;
        }
    }

    fn dealloc_rt_mem(&mut self, pid: i32) -> (){
        let (found, index) = self.get_rt_mem_index(pid);
        if !found {
            return
        }

        // unico bloco
        if self.realtime_mem.len() == 1 {
            self.realtime_mem[index].status = 'L';
            return;
        } else {
            // primeiro bloco com n blocos
            if index == 0 {
                if self.realtime_mem[index+1].status == 'L' {
                    self.merge_free_rt_block(index, index+1);
                }
            // ultimo bloco com n blocos
            } else if index == self.realtime_mem.len() - 1 {
                if self.realtime_mem[index-1].status == 'L' {
                    self.merge_free_rt_block(index-1, index);
                }
            // qualquer bloco de n blocos
            } else {
                if self.realtime_mem[index-1].status == 'L' && self.realtime_mem[index+1].status == 'L'{
                    self.merge_two_free_rt_blocks(index-1,index+1,index);
                } else if self.realtime_mem[index-1].status == 'L' {
                    self.merge_free_rt_block(index-1, index);
                } else if self.realtime_mem[index+1].status == 'L' {
                    self.merge_free_rt_block(index, index+1);
                } else {
                    self.realtime_mem[index].status = 'L';
                }
            }

            return;
        }
    }

    fn dealloc_user_mem(&mut self, pid: i32) -> (){
        let (found, index) = self.get_user_mem_index(pid);
        if !found {
            return
        }

        // unico bloco
        if self.user_mem.len() == 1 {
            self.user_mem[index].status = 'L';
            return;
        } else {
            // primeiro bloco com n blocos
            if index == 0 {
                if self.user_mem[index+1].status == 'L' {
                    self.merge_free_user_block(index, index+1);
                }
            // ultimo bloco com n blocos
            } else if index == self.user_mem.len() - 1 {
                if self.user_mem[index-1].status == 'L' {
                    self.merge_free_user_block(index-1, index);
                }
            // qualquer bloco de n blocos
            } else {
                if self.user_mem[index-1].status == 'L' && self.user_mem[index+1].status == 'L'{
                    self.merge_two_free_user_blocks(index-1,index+1,index);
                } else if self.user_mem[index-1].status == 'L' {
                    self.merge_free_user_block(index-1, index);
                } else if self.user_mem[index+1].status == 'L' {
                    self.merge_free_user_block(index, index+1);
                } else {
                    self.user_mem[index].status = 'L';
                }
            }

            return;
        }
    }

    fn dealloc_mem(&mut self, pid: i32, process_priority: i32) -> (){
        if process_priority > 0 {
            self.dealloc_user_mem(pid);
        } else {
            self.dealloc_rt_mem(pid);
        }
    }

    fn merge_free_rt_block(&mut self, stay_block: usize, remove_block: usize) -> (){
        self.realtime_mem[stay_block].status = 'L';
        self.realtime_mem[stay_block].extension += self.realtime_mem[remove_block].extension;
        self.realtime_mem.remove(remove_block);
        return;
    }

    fn merge_free_user_block(&mut self, stay_block: usize, remove_block: usize) -> (){
        self.user_mem[stay_block].status = 'L';
        self.user_mem[stay_block].extension += self.user_mem[remove_block].extension;
        self.user_mem.remove(remove_block);
        return;
    }

    fn merge_two_free_rt_blocks(&mut self, stay_block: usize, first_remove_block: usize, second_remove_block: usize) -> (){
        self.realtime_mem[stay_block].extension += self.realtime_mem[first_remove_block].extension;
        self.realtime_mem[stay_block].extension += self.realtime_mem[second_remove_block].extension;
        self.realtime_mem.remove(first_remove_block);
        self.realtime_mem.remove(second_remove_block);
    }

    fn merge_two_free_user_blocks(&mut self, stay_block: usize, first_remove_block: usize, second_remove_block: usize) -> (){
        self.user_mem[stay_block].extension += self.user_mem[first_remove_block].extension;
        self.user_mem[stay_block].extension += self.user_mem[second_remove_block].extension;
        self.user_mem.remove(first_remove_block);
        self.user_mem.remove(second_remove_block);
    }

    fn preemptavel(&mut self, pid: i32, new_p_size: i32) -> bool {
        for i in 0..self.user_mem.len(){
            if self.user_mem[i].pid == pid{
                let mut antes = 0;
                let mut depois = 0;
                if i != 0{
                    if self.user_mem[i-1].status == 'L'{
                        antes = self.user_mem[i-1].extension;
                    }
                } else if i == self.user_mem.len()-1{
                    if self.user_mem[i+1].status == 'L'{
                        depois = self.user_mem[i+1].extension;
                    }
                }
                return antes+depois+self.user_mem[i].extension > new_p_size;
            } else {
                continue;
            }
        }
        return false
    }
}
