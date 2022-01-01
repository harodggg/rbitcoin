use std::io::Error as error;
use structopt::StructOpt;
use log::info;

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

        SubComand::Testnet => {println!("Btc测试网启动")},
        SubComand::Btc => {println!("BTC主网启动")},
        _ => println!("kdfjdk"),
    }
    
	Ok(())
}

