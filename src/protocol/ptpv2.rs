use deku::prelude::*;

const MGS_SYNC: u8 = 0x00;
const MGS_DELAY_REQ: u8 = 0x01;
const MGS_FOLLOW_UP: u8 = 0x08;
const MGS_DELAY_RESP: u8 = 0x09;
const MGS_ANNOUNCE: u8 = 0x0b;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
struct PtpHeaderProtocol {
    #[deku(bits = "4")]
    majorsdoid: u8,
    #[deku(bits = "4")]
    messagetype: u8,
    #[deku(bits = "4")]
    minorversionptp: u8,
    #[deku(bits = "4")]
    versionptp: u8,
    messagelength: u16,
    domainnumber: u8,
    minorsdoid: u8,
    flags: u16,
    #[deku(bits = "48")]
    correction_ns: u64,
    correction_subns: u16,
    messagetypespecific: u32,
    clockidentity: u64,
    sourceportid: u16,
    sequenceid: u16,
    controlfield: u8,
    logmessageperiod: i8,
    #[deku(bits = "48")]
    origintimestamp_seconds: u64,
    origintimestamp_nanoseconds: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
struct PtpAnnouceProtocol {
    origincurrentutcoffset: u16,
    #[deku(pad_bytes_before = "1")]
    priority1: u8,
    grandmasterclockclass: u8,
    grandmasterclockaccuracy: u8,
    grandmasterclockvariance: u16,
    priority2: u8,
    grandmasterclockidentity: u64,
    localstepsremoved: u16,
    timesource: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
struct PtpDelayResp {
    requestingsourceportidentity: u64,
    requestingsourceportid: u16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn announce_msg() {
        let data = fs::read("src/protocol/test_data/msg_announce.bin").unwrap();
        let (rest, val) = PtpHeaderProtocol::from_bytes((&data, 0)).unwrap();
        assert_eq!(rest.0.len(), 20);
        assert_eq!(val.clockidentity, 0x485b39fffe11a8ab);
        assert_eq!(val.messagetype, MGS_ANNOUNCE);

        let (_rest, val) = PtpAnnouceProtocol::from_bytes(rest).unwrap();
        assert_eq!(val.priority1, 127);
    }

    #[test]
    fn delay_req_msg() {
        let data = fs::read("src/protocol/test_data/msg_delay_req.bin").unwrap();
        let (rest, val) = PtpHeaderProtocol::from_bytes((&data, 0)).unwrap();
        assert_eq!(rest.0.len(), 0);
        assert_eq!(val.clockidentity, 0x485B39FFFE520449);
        assert_eq!(val.messagetype, MGS_DELAY_REQ);
    }

    #[test]
    fn delay_resp_msg() {
        let data = fs::read("src/protocol/test_data/msg_delay_resp.bin").unwrap();
        let (rest, val) = PtpHeaderProtocol::from_bytes((&data, 0)).unwrap();
        assert_eq!(rest.0.len(), 10);
        assert_eq!(val.clockidentity, 0x485b39fffe11a8ab);
        assert_eq!(val.messagetype, MGS_DELAY_RESP);

        let (_rest, val) = PtpDelayResp::from_bytes(rest).unwrap();
        assert_eq!(val.requestingsourceportid, 1);
    }

    #[test]
    fn follow_up_msg() {
        let data = fs::read("src/protocol/test_data/msg_follow_up.bin").unwrap();
        let (rest, val) = PtpHeaderProtocol::from_bytes((&data, 0)).unwrap();
        assert_eq!(rest.0.len(), 0);
        assert_eq!(val.clockidentity, 0x485b39fffe11a8ab);
        assert_eq!(val.messagetype, MGS_FOLLOW_UP);
        assert_eq!(val.origintimestamp_nanoseconds, 521233408);
    }

    #[test]
    fn sync_msg() {
        let data = fs::read("src/protocol/test_data/msg_sync.bin").unwrap();
        let (rest, val) = PtpHeaderProtocol::from_bytes((&data, 0)).unwrap();
        assert_eq!(rest.0.len(), 0);
        assert_eq!(val.clockidentity, 0x485b39fffe11a8ab);
        assert_eq!(val.messagetype, MGS_SYNC);
    }
}
