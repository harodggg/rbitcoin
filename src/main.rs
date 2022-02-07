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

use color_eyre::eyre;
use log::LevelFilter;
use log::*;
use rlogs;

const LOG_LEVEL: LevelFilter = LevelFilter::Trace;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    rlogs::init(LOG_LEVEL);
    info!("🚀 初始化日志系统");
    rbitcoin_cli::run()?;
    warn!("❗ 客户端结束结束运行，成功退出");
    Ok(())
}
