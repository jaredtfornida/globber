use clap::Parser;

#[derive(Parser)]
#[command(name = "MyApp")]
#[command(author = "Ja Red")]
#[command(version = "1.1")]
#[command(about = "Does awesome stuff" , long_about = None)]
struct Cli {
    #[arg(long)]
    two: String
}

fn main() {
    let cli = Cli::parse();

    println!("LETS GOO: {:?}", cli.two);
}
