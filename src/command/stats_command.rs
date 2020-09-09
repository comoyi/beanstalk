use crate::command::COMMAND_STATS;

pub struct StatsCommand {
    command: String,
}

impl StatsCommand {
    pub fn new() -> Self {
        StatsCommand {
            command: COMMAND_STATS.to_string(),
        }
    }

    pub fn to_request(&self) -> String {
        let request = format!("{}\r\n", self.command);
        return request;
    }
}
