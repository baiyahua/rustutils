fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 0 {
        println!("rm: missing operand");
        std::process::exit(1);
    }
    for file_path in args {
        match std::fs::metadata(&file_path) {
            Ok(metadata) => {
                if metadata.file_type().is_file() {
                    std::fs::remove_file(&file_path).unwrap();
                } else if metadata.file_type().is_dir() {
                    std::fs::remove_dir_all(&file_path).unwrap();
                }
            }
            Err(e) => {
                println!("{e}");
                std::process::exit(1);
            }
        }
    }
}
