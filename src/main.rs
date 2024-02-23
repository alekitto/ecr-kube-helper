mod commands;

use crate::commands::{get_credentials_command, GetCredentialsArgs};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "ecr-kube-helper")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Gets ECR credentials and save them into a kube secret.
    #[command(arg_required_else_help = true)]
    GetCredentials(GetCredentialsArgs),
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let args = Cli::parse();

    match args.command {
        Commands::GetCredentials(args) => get_credentials_command(args).await,
    }
}
