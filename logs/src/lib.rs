use ansi_term::Color as Color;
use chrono::prelude::Local;
use log::{Record,Level,LevelFilter};
use std::io::Write;
use env_logger::{Builder, Target};


fn strftime() -> String { 
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn init() {
    let mut builder = Builder::from_default_env();

    builder.format(
        |buf, record| { 
        writeln!(buf,"{} [{}] {}",Color::Fixed(8).bold().paint(strftime()),record.level(),record.args())
        }
    ).filter(None,LevelFilter::Info);
    builder.target(Target::Stdout);
    builder.init();
}


#[cfg(test)]
mod tests {
    use super::*;    
    #[test]
    fn it_works() {
        init();
        log::info!("good");
        log::debug!("debug");
   }


}
