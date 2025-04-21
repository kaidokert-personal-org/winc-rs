use super::{error, info};
use embedded_nal::nb::block;
use embedded_nal::UdpClientStack;

use core::net::{IpAddr, Ipv4Addr, SocketAddr};

use super::Ipv4AddrWrap;

pub fn udp_streamer<T, S>(stack: &mut T, addr: Ipv4Addr, port: u16) -> Result<(), T::Error>
where
    T: UdpClientStack<UdpSocket = S> + ?Sized,
    T::Error: core::fmt::Debug,
{
    let sock = stack.socket();
    if let Ok(mut s) = sock {
        info!(
            "-----connecting to ----- {}.{}.{}.{} port {}",
            addr.octets()[0],
            addr.octets()[1],
            addr.octets()[2],
            addr.octets()[3],
            port
        );
        let remote = SocketAddr::new(IpAddr::V4(addr), port);
        stack.connect(&mut s, remote)?;
        info!("-----Socket connected-----");
        let http_get: &str = "{ \"test\": 15 }";
        let nbytes = block!(stack.send(&mut s, http_get.as_bytes()));
        info!("-----Request sent {:?}-----", nbytes.unwrap());
        stack.close(s)?;
    } else {
        error!("Socket creation failed");
    }
    Ok(())
}
