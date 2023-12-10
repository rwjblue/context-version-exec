use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Specify the manifest version -- 1 uses `prior_bin` and 2+ will use `current_bin`
    #[arg(long)]
    manifest_version: u8,
}

fn path_to_prior_bin() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.set_file_name("prior_bin");
    path.into_os_string().into_string().unwrap()
}

fn main() {
    let args = Args::parse();

    if args.manifest_version == 1 {
        println!("prior_bin");
        let args: Vec<String> = std::env::args().collect();

        let err = exec::Command::new(path_to_prior_bin())
            .args(&args[2..])
            .exec();
        println!("Error: {}", err);
    } else {
        println!("current_bin");
    }
}
