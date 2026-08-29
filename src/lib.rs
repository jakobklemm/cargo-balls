use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

pub fn measure(f: impl FnOnce()) -> Duration {
    let start = Instant::now();
    f();
    Instant::now() - start
}

pub fn path() -> PathBuf {
    let mut target = target_path();
    target.push("balls/");
    println!("{:?}", target);

    std::fs::create_dir_all(&target).unwrap();
    target
}

fn target_path() -> PathBuf {
    if let Ok(dir) = std::env::var("CARGO_TARGET_DIR") {
        println!("fast return: {:?}", dir);
        return PathBuf::from(dir);
    }

    let exe = std::env::current_exe().unwrap();
    let mut dir = exe.as_path();
    while let Some(parent) = dir.parent() {
        if dir.file_name().map(|n| n == "target").unwrap_or(false) {
            println!("dir: {:?}", dir);
            return dir.to_path_buf();
        }
        dir = parent;
    }

    panic!("invalid configuration, no target dir found");
}
