//! meow: Helvilette, a black cat that lives in your terminal.

mod art;
mod cat;
mod state;
mod tui;

const ZSH_PLUGIN: &str = include_str!("../shell/meow.zsh");

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        [] => {
            if let Err(e) = tui::run() {
                eprintln!("meow: {e}");
                std::process::exit(1);
            }
        }
        ["init", "zsh"] => print!("{ZSH_PLUGIN}"),
        _ => {
            eprintln!("usage: meow            open Helvilette's room");
            eprintln!("       meow init zsh   print the zsh plugin (eval \"$(meow init zsh)\")");
            std::process::exit(2);
        }
    }
}
