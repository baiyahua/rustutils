use std::io::BufRead;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        let mut line = 0;
        let mut word = 0;
        let mut bytes = 0;

        let stdin = std::io::stdin();

        let mut buf = String::new();

        loop {
            buf.clear();

            match stdin.read_line(&mut buf) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }

                    bytes += n;

                    word += buf.split_whitespace().count();

                    if buf.ends_with('\n') {
                        line += 1;
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }

        println!("{:>8} {:>8} {:>8}", line, word, bytes);
    } else {
        let mut total_line = 0;
        let mut total_word = 0;
        let mut total_bytes = 0;

        let n = args.len();

        for file_path in args {
            let file = match std::fs::File::open(&file_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            };

            let mut reader = std::io::BufReader::new(file);
            let mut buf = String::new();

            let mut line = 0;
            let mut word = 0;
            let mut bytes = 0;

            loop {
                buf.clear();

                match reader.read_line(&mut buf) {
                    Ok(n) => {
                        if n == 0 {
                            break;
                        }

                        bytes += n;
                        word += buf.split_whitespace().count();
                        if buf.ends_with('\n') {
                            line += 1;
                        }
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(1);
                    }
                }
            }
            println!("{line:>4} {word:>4} {bytes:>4} {file_path}");

            total_line += line;
            total_word += word;
            total_bytes += bytes;
        }

        if n > 1 {
            println!("{total_line:>4} {total_word:>4} {total_bytes:>4} total");
        }
    }
}
