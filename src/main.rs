/* like the grep */
/* 
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
*/

/* Error Report */
/*
use failure::ResultExt;
use exitfailure::ExitFailure;

fn main() -> Result<(), ExitFailure> {
    let path = "src/test.txt";
    let content = std::fs::read_to_string(path)
        .with_context(|_| format!("could not read file `{}`", path))?;
    println!("file content: {}", content);
    Ok(())
}*/


/* Progress bar */
/*
use indicatif::ProgressBar;

fn main() {
    let pb = ProgressBar::new(100);
    for i in 0..100 {
        //do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");

}
*/

use log::{info, warn};

fn main() {
    //env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
}

// Let's try it! 
// $ cargo run -- main src/main.rs
