use std::process::Output;
use dialoguer::{ theme::ColorfulTheme, Select };
pub fn check() {
    println!("执行检查环境包逻辑... | Executing check environment package logic...");
    let check_action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("请选择要执行的操作 | Select operations:")
        .items(
            &[
                "1. 检查 全部的环境包 | Check all environment packages",
                "2. 检查 Git 应用程序 | Check Git application",
                "3. 检查 Rustup 环境包 | Check Rustup environment packages",
                "4. 检查 Cargo 包管理器 | Check cargo environment packages",
                "5. 检查 Tauri 脚手架 | Check Tauri CLI",
                "6. 检查 Node.js 环境包 | Check Node.js environment packages",
                "7. 检查 npm 包管理器 | Check npm environment packages",
                "8. 检查 yarn 包管理器 | Check yarn environment packages",
                "9. 检查 pnpm 包管理器 | Check pnpm environment packages",
                "10. 检查 deno 包管理器 | Check deno environment packages",
                "11. 检查 Python 环境包 | Check Python environment packages",
                "12. 检查 Java 环境包 | Check Java environment packages",
                "13. 检查 Gradle 环境包 | Check Gradle environment packages",
                "14. 检查 WinGet (仅windows) 包管理器 | Check WinGet (only windows) environment packages",
            ]
        )
        .default(0)
        .interact()
        .unwrap();

    match check_action {
        0 => {
            let (git_installed, git_output) = check_git();
            let (rustup_installed, rustup_output) = check_rustup();
            let (cargo_installed, cargo_output) = check_cargo();
            let (tauri_cli_installed, tauri_cli_output) = check_tauri_cli();
            let (nodejs_installed, nodejs_output) = check_nodejs();
            let (npm_installed, npm_output) = check_npm();
            let (yarn_installed, yarn_output) = check_yarn();
            let (pnpm_installed, pnpm_output) = check_pnpm();
            let (deno_installed, deno_output) = check_deno();
            let (python_installed, python_output) = check_python();
            let (java_installed, java_output) = check_java();
            let (gradle_installed, gradle_output) = check_gradle();
            println!("\nGit 应用程序检查结果 | Git application check results:");
            check_optput(git_installed, git_output);
            println!("\nRustup 环境包检查结果 | Rustup environment package check results:");
            check_optput(rustup_installed, rustup_output);
            println!("\ncargo 环境包检查结果 | cargo environment package check results:");
            check_optput(cargo_installed, cargo_output);
            println!("\ntauri 脚手架检查结果 | tauri environment package check results:");
            check_optput(tauri_cli_installed, tauri_cli_output);
            println!("\nNode.js 环境包检查结果 | Node.js environment package check results:");
            check_optput(nodejs_installed, nodejs_output);
            println!("\nnpm 环境包检查结果 | npm environment package check results:");
            check_optput(npm_installed, npm_output);
            println!("\nyarn 环境包检查结果 | yarn environment package check results:");
            check_optput(yarn_installed, yarn_output);
            println!("\npnpm 环境包检查结果 | pnpm environment package check results:");
            check_optput(pnpm_installed, pnpm_output);
            println!("\ndeno 环境包检查结果 | deno environment package check results:");
            check_optput(deno_installed, deno_output);
            println!("\nPython 环境包检查结果 | Python environment package check results:");
            check_optput(python_installed, python_output);
            println!("\nJava 环境包检查结果 | Java environment package check results:");
            check_optput(java_installed, java_output);
            println!("\nGradle 环境包检查结果 | Gradle environment package check results:");
            check_optput(gradle_installed, gradle_output);
        }

        1 => {
            let (installed, output) = check_git();
            println!("Git 应用程序检查结果 | Git application check results:");
            check_optput(installed, output);
        }

        2 => {
            let (installed, output) = check_rustup();
            println!("Rustup 环境包检查结果 | Rustup environment package check results:");
            check_optput(installed, output);
        }

        3 => {
            let (installed, output) = check_cargo();
            println!("cargo 环境包检查结果 | cargo environment package check results:");
            check_optput(installed, output);
        }

        4 => {
            let (installed, output) = check_tauri_cli();
            println!("tauri 脚手架检查结果 | tauri environment package check results:");
            check_optput(installed, output);
        }

        5 => {
            let (installed, output) = check_nodejs();
            println!("Node.js 环境包检查结果 | Node.js environment package check results:");
            check_optput(installed, output);
        }

        6 => {
            let (installed, output) = check_npm();
            println!("npm 环境包检查结果 | npm environment package check results:");
            check_optput(installed, output);
        }

        7 => {
            let (installed, output) = check_yarn();
            println!("yarn 环境包检查结果 | yarn environment package check results:");
            check_optput(installed, output);
        }

        8 => {
            let (installed, output) = check_pnpm();
            println!("pnpm 环境包检查结果 | pnpm environment package check results:");
            check_optput(installed, output);
        }

        9 => {
            let (installed, output) = check_deno();
            println!("deno 环境包检查结果 | deno environment package check results:");
            check_optput(installed, output);
        }

        10 => {
            let (installed, output) = check_python();
            println!("Python 环境包检查结果 | Python environment package check results:");
            check_optput(installed, output);
        }

        11 => {
            let (installed, output) = check_java();
            println!("Java 环境包检查结果 | Java environment package check results:");
            check_optput(installed, output);
        }

        12 => {
            let (installed, output) = check_gradle();
            println!("Gradle 环境包检查结果 | Gradle environment package check results:");
            check_optput(installed, output);
        }

        _ => unreachable!(),
    }
}
fn check_optput(installed: bool, raw_output: String) {
    if installed {
        println!("已安装 | Installed");
        println!("完整输出 | Full output：\n{}", raw_output);
    } else {
        println!("未安装 | Not installed");
    }
}
fn check_optput_action(output: Option<Output>) -> (bool, String) {
    if let Some(output) = output {
        if output.status.success() {
            let full_output = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return (true, full_output);
        }
    }

    (false, "Command execution failed".into())
}
pub fn check_git() -> (bool, String) {
    let output = std::process::Command::new("git").arg("--version").output().ok();
    check_optput_action(output)
}
pub fn check_rustup() -> (bool, String) {
    let output = std::process::Command::new("rustup").arg("--version").output().ok(); // 改为使用ok()避免panic

    check_optput_action(output)
}
pub fn check_cargo() -> (bool, String) {
    let output = std::process::Command::new("cargo").arg("--version").output().ok();

    check_optput_action(output)
}
pub fn check_nodejs() -> (bool, String) {
    let output = std::process::Command::new("node").arg("--version").output().ok();

    check_optput_action(output)
}
pub fn check_pnpm() -> (bool, String) {
    #[cfg(target_os = "windows")]
    let output = std::process::Command::new("cmd").args(["/C", "pnpm --version"]).output().ok();
    #[cfg(not(target_os = "windows"))]
    let output = std::process::Command::new("sh").arg("-c").arg("pnpm --version").output().ok();
    check_optput_action(output)
}
pub fn check_npm() -> (bool, String) {
    #[cfg(target_os = "windows")]
    let output = std::process::Command::new("cmd").args(["/C", "npm --version"]).output().ok();
    #[cfg(not(target_os = "windows"))]
    let output = std::process::Command::new("sh").arg("-c").arg("npm --version").output().ok();
    check_optput_action(output)
}
pub fn check_yarn() -> (bool, String) {
    #[cfg(target_os = "windows")]
    let output = std::process::Command::new("cmd").args(["/C", "yarn --version"]).output().ok();
    #[cfg(not(target_os = "windows"))]
    let output = std::process::Command::new("sh").arg("-c").arg("yarn --version").output().ok();
    check_optput_action(output)
}
pub fn check_python() -> (bool, String) {
    let output = std::process::Command::new("python").arg("--version").output().ok();
    check_optput_action(output)
}
pub fn check_java() -> (bool, String) {
    let output = std::process::Command::new("java").arg("--version").output().ok();
    check_optput_action(output)
}
pub fn check_deno() -> (bool, String) {
    let output = std::process::Command::new("deno").arg("--version").output().ok();
    check_optput_action(output)
}
pub fn check_tauri_cli() -> (bool, String) {
    let output = std::process::Command
        ::new("cargo")
        .args(["tauri", "--version"]) // 正确写法
        .output()
        .ok();
    check_optput_action(output)
}
pub fn check_gradle() -> (bool, String) {
    let output = std::process::Command::new("gradle").arg("--version").output().ok();
    check_optput_action(output)
}
