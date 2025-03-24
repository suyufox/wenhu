use clap::{ Parser, Subcommand };

#[derive(Parser)]
#[command(
    about = "用于管理环境包相关的内容 | Used to manage environment package-related content",
    long_about = None
)]
pub struct DevOptions {
    #[command(subcommand)]
    command: Option<DevCommands>,
}

#[derive(Subcommand)]
enum DevCommands {}

pub fn command(options: EnvOptions) {
    match options.command {
        None =>
            println!(
                "请使用子命令来执行相关操作 | Please use subcommands to perform related operations"
            ),
        Some(cmd) =>
            match cmd {
                _ => (),
            }
    }
}
