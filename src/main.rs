//! SigmaOS-emacs-window-theme-directory — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_emacs_window_theme_directory_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-emacs-window-theme-directory: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-emacs-window-theme-directory: ok");
}
