use std::{fs, io};
use std::io::BufReader;
use fern::colors::{Color, ColoredLevelConfig};

pub fn init_log() {
    let colors = ColoredLevelConfig::new()
        .debug(Color::Magenta).info(Color::BrightGreen);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),                
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply().unwrap();
}

pub fn singer_player(dir: &str) -> io::Result<()> {
    let mut entries = fs::read_dir(dir)?
        .map(|res| res.map(|e| e.path()))
        .filter(|res|
            match res {
                Ok(path) => {
                    path.display().to_string().ends_with("wav")
                },
                Err(_) => false
            }
        )
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    for entry in entries {
        let file = std::fs::File::open(&entry).unwrap();
        info!("{}", &entry.display());
        sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
        sink.sleep_until_end();
    }

    Ok(())
}