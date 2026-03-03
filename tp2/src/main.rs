use tp2::run_game;

fn main() {
    if let Err(message) = run_game("story.yaml") {
        eprintln!("{message}");
        std::process::exit(1);
    }
}
