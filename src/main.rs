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

use color_eyre::eyre;
use logs;
#[macro_use]
extern crate log;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    info!("开始客户端");
    logs::init();
    rbitcoin_cli::run()?;
    Ok(())
}
