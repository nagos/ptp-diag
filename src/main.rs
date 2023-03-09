use std::net::Ipv4Addr;
use std::net::UdpSocket;
mod protocol;
mod storage;
mod ptphost;
use std::str::FromStr;
use protocol::ptpv2::PtpMsg;
use crate::ptphost::PtpHostFlag;
use std::time::Duration;
use std::time::SystemTime;

const PTP_ADDRESSS: &str = "224.0.1.129";
const LOCAL_ADDRESSS: &str = "0.0.0.0:320";

fn main() {
    let socket = UdpSocket::bind(LOCAL_ADDRESSS).expect("couldn't bind to address");
    socket.join_multicast_v4(
        &Ipv4Addr::from_str(PTP_ADDRESSS).unwrap(),
        &Ipv4Addr::UNSPECIFIED
    ).expect("Could not join multicast group");
    socket.set_read_timeout(Some(Duration::from_millis(1000))).expect("set_read_timeout call failed");

    let mut storage = storage::Storage::default();
    let start_time = SystemTime::now();
    loop {
        let mut buf = [0; 106];
        match socket.recv(&mut buf) {
            Ok(received) => {
                let msg = PtpMsg::build(&buf[..received]).unwrap();
                match msg {
                    PtpMsg::Announce(d) => storage.add(d.grandmasterclockidentity, d.domainnumber, PtpHostFlag::Announce),
                    PtpMsg::DelayReq(d) => storage.add(d.clockidentity, d.domainnumber, PtpHostFlag::DelayReq),
                    PtpMsg::DelayResp(d) => storage.add(d.clockidentity, d.domainnumber, PtpHostFlag::DelayResp),
                    PtpMsg::FollowUp(d) => storage.add(d.clockidentity, d.domainnumber, PtpHostFlag::FollowUp),
                    PtpMsg::Sync(d) => storage.add(d.clockidentity, d.domainnumber, PtpHostFlag::Sync),
                }
            },
            Err(_) => {
            }
        }
        if start_time.elapsed().unwrap() > Duration::from_secs(5) {
            break;
        }
    }

    for i in storage.into_iter() {
        dbg!(i);
    }

}
