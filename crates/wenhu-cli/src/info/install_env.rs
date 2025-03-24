use dialoguer::{ theme::ColorfulTheme, Select };

pub fn install() {
    println!("执行安装环境包逻辑... | Executing environment package installation logic...");

    let install_action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("请选择要执行的操作 | Select operations:")
        .items(
            &[
                "1. 安装 全部的环境包 | Install all environment packages",
                "2. 安装 Git 应用程序 | Install Git application",
                "3. 安装 Rustup 环境包 | Install Rustup environment packages",
                "4. 安装 Cargo 包管理器 | Install cargo environment packages",
                "5. 安装 Tauri 脚手架 | Install Tauri CLI",
                "6. 安装 Node.js 环境包 | Install Node.js environment packages",
                "7. 安装 npm 包管理器 | Install npm environment packages",
                "8. 安装 yarn 包管理器 | Install yarn environment packages",
                "9. 安装 pnpm 包管理器 | Install pnpm environment packages",
                "10. 安装 deno 包管理器 | Install deno environment packages",
                "11. 安装 Python 环境包 | Install Python environment packages",
                "12. 安装 Java 环境包 | Install Java environment packages",
                "13. 安装 Gradle 环境包 | Install Gradle environment packages",
            ]
        )
        .default(0)
        .interact()
        .unwrap();

    match install_action {
        0 => {
            println!("安装全部环境包...");
        }
        1 => {
            println!("安装 Git 应用程序...");
        }
        2 => {
            println!("安装 Rustup 环境包...");
        }
        3 => {
            println!("安装 Cargo 包管理器...");
        }
        4 => {
            println!("安装 Tauri 脚手架...");
        }
        5 => {
            println!("安装 Node.js 环境包...");
        }
        6 => {
            println!("安装 npm 包管理器...");
        }
        7 => {
            println!("安装 yarn 包管理器...");
        }
        8 => {
            println!("安装 pnpm 包管理器...");
        }
        9 => {
            println!("安装 deno 包管理器...");
        }
        10 => {
            println!("安装 Python 环境包...");
        }
        11 => {
            println!("安装 Java 环境包...");
        }
        12 => {
            println!("安装 Gradle 环境包...");
        }
        _ => unreachable!(),
    }
}

pub fn install_git() {
    #[cfg(target_os = "windows")]
    {}
}
