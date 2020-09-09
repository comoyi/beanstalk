use crate::command::stats_command::StatsCommand;

pub struct StatsCommandBuilder {}

impl StatsCommandBuilder {
    pub fn new() -> Self {
        return StatsCommandBuilder {};
    }

    pub fn build(&self) -> StatsCommand {
        let command = StatsCommand::new();
        return command;
    }
}
