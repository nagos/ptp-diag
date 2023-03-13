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
use std::thread;
use std::sync::mpsc;

const PTP_ADDRESSS: &str = "224.0.1.129";
const LOCAL_ADDRESSS: &str = "0.0.0.0:320";
const LOCAL_ADDRESSS2: &str = "0.0.0.0:319";
const PTP_DST_ADDRESSS: &str = "224.0.1.129:319";

fn parse_msg(msg: PtpMsg, storage: &mut Storage) -> Option<u8> {
    match msg {
        PtpMsg::Announce(d) => storage.add(d.clockidentity, d.domain, PtpHostFlag::Announce),
        PtpMsg::DelayReq(_) => None,
        PtpMsg::DelayResp(d) => storage.add(d.clockidentity, d.domain, PtpHostFlag::DelayResp),
        PtpMsg::FollowUp(d) => storage.add(d.clockidentity, d.domain, PtpHostFlag::FollowUp),
        PtpMsg::Sync(d) => storage.add(d.clockidentity, d.domain, PtpHostFlag::Sync),
    }
}

fn receive_loop(socket: UdpSocket, channel: mpsc::Sender<PtpMsg>, channel_req: Option<mpsc::Receiver<u8>>) {
    loop {
        let mut buf = [0; 106];
        
        if let Ok(received) = socket.recv(&mut buf) {
            let msg = PtpMsg::parse(&buf[..received]).unwrap();
            channel.send(msg).unwrap();
        }

        if let Some(ref x) = channel_req {
            if let Ok(domain) = x.try_recv() {
                send_delay_req(&socket, domain);
            }
        }
    }
}

fn send_delay_req(socket: &UdpSocket, domain: u8) {
    let buf = PtpMsg::build(PtpMsg::DelayReq(PtpData{clockidentity: 0x123, domain}));
    socket.send_to(&buf, PTP_DST_ADDRESSS).unwrap();
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
    let (tx, rx) = mpsc::channel::<PtpMsg>();
    let tx2 = tx.clone();
    let (tx_req, rx_req) = mpsc::channel::<u8>();

    thread::spawn(move || receive_loop(socket, tx, None));
    thread::spawn(move || receive_loop(socket2, tx2, Some(rx_req)));
    
    let start_time = SystemTime::now();
    while start_time.elapsed().unwrap() <= Duration::from_secs(5){
        if let Ok(msg) = rx.recv_timeout(Duration::from_millis(100)){
            if let Some(domain) = parse_msg(msg, &mut storage) {
                tx_req.send(domain).unwrap();
            }
        }
    }

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
