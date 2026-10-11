fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let n = args.len();

    if n != 2 {
        println!("cp: missing file operand");
        std::process::exit(1);
    }

    let from = &args[0];
    let to = &args[1];

    if let Err(e) = std::fs::copy(from, to) {
        println!("{e}");
        std::process::exit(1);
    }
}
