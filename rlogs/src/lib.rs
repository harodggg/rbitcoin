// Copyright © 2021 Harodggg.All Rights Reserved.

// This file is part of R-bitcoin.

// R-bitcoin is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.

// R-bitcoin is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Rbitcoin.  If not, see <http://www.gnu.org/licenses/>.

use ansi_term::Color;
use chrono::prelude::Local;
use env_logger::{Builder, Target};
use log::{LevelFilter, Record};
use std::io::Write;

fn strftime() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

fn level_add_color(record: &Record) -> String {
  let  record_level_and_colors = match record.level() { 
            log::Level::Error =>  Color::Fixed(9).bold().paint(record.level().to_string()),
            log::Level::Debug =>  Color::Fixed(14).paint(record.level().to_string()),
            log::Level::Info  =>  Color::Fixed(10).paint(record.level().to_string()),
            log::Level::Trace =>  Color::Fixed(12).paint(record.level().to_string()),
            log::Level::Warn  =>  Color::Fixed(11).bold().paint(record.level().to_string()),
    };
        
    format!(
                "{} [{}] {}",
                Color::Fixed(8).bold().paint(strftime()),
                record_level_and_colors,
                record.args() 
    )
}

pub fn init(log_level: LevelFilter) {
    let mut builder = Builder::from_default_env();
    builder
        .format(|buf, record| {
            writeln!(
                buf,
                "{}",
                level_add_color(record)
            )
        })
        .filter(None, log_level);
    builder.target(Target::Stdout);
    builder.init();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        init(LevelFilter::Trace);
        log::info!("good");
        log::debug!("debug");
    }
}
