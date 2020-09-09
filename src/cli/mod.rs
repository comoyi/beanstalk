pub struct Cli {}

impl Cli {
    pub fn new() -> Self {
        Cli {}
    }

    pub fn run(&self) {
        let subcommand_stats = clap::App::new("stats")
            .about("Print info about tubes")
            .arg(
                clap::Arg::with_name("host")
                    .long("host")
                    .default_value("127.0.0.1")
                    .takes_value(true),
            )
            .arg(
                clap::Arg::with_name("port")
                    .long("port")
                    .default_value("11300")
                    .takes_value(true),
            );

        let subcommand_put = clap::App::new("put")
            .about("Put data to a tube")
            .arg(
                clap::Arg::with_name("host")
                    .long("host")
                    .default_value("127.0.0.1")
                    .takes_value(true),
            )
            .arg(
                clap::Arg::with_name("port")
                    .long("port")
                    .default_value("11300")
                    .takes_value(true),
            )
            .arg(
                clap::Arg::with_name("tube")
                    .short('t')
                    .long("tube")
                    .takes_value(true)
                    .about("Specify a tube"),
            )
            .arg(
                clap::Arg::with_name("delay")
                    .long("delay")
                    .takes_value(true)
                    .default_value("0")
                    .about("Delay"),
            )
            .arg(
                clap::Arg::with_name("pri")
                    .long("pri")
                    .takes_value(true)
                    .default_value("1024")
                    .about(
                        "<pri> is an integer < 2^32. Jobs with smaller priority values will be
   scheduled before jobs with larger priorities. The most urgent priority is 0;
   the least urgent priority is 4,294,967,295.",
                    ),
            )
            .arg(
                clap::Arg::with_name("ttr")
                    .long("ttr")
                    .takes_value(true)
                    .default_value("3600")
                    .about(
                        "-- time to run -- is an integer number of seconds to allow a worker
   to run this job. This time is counted from the moment a worker reserves
   this job. If the worker does not delete, release, or bury the job within
   <ttr> seconds, the job will time out and the server will release the job.
   The minimum ttr is 1. If the client sends 0, the server will silently
   increase the ttr to 1. Maximum ttr is 2^32-1.",
                    ),
            )
            .arg(
                clap::Arg::with_name("data")
                    .long("data")
                    .takes_value(true)
                    .about("Data"),
            );

        let matches = clap::App::new("beanstalk")
            .about("A beanstalkd CLI")
            .version("0.1.0")
            .author("Michael")
            .subcommand(subcommand_stats)
            .subcommand(subcommand_put)
            .get_matches();

        match matches.subcommand() {
            (super::command::COMMAND_STATS, Some(matches)) => {
                println!("Match stat command");
                let command =
                    super::command::stats_command_builder::StatsCommandBuilder::new().build();
                println!("Command to be executed:\n{}", command.to_request());
                super::client::Client::new(
                    matches.value_of_t_or_exit("host"),
                    matches.value_of_t_or_exit("port"),
                )
                .execute(command.to_request());
            }
            (super::command::COMMAND_PUT, Some(matches)) => {
                println!("Match put command");
                let command = super::command::put_command_builder::PutCommandBuilder::new()
                    .delay(matches.value_of_t_or_exit("delay"))
                    .pri(matches.value_of_t_or_exit("pri"))
                    .ttr(matches.value_of_t_or_exit("ttr"))
                    .data(matches.value_of_t_or_exit("data"))
                    .build();
                println!("Command to be executed:\n{}", command.to_request());
                super::client::Client::new(
                    matches.value_of_t_or_exit("host"),
                    matches.value_of_t_or_exit("port"),
                )
                .execute(command.to_request());
            }
            ("", None) => println!("No subcommand was used"),
            _ => {}
        }
    }
}
