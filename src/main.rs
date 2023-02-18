use ghmsg::cli::Cli;

fn main() {
    let mut c = Cli::new();
    c.init();
    c.run();
}
