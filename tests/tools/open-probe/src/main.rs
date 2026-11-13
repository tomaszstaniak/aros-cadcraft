//! open-probe DIR: creates DIR (twice: the second call finds it there), then opens a file that is
//! not in it and prints the error. Expected on every system: kind NotFound.
use std::fs;
use std::path::PathBuf;

aros_startup::minimum_stack!(1 << 20);

fn report(what: &str, r: std::io::Result<fs::File>) -> bool {
    match r {
        Ok(_) => {
            println!("{what}: opened (unexpected)");
            false
        }
        Err(e) => {
            println!("{what}: kind {:?}, os error {:?}, text {e}", e.kind(), e.raw_os_error());
            e.kind() == std::io::ErrorKind::NotFound
        }
    }
}

fn main() -> std::process::ExitCode {
    let Some(dir) = std::env::args().nth(1).map(PathBuf::from) else {
        println!("usage: open-probe DIR");
        return std::process::ExitCode::from(10);
    };
    let missing = dir.join("no-such-file.json");
    println!("create_dir_all 1: {:?}", fs::create_dir_all(&dir));
    let a = report("open after a successful call", fs::File::open(&missing));
    println!("create_dir_all 2: {:?}", fs::create_dir_all(&dir));
    let b = report("open after create_dir_all of an existing directory", fs::File::open(&missing));
    println!("create_dir 3: {:?}", fs::create_dir(&dir).map_err(|e| (e.kind(), e.raw_os_error())));
    let c = report("open after create_dir failed with AlreadyExists", fs::File::open(&missing));
    let d = report("open for writing without create", fs::OpenOptions::new().write(true).open(&missing));
    println!("metadata: {:?}", fs::metadata(&missing).map(|_| ()).map_err(|e| (e.kind(), e.raw_os_error())));
    println!("read_to_string: {:?}", fs::read_to_string(&missing).map(|_| ()).map_err(|e| (e.kind(), e.raw_os_error())));
    if a && b && c && d {
        println!("PASS");
        std::process::ExitCode::SUCCESS
    } else {
        println!("FAIL");
        std::process::ExitCode::from(5)
    }
}
