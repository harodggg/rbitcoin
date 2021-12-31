use std::io::Error as error;
use structopt::StructOpt;
use log::info;

#[derive(StructOpt,Debug)]
#[structopt(about = "Select a subcommand")]
enum SubComand{
    #[structopt(name = "testnet")]
    Testnet { 
    },

    #[structopt(name = "btc")]
    Btc {
        ip: String,
    },

    #[structopt(name = "regtest")]
    Regtest { 
    },

    #[structopt(name = "import")]
    Import {
        import: String,
    },

    #[structopt(name = "rollback")]
    Rollback { 
        rollback: String,
    }

}

#[derive(Debug, StructOpt)]
#[structopt(name = "rbitcoin", about = "A rust client for bitcoin")]
struct Opt {
    #[structopt(subcommand)]
    subcommand: SubComand,
}

pub fn run() -> Result<(),error> { 
	let args = Opt::from_args();

    println!("{:?}", args);
    info!("启动客户端");
	Ok(())
}

