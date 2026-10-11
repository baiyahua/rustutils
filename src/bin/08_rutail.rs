use std::io::BufRead;

fn process(mut reader: impl BufRead) {
    let mut results = Vec::new();
    loop {
        let mut buf = String::new();
        match reader.read_line(&mut buf) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                results.push(buf);
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }

    let start = results.len().saturating_sub(10);
    for item in &results[start..] {
        print!("{item}");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        let stdin = std::io::stdin();
        process(std::io::BufReader::new(stdin));
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

            process(std::io::BufReader::new(file));
            println!("");
        }
    }
}
