use clap::{Arg, Command};

pub struct CliArgs {
    pub vault_token: String,
    pub guild_id: Option<u64>,
}

impl CliArgs {
    pub fn get_cli_args(cli_name: &'static str) -> Self {
        let matches = Command::new(cli_name)
            .arg(
                Arg::new("VAULT_TOKEN")
                    .required(true)
                    .help("hashicorp vault token"),
            )
            .arg(
                Arg::new("GUILD_ID")
                    .value_parser(clap::value_parser!(u64))
                    .required(false)
                    .help("discord guild id"),
            )
            .get_matches();

        let vault_token = matches
            .get_one::<String>("VAULT_TOKEN")
            .expect("vault token")
            .to_owned();
        let guild_id = matches.get_one::<u64>("GUILD_ID").map(|g| g.to_owned());

        CliArgs {
            vault_token,
            guild_id,
        }
    }
}
