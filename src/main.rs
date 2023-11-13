use std::{fs, io, thread, time};
use std::path::PathBuf;
use std::process::{Child, Command};
use libc::pid_t;

fn pause_process(id: u32) {
    unsafe { libc::kill(id as pid_t, libc::SIGSTOP); }
}

fn continue_process(id: u32) {
    unsafe { libc::kill(id as pid_t, libc::SIGCONT); }
}
fn main() {
    let time = time::Duration::from_millis(5);
    let interrupt = time::Duration::from_millis(20);

    // let job0: [i32; 1000] = [0; 1000];
    // let job1: [i32; 1000] = [1; 1000];
    // let job2: [i32; 1000] = [2; 1000];
    // let jobs: Vec<[i32; 1000]> = vec![job0, job1, job2];

    let jobs = fs::read_dir("jobs").unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().unwrap();

    //TODO: enable args
    let mut processes:Vec<Child> = vec![];
    let mut child: Child;
    for job in &jobs {
        // creates a.txt new child process in a.txt paused state and adds it to the vec of processes
        // TODO: open in new terminal
        child = Command::new(job).spawn().expect("Failed");
        pause_process(child.id());
        processes.push(child);
    };
    loop {
        for child in &processes {
            continue_process(child.id());
            thread::sleep(interrupt);
            pause_process(child.id());
            thread::sleep(interrupt);
        }
    }
}




