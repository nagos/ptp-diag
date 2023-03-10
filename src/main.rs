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
use crate::protocol::ptpv2::PtpData;

const PTP_ADDRESSS: &str = "224.0.1.129";
const LOCAL_ADDRESSS: &str = "0.0.0.0:320";
const LOCAL_ADDRESSS2: &str = "0.0.0.0:319";
const PTP_DST_ADDRESSS: &str = "224.0.1.129:319";

fn parse_msg(received: &[u8], storage: &mut storage::Storage) {
    let msg = PtpMsg::build(received).unwrap();
    match msg {
        PtpMsg::Announce(d) => storage.add(d.grandmasterclockidentity, d.domainnumber, PtpHostFlag::Announce),
        PtpMsg::DelayReq(_) => {},
        PtpMsg::DelayResp(d) => storage.add(d.clockidentity, d.domainnumber, PtpHostFlag::DelayResp),
        PtpMsg::FollowUp(d) => storage.add(d.clockidentity, d.domainnumber, PtpHostFlag::FollowUp),
        PtpMsg::Sync(d) => storage.add(d.clockidentity, d.domainnumber, PtpHostFlag::Sync),
    }
}

fn receive_loop(socket: &UdpSocket, socket2: &UdpSocket, storage: &mut storage::Storage, delay: u64) {
    let start_time = SystemTime::now();
    while start_time.elapsed().unwrap() <= Duration::from_secs(delay) {
        let mut buf = [0; 106];
        
        if let Ok(received) = socket.recv(&mut buf) {
                parse_msg(&buf[..received], storage);
        }

        if let Ok(received) = socket2.recv(&mut buf) {
            parse_msg(&buf[..received], storage);
        }
    }
}

fn open_socket(local_address: &str, ptp_address: &str) -> UdpSocket {
    let socket = UdpSocket::bind(local_address).expect("couldn't bind to address");
    socket.join_multicast_v4(
        &Ipv4Addr::from_str(ptp_address).unwrap(),
        &Ipv4Addr::UNSPECIFIED
    ).expect("Could not join multicast group");
    socket.set_read_timeout(Some(Duration::from_millis(100))).expect("set_read_timeout call failed");

    socket
}

fn main() {
    let socket = open_socket(LOCAL_ADDRESSS, PTP_ADDRESSS);
    let socket2 = open_socket(LOCAL_ADDRESSS2, PTP_ADDRESSS);
    let mut storage = storage::Storage::default();

    receive_loop(&socket, &socket2, &mut storage, 5);
    for i in storage.values() {
        let buf = PtpMsg::new(PtpMsg::DelayReq(PtpData{clockidentity: 0x123, domainnumber: i.domainnumber}));
        socket2.send_to(&buf, PTP_DST_ADDRESSS).unwrap();
    }
    receive_loop(&socket, &socket2, &mut storage, 1);

    for i in storage.into_iter() {
        dbg!(i);
    }

}
