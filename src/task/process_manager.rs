use alloc::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub enum ProcessKillSignal {
    RequestClose,               // Ask the process to, uhh, commit sudoku (the program may refuse).
    GracefulForcedClose,        // Let the process know about its imminent death so it can save and stuff, then kill it as soon as it completes the current running task.
    ForceClose,                 // Kill the process without any sort of warning, as soon as it completes the current running task.
    Terminate,                  // Instantly close the process, this can be dangerous
}

#[derive(Debug, Clone, Copy)]
pub enum ProcessKillError {
    RecuestRefused,             // The process kindly refused the offer to commit sudoku (the process didn't close after ProcessKillSignal::RequestClose).
    NoGracefulCloseHandler,     // The kernel can't find a way to notify the process that it will be assasinated soon.
    GracefulCloseHanderTimeout, // The process took too long to finish its GracefulCloseHandler
    ForceCloseTimeout,          // The process took too long to finish its running task
}

type ProcessId = usize;

#[derive(Debug, Clone)]
pub struct Process {
    id: ProcessId,
}

#[derive(Debug, Clone)]
pub struct ProcessManager {
    queue: Vec<Process>,
    current_id: ProcessId,
}

impl ProcessManager {
    pub fn new() -> ProcessManager {
        ProcessManager {
            queue: Vec::new(),
            current_id: 0,
        }
    }
}


// Managing the queue
impl ProcessManager {
    pub fn add_process(&mut self, process: Process) -> ProcessId {
        let id = self.current_id;
        self.current_id += 1;

        id
    }

    pub fn load_process(&mut self, data: &[u8]) -> ProcessId {
        self.add_process(todo!())
    }

    #[must_use]
    pub fn kill(&self, id: ProcessId, signal: ProcessKillSignal) -> Result<(), ProcessKillError> {
        match signal {
            ProcessKillSignal::RequestClose => todo!(),
            ProcessKillSignal::GracefulForcedClose => todo!(),
            ProcessKillSignal::ForceClose => todo!(),
            ProcessKillSignal::Terminate => Ok(()),
        }
    }
}

// Running processes
impl ProcessManager {
    pub fn run(&mut self) -> Option<ProcessRunStatistics> {
        for process in self.queue.iter_mut() {
            
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct ProcessRunStatistics {

}