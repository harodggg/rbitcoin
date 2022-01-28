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
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use ansi_term::Color;
use chrono::prelude::Local;
use env_logger::{Builder, Target};
use log::LevelFilter;
use std::io::Write;

fn strftime() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn init() {
    let mut builder = Builder::from_default_env();

    builder
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}",
                Color::Fixed(8).bold().paint(strftime()),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info);
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
