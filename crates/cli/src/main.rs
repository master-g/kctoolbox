use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
  #[command(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
enum Commands {
  /// Unpack sprite sheet
  UnpackSprite {
    /// Path to sprite sheet json file
    #[arg(short, long)]
    src: String,
    /// Path to output directory
    #[arg(short, long)]
    dst: String,
  },
}

fn main() {
  let cli = Cli::parse();

  // You can check for the existence of subcommands, and if found use their
  // matches just as you would the top level cmd
  match &cli.command {
    Commands::UnpackSprite { src, dst } => {
      println!("unpacking: {:?} to {:?}", src, dst);
      sprite_unpacker::execute(&src, &dst).unwrap();
    }
  }
}
