use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "greeter")]
#[command(author = "harsh sharma")]
#[command(version = "1.0")]
#[command(about = "Greets the user", long_about = None)]
struct Cli {
    #[arg(short, long)]
    name: String,

    /// Say it loudly
    #[arg(short, long, default_value_t = false)]
    loud: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Telis {
        #[arg(long, default_value = "telis status")]
        status: String,

        #[arg(long, default_value_t = true)]
        up: bool,
    }
}

fn main() {
    let cli = Cli::parse();
    let greeting = format!("Hello, {}!", cli.name);

    if cli.loud {
        println!("{}", greeting.to_uppercase());
    } else {
        println!("{}", greeting);
    }

    if let Some(Commands::Telis { status, up }) = cli.command {
        let status_msg = format!("{} :- test", status, if up { "up and running" } else { "down" });
        println!("{}", status_msg);
    }
}