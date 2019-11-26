fn main() {
    let four = IPAddrKind::V4;

    route(IPAddrKind::V4);

    let home = IPAddr_enum::V4(String::from("127.0.0.1"));
}

enum IPAddrKind {
    V4, // variants
    V6, 
}

struct IPAddr {
    kind: IPAddrKind,
    address: String,
}
// using 'enum' inside 'struct'

enum IPAddr_enum {
    V4(String),
    V6(u8, u8, u8, u8),
}
// more concise, putting data directly into each enum variant
// have different types and mounts of associated data

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
// standard library version

struct Ipv4Addr { }
struct Ipv6Addr { }

fn route(ip_kind: IPAddrKind) { }

// ------------------ //

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// has same definition as 

struct QuitMessage; //unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32);

// but using struct, we can't define a function that takes any kind
// of messages as we could with 'enum'

// ----------------- //
impl Message {
    fn call(&self) {

    }
}
// similar to 'struct', define functions in 'impl' block