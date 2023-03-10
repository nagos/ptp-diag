use deku::prelude::*;

pub const MGS_SYNC: u8 = 0x00;
pub const MGS_DELAY_REQ: u8 = 0x01;
pub const MGS_FOLLOW_UP: u8 = 0x08;
pub const MGS_DELAY_RESP: u8 = 0x09;
pub const MGS_ANNOUNCE: u8 = 0x0b;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct PtpHeaderProtocol {
    #[deku(bits = "4")]
    pub majorsdoid: u8,
    #[deku(bits = "4")]
    pub messagetype: u8,
    #[deku(bits = "4")]
    pub minorversionptp: u8,
    #[deku(bits = "4")]
    pub versionptp: u8,
    pub messagelength: u16,
    pub domainnumber: u8,
    pub minorsdoid: u8,
    pub flags: u16,
    #[deku(bits = "48")]
    pub correction_ns: u64,
    pub correction_subns: u16,
    pub messagetypespecific: u32,
    pub clockidentity: u64,
    pub sourceportid: u16,
    pub sequenceid: u16,
    pub controlfield: u8,
    pub logmessageperiod: i8,
    #[deku(bits = "48")]
    pub origintimestamp_seconds: u64,
    pub origintimestamp_nanoseconds: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct PtpAnnouceProtocol {
    pub origincurrentutcoffset: u16,
    #[deku(pad_bytes_before = "1")]
    pub priority1: u8,
    pub grandmasterclockclass: u8,
    pub grandmasterclockaccuracy: u8,
    pub grandmasterclockvariance: u16,
    pub priority2: u8,
    pub grandmasterclockidentity: u64,
    pub localstepsremoved: u16,
    pub timesource: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
struct PtpDelayResp {
    requestingsourceportidentity: u64,
    requestingsourceportid: u16,
}

impl PtpHeaderProtocol {
    pub fn build(messagetype: u8, messagelength: u16, domainnumber: u8, clockidentity: u64, sequenceid: u16, controlfield: u8) -> PtpHeaderProtocol {
        PtpHeaderProtocol { 
            majorsdoid: 0, 
            messagetype, 
            minorversionptp: 0, 
            versionptp: 2, 
            messagelength, 
            domainnumber, 
            minorsdoid: 0, 
            flags: 0, 
            correction_ns: 0, 
            correction_subns: 0, 
            messagetypespecific: 0, 
            clockidentity, 
            sourceportid: 1, 
            sequenceid, 
            controlfield, 
            logmessageperiod: 127, 
            origintimestamp_seconds: 0, 
            origintimestamp_nanoseconds: 0
        }
    }
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

    #[test]
    fn header_build() {
        let msg = PtpHeaderProtocol::build(MGS_DELAY_REQ, 44, 123, 0x123, 1, MGS_DELAY_REQ);
        assert_eq!(msg.messagetype, MGS_DELAY_REQ);
        assert_eq!(msg.domainnumber, 123);
        assert_eq!(msg.clockidentity, 0x123);
    }
}
