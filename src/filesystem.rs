#[derive(Debug, PartialEq, Eq, Display)]
pub enum Block {
    Free,
    Occupied
}

#[derive(Debug)]
pub struct File {
    pub free: bool,
    pub file_name: String,
    pub file_owner: i32,
    pub index: i32,
    pub size: i32
}

#[derive(Debug)]
pub struct FileSystem {
    pub fs: Vec<File>,
    pub blocks: Vec<Block>
}

impl FileSystem {

    fn print_filesystem(&self) -> () {
        println!("Mapa de ocupação de disco: ");
        for i in 0..self.blocks {
            println!("{}", self.blocks[i]);
        }
    }

    fn print_block(&mut self, quantidade: String, vec_index: usize) -> (){
        let blocks_index = self.fs[vec_index].index;
        let quantidade_int = quantidade.parse::<i32>().unwrap();
        for i in index..index+quantidade_int {
            self.blocks[i] = Block::Occupied;
        }
    }

    fn print_remove_block(&mut self, nome: String) -> {
        let mut block_index;
        let mut block_size
        for i in 0..self.fs.len(){
            if self.fs[i].file_name == nome {
                block_index = self.fs[i].index;
                block_size = self.fs[i].size;
            }
        }
        for i in block_index..block_index+block_size{
            self.blocks[i] = Block::Free;
        }
    }

    fn get_file_index(&self, filename: String) -> (bool, usize) {
        for i in 0..self.fs.len() {
            if self.fs[i].file_name == filename {
                return (true, i);
            }
        }
        return (false, 0);
    }

    fn storage_available(&self, file_size: i32) -> (bool, usize) {        
        for i in 0..self.fs.len() {
            if self.fs[i].free && self.fs[i].size >= file_size{
                return (true, i);
            }
        }
        return (false, 0);
    }

    fn create_file(&mut self, pid: i32, filename: String, filesize: i32, vec_index: usize) -> () {
        let file = File{
            free: true,
            file_name: filename,
            file_owner: pid,
            index: self.fs[vec_index].index,
            size: filesize
        };

        if self.fs[vec_index].size == file.size {
            self.fs[vec_index] = file;
        } else {
            // Assumindo que muda o objeto do vetor
            self.fs[vec_index].index += filesize;
            self.fs[vec_index].size -= filesize;

            self.fs.insert(vec_index,file);
        }
    }

    fn delete_file(&mut self, pid: i32, process_priority:i32, filename: String) -> bool {
        let (found, vec_index) = self.get_file_index(filename);

        if !found {
            return false
        }


        if process_priority > 0 && self.fs[vec_index].file_owner != pid{
            return false
        }

        // unico bloco
        if self.fs.len() == 1 {
            self.fs[vec_index].free = true;
            return true;
        } else {
            // primeiro bloco com n blocos
            if vec_index == 0 {
                if self.fs[vec_index+1].free {
                    self.merge_free_block(vec_index, vec_index+1);
                }
            // ultimo bloco com n blocos
            } else if vec_index == self.fs.len() - 1 {
                if self.fs[vec_index-1].free {
                    self.merge_free_block(vec_index-1, vec_index);
                }
            // qualquer bloco de n blocos
            } else {
                if self.fs[vec_index-1].free && self.fs[vec_index+1].free{
                    self.merge_two_free_blocks(vec_index-1,vec_index+1,vec_index);
                } else if self.fs[vec_index-1].free {
                    self.merge_free_block(vec_index-1, vec_index);
                } else if self.fs[vec_index+1].free {
                    self.merge_free_block(vec_index, vec_index+1);
                } else {
                    self.fs[vec_index].free = true;
                }
            }

            return true;
        }   
    }

    fn merge_free_block(&mut self, stay_block: usize, remove_block: usize) -> (){
        self.fs[stay_block].free = true;
        self.fs[stay_block].size += self.fs[remove_block].size;
        self.fs.remove(remove_block);
        return;
    }

    fn merge_two_free_blocks(&mut self, stay_block: usize, first_remove_block: usize, second_remove_block: usize) -> (){
        self.fs[stay_block].size += self.fs[first_remove_block].size;
        self.fs[stay_block].size += self.fs[second_remove_block].size;
        self.fs.remove(first_remove_block);
        self.fs.remove(second_remove_block);
        return;
    }
}
