
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
    
    fn create(node:Node) -> () { 
        //1，建立初使得成员列表
        config = new Config()

        //2，设置node状态
        set.alive
        node.alive();

        //3,进行shedule ，周期性事件
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
}



1, 周期性任务 scheme() -> bool 
    1，故障检测 troubleshooting()
        1,直接探测direct_detection() -> bool { 

        }

        2，间接探测indirect_detectio() -> bool { 

        }

    2，状态合并 state_merge()
        1,发送本地状态信息 send_local_state(message:bytes) -> bool
        2，获取远端状态信息 get_remote_state() -> bytes
        3，将本地状态信息和远端信息合并merge() -> bool { 
            compare_state(local_state,remote_state) -> bool 
            update_state() -> bool { 

            }
        }
    3，广播gossip消息。broadcast(message):
        1，从满足条件的前提节点，选几个节点。
        2，构造广播信息
        3，依次向那些节点发送广播信息


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





