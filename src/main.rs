use std::env;
use std::process;

use minigrep::ConfigFile;
use minigrep::ConfigString;
use minigrep::ConfigType;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    // print!("\n{}\n",args.get(1).unwrap());
    if args.len() < 2 {
        print_help();
        return;
    }
    let config = if args[1] == "--f" {
        let file_config = ConfigFile::new(&args).unwrap_or_else(|err| {
            println!("Problem passing the arguments : {err}");
            process::exit(1);
        });
        ConfigType::File(file_config)
    } else {
        let string_config = ConfigString::new(&args).unwrap_or_else(|err| {
            println!("Problem passing the arguments : {err}");
            process::exit(1);
        });
        ConfigType::String(string_config)
    };
    if let Err(e) = run(config) {
        eprintln!("Error reading the file : {e}");
        process::exit(1);
    }
}

fn print_help() {
    println!(
        " 
\t\tminigrep in rust\n
 Help: use --f [File path] for files or directly input the query and string \n
 format: minigrep query string -> finds query in string 
 format: minigrep --f  query [File Path] -> finds query in string 
"
    )
}
