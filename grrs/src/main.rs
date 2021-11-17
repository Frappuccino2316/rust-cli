use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
}

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
