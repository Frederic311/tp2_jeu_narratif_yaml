use tp2::run_game;

fn main() {
    if let Err(err) = run_game("story.yaml") {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
