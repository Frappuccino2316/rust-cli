use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    // let content = std::fs::read_to_string(&args.path).expect("could not read file");
    let result = std::fs::read_to_string(&args.path);
    match result {
        Ok(content) => {
            println!("File content: {}", content);

            for line in content.lines() {
                if line.contains(&args.pattern) {
                    println!("{}", line);
                }
            }
        }
        Err(error) => { println!("oh noes: {}", error); }
    }
}

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
