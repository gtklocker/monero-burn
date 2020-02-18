use monero_burn::gen_burn_addr;

use structopt::StructOpt;

extern crate hex;

#[derive(Debug, StructOpt)]
struct Cli {
    tag: String,
}

fn main() {
    let args = Cli::from_args();
    let tag = hex::decode(&args.tag).expect("invalid tag");
    println!("{}", gen_burn_addr(&tag))
}
