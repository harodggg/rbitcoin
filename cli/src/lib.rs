mod contants;

use contants::*;
use std::io::Error as error;
use structopt::StructOpt;
use log::*;
use db;
use rpc;
use p2p;


#[derive(StructOpt,Debug)]
#[structopt(about = "Select a subcommand")]
enum SubComand{
    #[structopt(name = "testnet")]
    Testnet,

    #[structopt(name = "btc")]
    Btc,

    #[structopt(name = "regtest")]
    Regtest,

    #[structopt(name = "import")]
    Import,

    #[structopt(name = "rollback")]
    Rollback,

}

#[derive(Debug, StructOpt)]
#[structopt(name = "rbitcoin", about = "A rust client for bitcoin")]
struct Opt {
    #[structopt(subcommand)]
    subcommand: SubComand,
}

pub fn run() -> Result<(),error> { 
	let args = Opt::from_args();
    match &args.subcommand {

        SubComand::Testnet => {info!("🟢 Btc测试网启动")},
        SubComand::Btc => {
                    info!("🟢 开始运行rbitcoin主网客户端程序");
                    start_bitcoin_network(SOFTWARE_NAME, AUTHOR, BITCOIN_VERSION);},
        _ => error!("命令错误,请重试")
    }
    
	Ok(())
}


fn show_metadata_message(software_name: &str,author: &str,version: u64) {
    info!("*************************************");
    info!("🔶 <客户端名字>: {}          **",  software_name );
    info!("🔷 <作者>: {}                 **", author);
    info!("💠 <rbitcoin版本>: {}       **", version);
    info!("*************************************");
    warn!("🟡 warning");
    debug!("🔵 debug");
    error!("🔴 error");
    trace!("🟣 trace");
}
// 初始化网络
fn start_bitcoin_network(software_name: &str,author: &str,version: u64) { 
    show_metadata_message(software_name, author, version);
    db::init();
    rpc::init();
    p2p::init();
}