fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 0 {
        println!("rumkdir: missing operand");
        std::process::exit(1);
    }

    for path in args {
        if let Err(e) = std::fs::create_dir_all(&path) {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
