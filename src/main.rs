#[macro_use] extern crate log;
extern crate fern;

mod cli;
mod serial;
mod player;

fn main() {
    let matches = cli::matches();

    if let Some(matches) = matches.subcommand_matches("logcat") {
        let port_name = matches.value_of("port").unwrap();
        let baud_rate = matches.value_of("baud_rate").unwrap();
        let log_level = matches.value_of("log_level");
        serial::init_log(log_level);
        if let Ok(rate) = baud_rate.parse::<u32>() {
            serial::receive(&port_name, rate.into());
        } else {
            error!("Error: Invalid baud rate '{}' specified", baud_rate);
            ::std::process::exit(1);
        }
    }

    if let Some(matches) = matches.subcommand_matches("player") {
        let dir = matches.value_of("dir").unwrap();
        player::init_log();
        player::singer_player(dir).unwrap();
        ::std::process::exit(1);
    }
}