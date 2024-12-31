use embedded_nal::nb::block;
use embedded_nal::UdpClientStack;

use core::net::Ipv4Addr;

use wincwifi::transfer::PrefixXfer;
use wincwifi::WincClient;

fn run_udp_connect<S, E>(stack: &mut S) -> Result<(), E>
where
    E: core::fmt::Debug,
    S: UdpClientStack<Error = E>,
{
    let ip = core::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let sockaddr = core::net::SocketAddr::new(ip, 1600);
    let mut sock = stack.socket()?;
    stack.connect(&mut sock, sockaddr)?;
    block!(stack.send(&mut sock, b"Hello, world!"))?;
    Ok(())
}

#[test]
fn test_udp_connect() {
    let mut client = WincClient::<PrefixXfer<&mut [u8]>>::new();

    //run_udp_connect(&mut client).unwrap();
}
