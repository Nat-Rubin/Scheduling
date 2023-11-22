use std::{fs, io, thread, time};
use std::process::{Child, Command};
use libc::pid_t;

fn pause_process(id: u32) {
    unsafe { libc::kill(id as pid_t, libc::SIGSTOP); }
}

fn continue_process(id: u32) {
    unsafe { libc::kill(id as pid_t, libc::SIGCONT); }
}
fn main() {
    let interrupt = time::Duration::from_millis(1);

    let jobs = fs::read_dir("jobs").unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().unwrap();

    let mut processes:Vec<Child> = vec![];
    let mut child: Child;
    for job in &jobs {
        // creates new child process in paused state and adds it to the vec of processes
        let job_command = format!("./{}", job.to_string_lossy().to_string());
        println!("{}", job_command);
        child = Command::new("konsole").arg("--separate").arg("--hold").arg("-e").arg(job_command).spawn().expect("Failed");
        pause_process(child.id());
        processes.push(child);
    };
    loop {
        for child in &processes {
            continue_process(child.id());
            println!("Started process {}", child.id());
            thread::sleep(interrupt);
            pause_process(child.id());
            println!("Stopped process {}", child.id());
        }
    }
}




