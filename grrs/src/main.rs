use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();

    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => { content }
        Err(error) => { panic!("oh noes: {}", error); }
    };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
