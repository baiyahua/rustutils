use std::io::BufRead;

fn process(mut reader: impl BufRead, results: &mut Vec<String>) {
    loop {
        let mut buf = String::new();
        let n = reader.read_line(&mut buf).unwrap();
        if n == 0 {
            break;
        }
        if buf.ends_with('\n') {
            buf.pop();
        }
        results.push(buf);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let mut results = Vec::new();

    if args.len() == 0 {
        let stdin = std::io::stdin();
        let reader = std::io::BufReader::new(stdin);
        process(reader, &mut results);
    } else {
        for file_path in args {
            let file = match std::fs::File::open(&file_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(1);
                }
            };

            let reader = std::io::BufReader::new(file);
            process(reader, &mut results);
        }
    }

    results.sort();

    for item in results {
        println!("{item}");
    }
}
