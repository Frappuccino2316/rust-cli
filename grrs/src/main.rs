// use structopt::StructOpt;

// unwrapを使うパターン
// fn main() {
//     let args = Cli::from_args();

//     let content = std::fs::read_to_string(&args.path).unwrap();

//     for line in content.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line);
//         }
//     }
// }

// #[derive(StructOpt)]
// struct Cli {
//     pattern: String,
//     #[structopt(parse(from_os_str))]
//     path: std::path::PathBuf,
// }

// panicを使わないパターン
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let result = std::fs::read_to_string("src/test2.txt");
//     let content = match result {
//         Ok(content) => { content },
//         Err(error) => { return Err(error.into()); }
//     };
//     println!("file content: {}", content);
//     Ok(())
// }

// ?を使うパターン
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("src/test.txt")?;
    println!("file content: {}", content);
    Ok(())
}