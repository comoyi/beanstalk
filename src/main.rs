mod cli;
mod client;
mod command;
mod lib;
mod handler;

fn main() {
    lib::App::new().run();
}
