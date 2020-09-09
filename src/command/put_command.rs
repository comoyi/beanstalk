use crate::command::COMMAND_PUT;

pub struct PutCommand {
    command: String,
    pri: u32,
    delay: u32,
    ttr: u32,
    data: String,
}

impl PutCommand {
    pub fn new() -> Self {
        PutCommand {
            command: COMMAND_PUT.to_string(),
            pri: 0,
            delay: 0,
            ttr: 0,
            data: "".to_string(),
        }
    }

    pub fn to_request(&self) -> String {
        let bytes = self.data.len();
        let request = format!(
            "{} {} {} {} {}\r\n{}\r\n",
            self.command, self.pri, self.delay, self.ttr, bytes, self.data
        );
        return request;
    }

    pub fn set_pri(&mut self, pri: u32) -> &mut Self {
        self.pri = pri;
        return self;
    }

    pub fn set_delay(&mut self, delay: u32) -> &mut Self {
        self.delay = delay;
        return self;
    }

    pub fn set_ttr(&mut self, ttr: u32) -> &mut Self {
        self.ttr = ttr;
        return self;
    }

    pub fn set_data(&mut self, data: String) -> &mut Self {
        self.data = data;
        return self;
    }
}
