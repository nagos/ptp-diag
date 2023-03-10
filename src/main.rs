mod protocol;
mod storage;
mod ptphost;

use std::net::Ipv4Addr;
use std::net::UdpSocket;
use std::str::FromStr;
use protocol::ptpv2::PtpMsg;
use crate::ptphost::PtpHostFlag;
use std::time::Duration;
use std::time::SystemTime;
use crate::protocol::ptpv2::PtpData;
use storage::Storage;

const PTP_ADDRESSS: &str = "224.0.1.129";
const LOCAL_ADDRESSS: &str = "0.0.0.0:320";
const LOCAL_ADDRESSS2: &str = "0.0.0.0:319";
const PTP_DST_ADDRESSS: &str = "224.0.1.129:319";

fn parse_msg(received: &[u8], storage: &mut Storage) {
    let msg = PtpMsg::new(received).unwrap();
    match msg {
        PtpMsg::Announce(d) => storage.add(d.grandmasterclockidentity, d.domainnumber, PtpHostFlag::Announce),
        PtpMsg::DelayReq(_) => {},
        PtpMsg::DelayResp(d) => storage.add(d.clockidentity, d.domainnumber, PtpHostFlag::DelayResp),
        PtpMsg::FollowUp(d) => storage.add(d.clockidentity, d.domainnumber, PtpHostFlag::FollowUp),
        PtpMsg::Sync(d) => storage.add(d.clockidentity, d.domainnumber, PtpHostFlag::Sync),
    }
}

fn receive_loop(socket: &UdpSocket, socket2: &UdpSocket, storage: &mut Storage, delay: u64) {
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

fn bool_to_str(value: bool) -> String {
    if value {
        String::from("+")
    } else {
        String::from("-")
    }
}

fn main() {
    let socket = open_socket(LOCAL_ADDRESSS, PTP_ADDRESSS);
    let socket2 = open_socket(LOCAL_ADDRESSS2, PTP_ADDRESSS);
    let mut storage = Storage::default();

    receive_loop(&socket, &socket2, &mut storage, 5);
    for i in storage.values() {
        let buf = PtpMsg::build(PtpMsg::DelayReq(PtpData{clockidentity: 0x123, domainnumber: i.domainnumber}));
        socket2.send_to(&buf, PTP_DST_ADDRESSS).unwrap();
    }
    receive_loop(&socket, &socket2, &mut storage, 1);

    println!("{:016} {:6} {:9} {:9} {:9} {:9}", 
        "ID", 
        "Domain", 
        "Announce", 
        "Sync", 
        "Follow Up", 
        "Delay Resp"
    );
    for i in storage.into_values() {
        println!("{:<016x} {:<6} {:9} {:9} {:9} {:9}", 
            i.clockidentity, 
            i.domainnumber, 
            bool_to_str(i.announce), 
            bool_to_str(i.sync), 
            bool_to_str(i.follow_up), 
            bool_to_str(i.delay_resp)
        )
    }

}
