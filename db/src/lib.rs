use std::fmt::Display;

use log::*;

#[derive(Default,Debug)]
pub struct Database { 
    path: String

}

pub fn init()  {
    let db = Database::default();
    info!("初始化数据库,数据库文件路径是: {:?}", db);
}