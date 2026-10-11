fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        let mut stdin = std::io::stdin();
        let mut stdout = std::io::stdout();

        if let Err(e) = std::io::copy(&mut stdin, &mut stdout) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    } else {
        for file_path in args {
            let mut file = match std::fs::File::open(&file_path) {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("open {} failed, {}", file_path, e);
                    std::process::exit(1);
                }
            };

            let mut stdout = std::io::stdout();
            if let Err(e) = std::io::copy(&mut file, &mut stdout) {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }
}
