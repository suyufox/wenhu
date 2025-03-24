fn main() {
    // 注入构建时间
    println!(
        "cargo:rustc-env=CARGO_BUILD_DATE={}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    );

    // 注入Git提交哈希（短格式）
    // let git_hash = std::process::Command
    //     ::new("git")
    //     .args(["rev-parse", "--short", "HEAD"])
    //     .output()
    //     .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    //     .unwrap_or_else(|_| "unknown".into());
    // println!("cargo:rustc-env=GIT_COMMIT_HASH={}", git_hash);

    // 当项目文件变更时重新构建
    println!("cargo:rerun-if-changed=build.rs");
}
