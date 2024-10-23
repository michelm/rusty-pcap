
use clap::Parser;

/// Read a pcapng file and prints out its content
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File name including path
    #[arg(short, long, default_value = "test.pcapng")]
    fname: String,
}

fn main() {
    let args = Args::parse();
    let fname = args.fname;
    
    match rusty_pcap::read_pcapng_file(&fname) {
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        },
        Ok(n) => {
            println!("Read {} blocks", n);
        },
    }
}
