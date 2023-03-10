use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::env;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use crate::processo::Processo;
use crate::processo::State;
use crate::mem_ram::RAM;
use crate::filesystem::FileSystem;
use crate::filesystem::File;
use crate::filesystem::Block;
use crate::resources::ResourceManager;
use crate::queues::Dispatcher;

mod processo;
mod mem_ram;
mod filesystem;
mod resources;
mod queues;

// Recursos Globais
static mut RAM: RAM = RAM {
    realtime_mem: Vec::new(),
    user_mem: Vec::new()
};
static mut DISK: FileSystem = FileSystem { fs: Vec::new(), blocks: Vec::new() };

// Dispatcher
fn main() {
    let files: Vec<String> = env::args().collect(); // Le os argumentos da linha de comando
    dbg!(&files);
    check_files(&files);
    let mut processos = create_processes(&files[1]);
    load_instructions(&files[2], &mut processos);
    dbg!(&processos);


    let mut resource_manager = ResourceManager{
        scanner: true,
        printer: [true, true],
        modem: true,
        drive: [true, true]
    };
    let mut dispatcher = Dispatcher{
        pcb:Mutex::new(0_u32),
        ram:Mutex::new(0_u32),
        resources:Mutex::new(resource_manager),
        filesystem:Mutex::new(0_u32)
    };

    dispatcher.run();
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

fn create_processes(file_str: &String) -> Vec<Vec<Processo>> {
    // Le arquivo e guarda informação dos processos
    let f1 = Path::new(file_str);
    let mut raw_data = fs::read_to_string(f1).unwrap();
    match raw_data.chars().last() {
        None => panic!("Erro na leitura do arquivo"),
        Some('\n') => raw_data.pop(),
        Some(c) => Some(c),
    };
    let process_info: Vec<&str> = raw_data.split('\n').collect();
    dbg!(&process_info);

    // Cria vetor de processos e popula ele
    let mut processos: Vec<Vec<Processo>> = Vec::new();
    for _ in 0..4{
        processos.push(vec![])
    }

    let mut i = 0;
    while i < process_info.len() {
        let values: Vec<&str> = process_info[i].split(", ").collect();
        unsafe {
            let priority = values[1].parse::<usize>().unwrap();
            let offset = if i != 0 {
                if !processos[priority].is_empty() {
                    processos[priority][processos[priority].len()-1].blocks +
                    processos[priority][processos[priority].len()-1].offset + 1
                } else { 0 }
            } else { 0 };
            let p = Processo {
                pid: i as u16,
                offset: offset as u32,
                init_time: values[0].parse::<u32>().unwrap(),
                priority,
                time: values[2].parse::<u32>().unwrap(),
                blocks: values[3].parse::<u32>().unwrap(),
                printer: values[4].parse::<usize>().unwrap(),
                scanner: values[5].parse::<u8>().unwrap() != 0,
                modem: values[6].parse::<u8>().unwrap() != 0,
                drive: values[7].parse::<usize>().unwrap(),
                instructions: Vec::new(),
                state: State::Ready
            };
            if RAM.mem_available(p.priority as i32, p.blocks as i32).0 {
               println!("Não foi possível criar o processo {}. Memória insuficiente.", p.pid);
               continue;
            }
            RAM.alloc_mem(p.priority as i32, p.pid as i32, p.blocks as i32, processos[p.priority].len() as usize);

            processos[p.priority].push(p); // Coloca processo criado na lista de processos
        }

        i += 1;
    }

    processos
}

fn load_instructions(file_str: &String, processos: &mut Vec<Vec<Processo>>) {
    // Le segundo arquivo e guarda informações
    let f2 = Path::new(file_str);
    let mut raw_data = fs::read_to_string(f2).unwrap();
    match raw_data.chars().last() {
        None => panic!("Erro na leitura do arquivo"),
        Some('\n') => raw_data.pop(),
        Some(c) => Some(c),
    };
    let mut init_info: Vec<&str> = raw_data.split('\n').collect();
    dbg!(&init_info);

    // Define configurações iniciais do sistema
    let disk_size = init_info.remove(0).parse::<usize>().unwrap();
    create_disk(disk_size);
    let file_count = init_info.remove(0).parse::<usize>().unwrap();
    let mut i = 0;
    while i < file_count {
        populate_disk(init_info.remove(0).to_string());
        i += 1;
    }
    unsafe { dbg!(&DISK); }

    // Carrega instruções nos processos
    for instrucao in init_info {
        let inst_id = instrucao.split(", ").collect::<Vec<&str>>()[0];
        for fila in &mut *processos {
            for p in fila {
                if p.pid == inst_id.parse::<u16>().unwrap() {
                    p.instructions.push(instrucao.to_string());
                }
            }
        }
    }
}

fn create_disk(disk_size: usize) {
    unsafe {
        DISK.fs.push(File {
            free: true,
            file_name: "Disk Free".to_string(),
            file_owner: 1001,
            index: 0,
            size: disk_size as i32
        });
    }
    for _i in 0..disk_size {
        unsafe { DISK.blocks.push(Block::Free) }
    }
}

fn populate_disk(config: String) {
    let args: Vec<&str> = config.split(", ").collect();
    let file_name = args[0];
    let index = args[1].parse::<usize>().unwrap();
    let size = args[2].parse::<usize>().unwrap();
    unsafe {
        for i in index..index+size {
            if DISK.blocks[i] == Block::Occupied {
                println!("Não foi possível criar arquivo {file_name} no espaço de disco desejado.");
                return;
            }
        }

        for j in index..index+size {
            DISK.blocks[j] = Block::Occupied;
        }
        DISK.fs[0].size -= size as i32;
        DISK.fs.push(File {
            free: false,
            file_name: file_name.to_string(),
            file_owner: 1001,
            index: index as i32,
            size: size as i32
        });
    }
}
