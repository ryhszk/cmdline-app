/* like the grep */
use structopt::StructOpt;
use failure::ResultExt;
use exitfailure::ExitFailure;
//use std::io::{stdout, Write};
// use std::fmt::Write;
// use std::io;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
            //println!("{}", line);
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}

fn main() -> Result<(), ExitFailure>  {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|_| format!("could not read file `{}`", args.path.display()))?;

    // for line in content.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

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

// use log::{info, warn};

// fn main() {
//     //env_logger::init();
//     info!("starting up");
//     warn!("oops, nothing implemented!");
// }

// Let's try it! 
// $ cargo run -- main src/main.rs

// #[test]
// fn find_a_match() {
//     let mut result = Vec::new();
//     find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
//     assert_eq!(result, b"lorem ipsum\n");
// }

// fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
//     for line in content.lines() {
//         if line.contains(pattern) {
//             writeln!(writer, "{}", line);
//         }
//     }
// }

// #[test]
// fn find_a_match() {
//     let mut result = Vec::new();
//     find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
//     assert_eq!(result, b"lorem ipsum\n");
// }

// fn main() -> Result<(), ExitFailure> {
//     let args = Cli::from_args();
//     let content = std::fs::read_to_string(&args.path)
//         .with_context(|_| format!("could not read file `{}`", args.path.display()))?;

//     find_matches(&content, &args.pattern, &mut std::io::stdout());

//     Ok(())
// }