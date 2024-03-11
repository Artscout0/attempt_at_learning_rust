/*
// You can declare an enum by using the enum keyword. Shocking.
enum IpAddrKind {
    V4,
    V6,
}

// Enums are useful, like in this case, for stuff where you don't know the details of.
// Here, we know we have an IP address, but don't know i it's IPV4 or IPV6.
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main(){
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
*/

// although there are better ways to do the same, as here you can do:
enum IpAddrKind{
    V4(String),
    V6(String),
}

fn main(){
    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let loopback = IpAddrKind::V6(String::from("::1"));
}
// This will make it so that the enum parts are functions, and on declare 
// they will take a value, and on call they'll an enum version return that value.

/*
 * TLDR:
 * enums are useful, use them
 */