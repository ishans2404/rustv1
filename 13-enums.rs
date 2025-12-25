// Enums

fn main() {
    enum IpAddrVer {
        V4,
        V6
    }

    let ip4 = IpAddrVer::V4;
    let ip6 = IpAddrVer::V6;

    fn route(ip_ver: IpAddrVer) {
    }

    route(ip_ver: IpAddrVer::V4);

    // using structs
    struct IpAddr{
        ver: IpAddrVer,
        address: String
    }

    // let home = IpAddr{
    //     ver: IpAddrVer::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr{
    //     ver: IpAddrVer::V6,
    //     address: String::from("::1"),
    // };

    // using enums
    enum IpAddrVersion {
        V4(String),
        V6(String),
    }

    let home = IpAddrVersion::V4(String::from("127.0.0.1"));

}