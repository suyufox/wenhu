use clap::{ Parser, Subcommand };

// mod env;
mod dev;

#[derive(Parser)]
#[command(
    name = "wenhu-cli",
    version = "0.1.0",
    author = "suyufox",
    about = "文化项目CLI工具",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 环境管理命令
    // Env(env::EnvOptions),
    Dev(dev::DevOpions),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        None => {
            // println!("欢迎使用文化项目CLI工具");
            // println!("默认功能待实现，可以使用子命令：");
            // println!("  xxxx - 环境管理功能");
            println!("xxxxxxxxxxxx")
        }
        Some(cmd) =>
            match cmd {
                // Commands::Env(options) => env::command(options),
                Commands::Dev(options) => dev::command(options),
            }
    }
}
