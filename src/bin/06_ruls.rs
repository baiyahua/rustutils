use std::io::IsTerminal;

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let n = args.len();

    let terminal = std::io::stdout().is_terminal();

    if n == 0 {
        args.push(".".to_string());
    }

    for path in args {
        let dirs = match std::fs::read_dir(&path) {
            Ok(dirs) => dirs,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };

        let mut result = Vec::new();

        for item in dirs {
            match item {
                Ok(dir) => {
                    if dir.file_name().to_string_lossy().starts_with(".") {
                        continue;
                    }
                    result.push(dir);
                }
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }

        result.sort_by_key(|entry| entry.file_name());

        if n > 1 {
            println!("{path}:");
        }

        for item in result {
            if terminal && item.file_type().unwrap().is_dir() {
                print!("\x1b[34m{}\x1b[0m  ", item.file_name().display());
            } else {
                print!("{}  ", item.file_name().display());
            }
        }

        println!();
    }
}
