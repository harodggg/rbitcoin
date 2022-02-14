use log::*;
use std::path::PathBuf;
use std::{env, fs};
use leveldb::database::Database;
use leveldb::options::{Options};
use std::fmt;



pub struct BlockChainDatabase { 
    db_path: PathBuf,
    db: Database<i32>,
}


impl BlockChainDatabase { 
    fn default(path:String) -> Self { 
        let mut dir = env::current_dir().unwrap();
        dir.push(path);
        let path_buf = dir.clone();
        fs::create_dir_all(dir).unwrap();

        let path = path_buf.as_path();
        let mut options = Options::new();
        options.create_if_missing = true;
        let db = match Database::open(path, options) {
                Ok(db) => { db },
                Err(e) => { panic!("failed to open database: {:?}", e) }
         };
         //let path_str = Box::new(path.to_owned());
         //*path_str = "llg"
     
        BlockChainDatabase { 
            db_path: path_buf.clone(),
            db: db,     
        }
    }
}

impl fmt::Display for BlockChainDatabase{ 
    fn fmt(&self,f: &mut fmt::Formatter)-> fmt::Result { 
        write!(f,"数据库路径是{:?}",self.db_path )
    }
}

pub fn init()  {
    let path =String::from(".db");

    let db = BlockChainDatabase::default(path);
    info!("🟢 初始化数据库,数据库文件路径是: {} ", db);
}
