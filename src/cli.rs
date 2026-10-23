use std::process::Command;

pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd")
            .args(["/C", "cls"])
            .status();
    } else {
        let _ = Command::new("clear")
            .status();
    }
}

pub fn runJs(pathOfArchive: &str) {
    clear_screen();
    let status = Command::new("node")
        .arg(pathOfArchive)
        .status()
        .expect("error: You have npm installed?");

    if status.success() {
        // Mantido vazio conforme o original
    } else {
        print!("err ocorred");
    }
}
