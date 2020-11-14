use clap::ArgMatches;

pub struct StatsHandler {

}

impl StatsHandler {
    pub fn new() ->Self {
        StatsHandler {

        }
    }

    pub fn handle(&self, matches: &ArgMatches) {
        println!("Match stat command");
        let command =
            super::super::command::stats_command_builder::StatsCommandBuilder::new().build();
        println!("Command to be executed:\n{}", command.to_request());
        super::super::client::Client::new(
            matches.value_of_t_or_exit("host"),
            matches.value_of_t_or_exit("port"),
        )
            .execute(command.to_request());
    }
}