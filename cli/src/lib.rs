use std::io::Error as error;
use structopt::StructOpt;
use log::*;

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
    info!("启动客户端");
    match &args.subcommand {

        SubComand::Testnet => {info!("Btc测试网启动")},
        SubComand::Btc => {info!("开始运行rbitcoin主网客户端程序")},
        _ => error!("命令错误,请重试")
    }
    
	Ok(())
}
