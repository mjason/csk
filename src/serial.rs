use std::time::Duration;
use serialport::prelude::*;
use serial_frame::{create_line_sender};
use fern::colors::{Color, ColoredLevelConfig};

fn logger(mark: &str, message: &str) {
    if let Some(message) = message.get(3..) {
        match mark {
            "[D]" => debug!("{}", message),
            "[I]" => info!("{}", message),
            _ => info!("{}", message)
        }
    }
}

fn logger_route(message: String) {
    if let Some(message) = message.strip_suffix("\n") {
        match message.get(..3) {
            None => info!("{}", message),
            Some(mark) => logger(mark, &message)
        }
    }
}

pub fn receive(port_name: &str, baud_rate: u32) {    
    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(10);
    settings.baud_rate = baud_rate;

    let serialport = match serialport::open_with_settings(port_name, &settings) {
        Ok(port) => port,
        Err(e) => {
            error!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    };

    let (rx, linestop) = create_line_sender(serialport).unwrap();
    while let Ok(line) = rx.recv_timeout(Duration::from_secs(50)) {
        let line = String::from(line);
        logger_route(line);
    }
    let e = linestop.stop();
    info!("Stop: {:?}", e);
}

pub fn init_log(log_level: Option<&str>) {

    let level = match log_level {
        Some(leve) => {
            match leve {
                "info" => log::LevelFilter::Info,
                "trace" => log::LevelFilter::Trace,
                "error" => log::LevelFilter::Error,
                "warn" => log::LevelFilter::Warn,
                _ => log::LevelFilter::Debug
            }
        },
        None => log::LevelFilter::Debug,
    };

    let colors = ColoredLevelConfig::new()
        .debug(Color::Magenta).info(Color::BrightGreen);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}]{}",
                colors.color(record.level()),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply().unwrap();
}