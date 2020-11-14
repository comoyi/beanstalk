use clap::ArgMatches;

pub struct PutHandler {

}

impl PutHandler {
    pub fn new() ->Self {
        PutHandler {

        }
    }

    pub fn handle(&self, matches: &ArgMatches) {
        println!("Match put command");
        let command = super::super::command::put_command_builder::PutCommandBuilder::new()
            .delay(matches.value_of_t_or_exit("delay"))
            .pri(matches.value_of_t_or_exit("pri"))
            .ttr(matches.value_of_t_or_exit("ttr"))
            .data(matches.value_of_t_or_exit("data"))
            .build();
        println!("Command to be executed:\n{}", command.to_request());
        super::super::client::Client::new(
            matches.value_of_t_or_exit("host"),
            matches.value_of_t_or_exit("port"),
        )
            .execute(command.to_request());
    }
}