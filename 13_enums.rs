// Enum
// A versatile tool used to represent a type 
// that can take on one of several possible variants 

fn main(){
    
    enum IpAddrKind {
        V4,
        V6
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    fn route(ip_kind: IpAddrKind){}

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // Using structs 
    struct IpAddr{
        kind: IpAddrKind,
        address: String
    }

    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let knocback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    // better way to use them
    enum IpAddrKindNew{
        V4(String),
        V6(String)
    }

    let home_new = IpAddrKindNew::V4(String::from("127.0.0.1"));
    let potato_pc = IpAddrKindNew::V6(String::from("::1"));


    enum IpAddrDifferent{
        V4(u8,u8,u8,u8),
        V6(String)
    }

    let home_ipv4 = IpAddrDifferent::V4(127,0,0,1);
    let master_pc = IpAddrDifferent::V6(String::from("::1"));

    // TODO - Learn how and why these enums are used!
}