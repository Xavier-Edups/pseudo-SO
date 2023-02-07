use crate::processo::Processo;

pub struct ResourceManager {
    // true == free
    scanner: bool,
    printer:[bool; 2],
    modem:bool,
    drive:[bool; 2],
}

impl ResourceManager {

    fn resources_available(&self, process: Processo) -> bool {
        let scanner;
        let printer;
        let modem;
        let drive;        
        
        if process.scanner {
            scanner = self.scanner;
        } else {
            scanner = true;
        }

        if process.modem {
            modem = self.modem;
        } else {
            modem = true;
        }

        if process.printer > 0 {
            printer = self.printer[process.printer];
        } else {
            printer = true;
        }
        if process.drive > 0 {
            drive = self.drive[process.drive];
        } else {
            drive = true;
        }

        return scanner && printer && modem && drive; 
    }

    fn alloc_resources(&mut self, process: Processo) -> () {
        if process.scanner {
            self.scanner = false;
        }
        if process.modem {
            self.modem = false;
        }
        if process.printer > 0 {
            self.printer[process.printer] = false;
        }
        if process.drive > 0 {
            self.drive[process.drive] = false;
        }
    }

    fn dealloc_resources(&mut self, process: Processo) -> () {
        if process.scanner {
            self.scanner = true;
        }
        if process.modem {
            self.modem = true;
        }
        if process.printer > 0 {
            self.printer[process.printer] = true;
        }
        if process.drive > 0 {
            self.drive[process.drive] = true;
        }
    }
}
