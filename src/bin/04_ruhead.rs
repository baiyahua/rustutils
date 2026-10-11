use std::io::BufRead;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        let stdin = std::io::stdin();
        let mut buf = String::new();

        for _ in 0..10 {
            if let Err(e) = stdin.read_line(&mut buf) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }

        print!("{buf}");
    } else {
        let n = args.len();

        for file_path in args {
            let file = match std::fs::File::open(&file_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            };

            if n > 1 {
                println!("==> {file_path} <==");
            }

            let mut buf = String::new();

            let mut reader = std::io::BufReader::new(file);

            for _ in 0..10 {
                if let Err(e) = reader.read_line(&mut buf) {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }

            println!("{buf}");
        }
    }
}
