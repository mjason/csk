// use std::io::BufReader;

// fn main() {
//     let path = "./dataset/你好小奥50.wav";
//     let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
//     let sink = rodio::Sink::try_new(&handle).unwrap();

//     let file = std::fs::File::open(path).unwrap();
//     sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

//     sink.sleep_until_end();
//     println!("{}", "done")
// }

use std::io::{self, Write};
use std::time::Duration;
use serialport::prelude::*;


fn main() {
    println!("{}", "done");
    
    let baud_rate = 115200;
    let port_name = "/dev/tty.wchusbserial14210";

    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(10);
    settings.baud_rate = baud_rate;
    

    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 1000];
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
    
}