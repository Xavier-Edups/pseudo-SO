#[derive(Debug)]
pub struct Processo {
    pub pid: u16,
    pub offset: u32,
    pub init_time: u32,
    pub priority: usize,
    pub time: u32,
    pub blocks: u32,
    pub printer: u8,
    pub scanner: bool,
    pub modem: bool,
    pub drive: u8,
    pub instructions: Vec<String>
}

impl Processo {
    fn pop_instruction(&mut self) {
        self.instructions.pop();
    }

    fn get_instruction(&self) -> &String {
        self.instructions.last().unwrap()
    }
}
