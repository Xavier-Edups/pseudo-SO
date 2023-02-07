struct File {
    status: char,
    file_name: String,
    file_owner: i32,
    index: i32,
    extension: i32
}

pub struct FileSystem {
    fs: Vec<File>
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
            if self.fs[i].status == 'L' && self.fs[i].extension >= file_size{
                return (true, i);
            }
        }
        return (false, 0);
    }

    fn create_file(&mut self, pid: i32, filename: String, filesize: i32, vec_index: usize) -> () {
        let file = File{
            status: 'F',
            file_name: filename,
            file_owner: pid,
            index: self.fs[vec_index].index,
            extension: filesize
        };

        if self.fs[vec_index].extension == file.extension {
            self.fs[vec_index] = file;
        } else {
            // Assumindo que muda o objeto do vetor
            self.fs[vec_index].index += filesize;
            self.fs[vec_index].extension -= filesize;

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
            self.fs[vec_index].status = 'L';
            return;
        } else {
            // primeiro bloco com n blocos
            if vec_index == 0 {
                if self.fs[vec_index+1].status == 'L' {
                    self.merge_free_block(vec_index, vec_index+1);
                }
            // ultimo bloco com n blocos
            } else if vec_index == self.fs.len() - 1 {
                if self.fs[vec_index-1].status == 'L' {
                    self.merge_free_block(vec_index-1, vec_index);
                }
            // qualquer bloco de n blocos
            } else {
                if self.fs[vec_index-1].status == 'L' && self.fs[vec_index+1].status == 'L'{
                    self.merge_two_free_blocks(vec_index-1,vec_index+1,vec_index);
                } else if self.fs[vec_index-1].status == 'L' {
                    self.merge_free_block(vec_index-1, vec_index);
                } else if self.fs[vec_index+1].status == 'L' {
                    self.merge_free_block(vec_index, vec_index+1);
                } else {
                    self.fs[vec_index].status = 'L';
                }
            }

            return;
        }   
    }

    fn merge_free_block(&mut self, stay_block: usize, remove_block: usize) -> (){
        self.fs[stay_block].status = 'L';
        self.fs[stay_block].extension += self.fs[remove_block].extension;
        self.fs.remove(remove_block);
        return;
    }

    fn merge_two_free_blocks(&mut self, stay_block: usize, first_remove_block: usize, second_remove_block: usize) -> (){
        self.fs[stay_block].extension += self.fs[first_remove_block].extension;
        self.fs[stay_block].extension += self.fs[second_remove_block].extension;
        self.fs.remove(first_remove_block);
        self.fs.remove(second_remove_block);
        return;
    }
}