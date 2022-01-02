
enum State { 
    Alive,
    Suspect,
    Dead,
}

enum Address {
    Ipv4(u8,u8,u8,u8),
    Ipv6(String)
}

struct Node {
    name: String,
    address: Address,
    port: u32,
    state: State,
}

1, 周期性任务
    1，故障检测
        1,直接探测
        2，间接探测
    2，状态合并
        1,发送本地状态信息
        2，获取远端状态信息
        3，将本地状态信息和远端信息合并
    3，广播gossip消息。
        1，从满足条件的前提节点，选几个节点。
        2，构造广播信息
        3，依次向那些节点发送广播信息


fn s
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





