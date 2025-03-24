use clap::{ Parser, Subcommand };
use dialoguer::{ Select, theme::ColorfulTheme };

mod list;
mod install;

pub enum EnvPackages {
    Git,
    Rustup,
    Cargo,
    Tauri,
    NodeJs,
    Npm,
    Yarn,
    Pnpm,
    Deno,
    Python,
    Java,
    Gradle,
    #[cfg(target_os = "windows")]
    WinGet,
}

impl EnvPackages {
    pub fn all() -> &'static [Self] {
        &[
            Self::Git,
            Self::Rustup,
            Self::Cargo,
            Self::Tauri,
            Self::NodeJs,
            Self::Npm,
            Self::Yarn,
            Self::Pnpm,
            Self::Deno,
            Self::Python,
            Self::Java,
            Self::Gradle,
            #[cfg(target_os = "windows")] Self::WinGet,
        ]
    }

    pub fn bin_name(&self) -> &'static str {
        match self {
            Self::Git => "git",
            Self::Rustup => "rustup",
            Self::Cargo => "cargo",
            Self::Tauri => "cargo-tauri",
            Self::NodeJs => "node",
            Self::Npm => "npm",
            Self::Yarn => "yarn",
            Self::Pnpm => "pnpm",
            Self::Deno => "deno",
            Self::Python => "python",
            Self::Java => "java",
            Self::Gradle => "gradle",
            #[cfg(target_os = "windows")]
            Self::WinGet => "winget",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Git =>
                "Git是一个开源的分布式版本控制系统，可以有效、高速地处理从很小到非常大的项目版本管理。\n Git is an open source distributed version control system that can efficiently and quickly handle versioning from very small to very large projects",
            Self::Rustup =>
                "Rustup是Rust官方提供的工具，用于管理Rust的版本和工具链。\n Rustup is an official Rust tool for managing Rust versions and toolchains.",
            Self::Cargo =>
                "Cargo是Rust官方提供的包管理工具，用于构建、测试和发布Rust项目。\n Cargo is Rust's official package management tool for building, testing, and publishing Rust projects.",
            Self::Tauri =>
                "Tauri是一个用于构建跨平台桌面应用程序的工具，使用Rust和Web技术。 \n Tauri is a tool for building cross-platform desktop applications, using Rust and Web technologies.",
            Self::NodeJs =>
                "Node.js是一个基于Chrome V8引擎的JavaScript运行时环境，用于构建服务器端应用程序。\n Node.js is a JavaScript runtime environment based on the Chrome V8 engine for building server-side applications.",
            Self::Npm =>
                "npm是Node.js的包管理工具，用于安装、更新和管理Node.js项目的依赖项。\n npm is a package management tool for Node.js that is used to install, update, and manage the dependencies of Node.js projects.",
            Self::Yarn =>
                "Yarn是Facebook开发的替代npm的包管理工具，用于快速、可靠地安装和管理Node.js项目的依赖项。\nYarn is a package management tool developed by Facebook to replace npm to quickly and reliably install and manage Node.js project dependencies.",
            Self::Pnpm =>
                "pnpm是一个快速、可靠的替代npm的包管理工具，用于安装和管理Node.js项目的依赖项。\npnpm is a fast and reliable alternative to npm package management tool for installing and managing dependencies of Node.js projects.",
            Self::Deno =>
                "Deno是一个安全的JavaScript运行时环境，用于构建服务器端应用程序和Web应用程序。\nDeno is a secure JavaScript runtime environment for building server-side applications and Web applications.",
            Self::Python =>
                "Python是一种通用的、解释性的编程语言，用于开发各种类型的应用程序。\nPython is a general-purpose, interpretive programming language used to develop various types of applications.",
            Self::Java =>
                "Java是一种面向对象的编程语言，用于开发各种类型的应用程序。\nJava is an object-oriented programming language used to develop various types of applications.",
            Self::Gradle =>
                "Gradle是一个用于构建和管理Java项目的工具，用于自动化构建过程。\nGradle is a tool for building and managing Java projects to automate the build process.",
            #[cfg(target_os = "windows")]
            Self::WinGet =>
                "WinGet是一个用于安装和管理Windows应用程序的工具，类似于macOS的Homebrew。\nWinGet is a tool for installing and managing Windows applications, similar to Homebrew for macOS.",
        }
    }

    pub fn website(&self) -> &'static str {
        match self {
            Self::Git => "https://git-scm.com/book/zh/v2/%E8%B5%B7%E6%AD%A5-%E5%AE%89%E8%A3%85-Git",
            Self::Rustup => "https://www.rust-lang.org/zh-CN/",
            Self::Cargo => "https://www.rust-lang.org/zh-CN/",
            Self::Tauri => "https://tauri.app/",
            Self::NodeJs => "https://nodejs.org/zh-cn",
            Self::Npm => "https://nodejs.org/zh-cn",
            Self::Yarn => "https://yarnpkg.com/",
            Self::Pnpm => "https://pnpm.io/",
            Self::Deno => "https://deno.com/",
            Self::Python => "https://www.python.org/",
            Self::Java => "https://www.java.com/zh-CN/  |  https://www.oracle.com/cn/java/",
            Self::Gradle => "https://gradle.org/",
            #[cfg(target_os = "windows")]
            Self::WinGet => "https://github.com/microsoft/winget-cli",
        }
    }
}

#[derive(Parser)]
#[command(
    about = "用于管理环境包相关的内容 | Used to manage environment package-related content",
    long_about = None
)]
pub struct EnvOptions {
    #[command(subcommand)]
    command: Option<EnvCommands>,
}

#[derive(Subcommand)]
enum EnvCommands {
    List(list::EnvListOptions),
    Install(install::EnvInstallOptions),
    Uninstall(list::EnvListOptions),
}

pub fn command(options: EnvOptions) {
    match options.command {
        None => env_main(),
        Some(cmd) =>
            match cmd {
                EnvCommands::List(options) => list::command(options),
                EnvCommands::Install(options) => install::command(options),
                EnvCommands::Uninstall(options) => list::command(options),
            }
    }
}

fn env_main() {
    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("请选择要执行的操作 | Select operations:")
        .items(
            &[
                "查看 包 | View packages",
                "安装 包 | Install package",
                "卸载或修改包 | Uninstall or modify package",
            ]
        )
        .default(0)
        .interact()
        .unwrap();

    match action {
        0 => list::command(list::EnvListOptions { all: false, locale: false, optional: false }),
        1 =>
            install::command(install::EnvInstallOptions {
                package: String::from(""),
                method: None,
                arch: None,
                version: None,
                mirror: None,
            }),
        2 => {
            println!("卸载或修改包...");
        }
        _ => {
            println!("无效的选择 | Invalid selection");
        }
    }
}
