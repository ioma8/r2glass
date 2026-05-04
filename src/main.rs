use std::path::PathBuf;

use r2glass::app::R2GlassApp;

fn main() -> eframe::Result {
    let args: Vec<String> = std::env::args().collect();
    for arg in &args {
        match arg.as_str() {
            "--help" | "-h" => {
                println!("r2glass {} — radare2 GUI frontend", env!("CARGO_PKG_VERSION"));
                println!();
                println!("Usage: r2glass [TARGET]");
                println!("  TARGET  Path to a binary to analyze (opens in GUI)");
                println!();
                println!("Options:");
                println!("  -h, --help     Print this help message");
                println!("  -V, --version  Print version information");
                return Ok(());
            }
            "--version" | "-V" => {
                println!("r2glass {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            _ => {}
        }
    }
    let target = args.get(1).map(PathBuf::from);
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "r2glass",
        options,
        Box::new(|_cc| Ok(Box::new(R2GlassApp::new(target)))),
    )
}
