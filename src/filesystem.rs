#[derive(Debug, PartialEq, Eq)]
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

    fn delete_file(&mut self, pid: i32, process_priority:i32, filename: String) -> () {
        let (found, vec_index) = self.get_file_index(filename);

        if !found {
            return
        }


        if process_priority > 0 && self.fs[vec_index].file_owner != pid{
            return
        }

        // unico bloco
        if self.fs.len() == 1 {
            self.fs[vec_index].free = true;
            return;
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

            return;
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
