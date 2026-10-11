use std::io::{BufRead, Write};

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() == 0 {
        let mut stdin = std::io::stdin();
        let mut stdout = std::io::stdout();

        std::io::copy(&mut stdin, &mut stdout)?;
    } else {
        let stdin = std::io::stdin();
        let mut reader = std::io::BufReader::new(stdin);
        loop {
            let mut buf = String::new();

            let n = reader.read_line(&mut buf)?;
            if n == 0 {
                break;
            }

            print!("{buf}");

            for file_path in &args {
                let mut file = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&file_path)?;
                file.write(buf.as_bytes())?;
            }
        }
    }

    Ok(())
}
