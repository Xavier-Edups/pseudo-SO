use std::env;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use crate::processo::Processo;

mod processo;

// Dispatcher
fn main() {
    let files: Vec<String> = env::args().collect(); // Le os argumentos da linha de comando
    dbg!(&files);
    check_files(&files);
    create_processes(&files[1]);
    load_instructions(&files[2]);
    //loop {}
}

fn check_files(files: &Vec<String>) { // Checa validade dos arquivos
    // Verifica se o número de argumentos é o correto
    match &files.len() {
        1 => panic!("Not enough arguments... Usage: ./dispatcher <file1name>.txt <file2name>.txt"),
        2 => panic!("Not enough arguments... Usage: ./dispatcher <file1name>.txt <file2name>.txt"),
        3 => (),
        _ => panic!("Too many arguments! Usage: ./dispatcher <file1name>.txt <file2name>.txt")
    };

    // Verifica se os arquivos existem
    let f1 = Path::new(&files[1]);
    let f2 = Path::new(&files[2]);
    match (&f1.exists(), &f2.exists()) {
        (true, true) => (),
        _ => panic!("Make sure both files exist!")
    };

    // Verifica se os arquivos são do tipo txt
    match (f1.extension().and_then(OsStr::to_str), f2.extension().and_then(OsStr::to_str)) {
        (Some("txt"), Some("txt")) => (),
        _ => panic!("Both files must have the format <filename>.txt")
    };
}

fn create_processes(file_str: &String) {
    // Le arquivo e guarda informação dos processos
    let f1 = Path::new(file_str);
    let mut raw_data = fs::read_to_string(f1).unwrap();
    raw_data.pop();
    let process_info: Vec<&str> = raw_data.split('\n').collect();
    dbg!(&process_info);

    // Cria vetor de processos e popula ele
    let mut processos: Vec<Processo> = Vec::new();
    let mut i = 0;
    while i < process_info.len() {
        let values: Vec<&str> = process_info[i].split(", ").collect();
        //dbg!(&values);
        let p = Processo {
            pid: i as u16,
            offset: if i != 0 { processos[i-1].blocks + 1 } else { 0 },
            init_time: values[0].parse::<u32>().unwrap(),
            priority: values[1].parse::<u8>().unwrap(),
            time: values[2].parse::<u32>().unwrap(),
            blocks: values[3].parse::<u32>().unwrap(),
            printer: values[4].parse::<u8>().unwrap(),
            scanner: values[5].parse::<u8>().unwrap() != 0,
            modem: values[6].parse::<u8>().unwrap() != 0,
            drive: values[7].parse::<u8>().unwrap(),
            instructions: Vec::new()
        };

        processos.push(p); // Coloca processo criado na lista de processos

        i += 1;
    }
    dbg!(&processos);
}

fn load_instructions(file_str: &String) {}

fn populate_memmory() {}
