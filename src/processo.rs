#[derive(Debug)]
pub struct Processo {
    pub pid: u16,
    pub offset: u32,
    pub init_time: u32,
    pub priority: usize,
    pub time: u32,
    pub blocks: u32,
    pub printer: usize,
    pub scanner: bool,
    pub modem: bool,
    pub drive: usize,
    pub instructions: Vec<String>,
    pub state: u8
}

impl Processo {
    fn pop_instruction(&mut self) {
        self.instructions.pop();
    }

    fn get_instruction(&self) -> &String {
        self.instructions.last().unwrap()
    }
}
