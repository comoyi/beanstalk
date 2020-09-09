mod cli;
mod client;
mod command;
mod lib;

fn main() {
    lib::App::new().run();
}
