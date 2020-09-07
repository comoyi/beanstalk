pub struct Cli {}

impl Cli {
    pub fn new() -> Self {
        Cli {}
    }

    pub fn run(&self) {
        let subcommand_stats = clap::App::new("stat")
            .about("Print info about tubes")
            .arg(
                clap::Arg::with_name("host")
                    .long("host")
                    .default_value("127.0.0.1")
                    .takes_value(true)
            )
            .arg(
                clap::Arg::with_name("port")
                    .long("port")
                    .default_value("11300")
                    .takes_value(true)
            );

        let subcommand_put = clap::App::new("put")
            .about("Put data to a tube")
            .arg(
                clap::Arg::with_name("host")
                    .long("host")
                    .default_value("127.0.0.1")
                    .takes_value(true)
            )
            .arg(
                clap::Arg::with_name("port")
                    .long("port")
                    .default_value("11300")
                    .takes_value(true)
            )
            .arg(
                clap::Arg::with_name("tube")
                    .short('t')
                    .long("tube")
                    .takes_value(true)
                    .about("Specify a tube")
            );

        let matches = clap::App::new("beanstalk")
            .about("A beanstalkd CLI")
            .version("0.1.0")
            .author("Michael")
            .subcommand(subcommand_stats)
            .subcommand(subcommand_put)
            .get_matches();

        match matches.subcommand() {
            ("stat", Some(_stat_matches)) => {
                println!("stat");
            }
            ("put", Some(_put_matches)) => {
                println!("put");
            }
            ("", None) => {
                println!("No subcommand was used")
            }
            _ => {}
        }
    }
}
