
enum State { 
    Alive,
    Suspect,
    Dead,
}

enum Addr {
    Ipv4(u8,u8,u8,u8),
    Ipv6(String)
}

struct Node {
    name: String,
    address: Addr,
    port: u32,
    state: State,
}

struct GossipProtocol {
    config: Config,
    connections: vec![addr]
}

struct Config {
    seed: vec![$node],
    thread_num: u16,

}



trait brockcast { 
    fn brockcast(message: bytes) -> bool { 
        nodes = random(connection,3)
        for node in nodes{
                  localnode.send_tcpStream(message)
        }
    }
}

impl brockcast for GossipProtocol { 
    fn brockcast(Message: bytes) { 
    }
}
trait troubleshooting { 
    fn direct_detection() -> bool { 

    }
    fn indirect_detectio() -> bool { 

    }

}
impl troubleshooting for GossipProtocol{ 

}

trait state_merge { 
    fn send_local_state(message:bytes) -> bool {
    }
    fn  get_remote_state() -> bytes { 
    }
    fn compare_state() -> bool { 

    }
    fn update_state() -> bool { 

    }
    fn merge() -> bool { 
        }
}




impl GossipProtocal { 
    
    fn create(config:&Config) -> () { 
        ///1，建立初使得成员列表
        config = config.default();

        ///2，设置node状态
        set.alive
        node.alive();

        ///3,进行shedule ，周期性事件
        fn shedule() {
            tokio::std::thread::spawn(move || {
                
            }),
            tokio::std::thread::spawn(move || {
                
            }),
            tokio::std:std::thread::spawn(move || {
                
            })

        }    
            Ok()
    }

    fn add_node(node:Node) { 

    }

    fn send_message(node:Node,message:String) -> Result((),Error) { 

    }

    fn brockcast_message(node:vec<Node>,message: String) -> Result((),Error) { 

    }
}


#[cfg(test)]
pub mod Tests{
    
    #[test]
   fn test() -> bool { 
    println!("hello world")
} 
    fn gen() -> bool { 
    println!("gou")
    }
}





