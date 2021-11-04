use std::io::Error as error;
use structopt::StructOpt;

#[derive(StructOpt,Debug)]
#[structopt(about = "Select a subcommand")]
enum SubComand{
    #[structopt(name = "testnet")]
    Testnet { 
    },

    #[structopt(name = "btc")]
    Btc {
       
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
	Opt::from_args();
	Ok(())
}
