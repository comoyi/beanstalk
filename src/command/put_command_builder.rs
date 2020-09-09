use crate::command::put_command::PutCommand;

pub struct PutCommandBuilder {
    pri: u32,
    delay: u32,
    ttr: u32,
    data: String,
}

impl PutCommandBuilder {
    pub fn new() -> Self {
        return PutCommandBuilder {
            pri: 0,
            delay: 0,
            ttr: 0,
            data: String::new(),
        };
    }

    #[allow(dead_code)]
    pub fn pri(&mut self, pri: u32) -> &mut Self {
        self.pri = pri;
        return self;
    }

    #[allow(dead_code)]
    pub fn delay(&mut self, delay: u32) -> &mut Self {
        self.delay = delay;
        return self;
    }

    #[allow(dead_code)]
    pub fn ttr(&mut self, ttr: u32) -> &mut Self {
        self.ttr = ttr;
        return self;
    }

    #[allow(dead_code)]
    pub fn data(&mut self, data: String) -> &mut Self {
        self.data = data;
        return self;
    }

    pub fn build(&self) -> PutCommand {
        let mut command = PutCommand::new();
        command
            .set_pri(self.pri)
            .set_delay(self.delay)
            .set_ttr(self.ttr)
            .set_data(self.data.to_string());
        return command;
    }
}
