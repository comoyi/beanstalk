mod cli;
mod command;
mod lib;

fn main() {
    lib::App::new().run();
}
