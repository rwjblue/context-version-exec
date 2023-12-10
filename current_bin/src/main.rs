use clap::Parser;

#[derive(Parser)]
struct Args {
    /// Specify the manifest version -- 1 uses `prior_bin` and 2+ will use `current_bin`
    #[arg(long)]
    manifest_version: u8,
}

fn main() {
    let args = Args::parse();

    if args.manifest_version == 1 {
        println!("prior_bin");
    } else {
        println!("current_bin");
    }
}
