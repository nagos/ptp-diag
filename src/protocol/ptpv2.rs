use std::fmt::Display;
use crate::protocol::error::Error;
use crate::protocol::messages::{
        PtpHeaderProtocol, 
        PtpAnnouceProtocol, 
        MGS_SYNC,
        MGS_ANNOUNCE, 
        MGS_DELAY_REQ,
        MGS_FOLLOW_UP,
        MGS_DELAY_RESP,
};
use deku::prelude::*;

#[derive(Debug)]
pub enum PtpMsg {
    Sync(PtpData),
    DelayReq(PtpData),
    FollowUp(PtpData),
    DelayResp(PtpData),
    Announce(AnnounceData),
}

#[derive(Debug)]
pub struct AnnounceData {
    pub clockidentity: u64,
    pub domain: u8,
}

#[derive(Debug)]
pub struct PtpData {
    pub clockidentity: u64,
    pub domain: u8,
}

impl PtpMsg {
    pub fn parse(data: &[u8]) -> Result<PtpMsg, Error> {
        let (rest, val) = PtpHeaderProtocol::from_bytes((data, 0)).map_err(|_|Error)?;
        let ptp_data = PtpData{clockidentity: val.clockidentity, domain: val.domainnumber};
        match val.messagetype {
            MGS_ANNOUNCE => {
                let (_, val) = PtpAnnouceProtocol::from_bytes(rest).map_err(|_|Error)?;
                Ok(PtpMsg::Announce(AnnounceData{
                    clockidentity: val.grandmasterclockidentity,
                    domain: ptp_data.domain,
                }))
            },
            MGS_DELAY_REQ => Ok(PtpMsg::DelayReq(ptp_data)),
            MGS_FOLLOW_UP => Ok(PtpMsg::FollowUp(ptp_data)),
            MGS_DELAY_RESP => Ok(PtpMsg::DelayResp(ptp_data)),
            MGS_SYNC => Ok(PtpMsg::Sync(ptp_data)),
            _ => Err(Error),
        }
    }

    pub fn build(msg: PtpMsg) -> Vec<u8> {
        match msg {
            PtpMsg::DelayReq(x) => PtpHeaderProtocol::build(
                MGS_DELAY_REQ, 
                44, 
                x.domain, 
                x.clockidentity, 
                1, 
                MGS_DELAY_REQ
            ).to_bytes().unwrap(),
            _ => panic!("Unsupported message"),
        }
    }
}

impl Display for PtpMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PtpMsg::Sync(x) => write!(f, "Sync {:X}", x.clockidentity),
            PtpMsg::DelayReq(x) => write!(f, "Delay_Req {:X}", x.clockidentity),
            PtpMsg::FollowUp(x) => write!(f, "Follow_Up {:X}", x.clockidentity),
            PtpMsg::DelayResp(x) => write!(f, "Delay_Resp {:X}", x.clockidentity),
            PtpMsg::Announce(x) => write!(f, "Announce {:X}", x.clockidentity),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn build_announce(){
        let data = fs::read("src/protocol/test_data/msg_announce.bin").unwrap();
        let msg = PtpMsg::parse(&data);
        assert!(matches!(msg, Ok(PtpMsg::Announce(_))));
        if let Ok(PtpMsg::Announce(data)) = msg {
            assert_eq!(data.clockidentity, 0x485b39fffe11a8ab);
        }
    }

    #[test]
    fn build_delay_req(){
        let data = fs::read("src/protocol/test_data/msg_delay_req.bin").unwrap();
        let msg = PtpMsg::parse(&data);
        assert!(matches!(msg, Ok(PtpMsg::DelayReq(_))));
        if let Ok(PtpMsg::DelayReq(data)) = msg {
            assert_eq!(data.clockidentity, 0x485B39FFFE520449);
        }
    }

    #[test]
    fn build_follow_up(){
        let data = fs::read("src/protocol/test_data/msg_follow_up.bin").unwrap();
        let msg = PtpMsg::parse(&data);
        assert!(matches!(msg, Ok(PtpMsg::FollowUp(_))));
        if let Ok(PtpMsg::FollowUp(data)) = msg {
            assert_eq!(data.clockidentity, 0x485b39fffe11a8ab);
        }
    }

    #[test]
    fn build_delay_resp(){
        let data = fs::read("src/protocol/test_data/msg_delay_resp.bin").unwrap();
        let msg = PtpMsg::parse(&data);
        assert!(matches!(msg, Ok(PtpMsg::DelayResp(_))));
        if let Ok(PtpMsg::DelayResp(data)) = msg {
            assert_eq!(data.clockidentity, 0x485b39fffe11a8ab);
        }
    }

    #[test]
    fn build_sync(){
        let data = fs::read("src/protocol/test_data/msg_sync.bin").unwrap();
        let msg = PtpMsg::parse(&data);
        assert!(matches!(msg, Ok(PtpMsg::Sync(_))));
        if let Ok(PtpMsg::Sync(data)) = msg {
            assert_eq!(data.clockidentity, 0x485b39fffe11a8ab);
        }
    }

    #[test]
    fn display(){
        let data = fs::read("src/protocol/test_data/msg_sync.bin").unwrap();
        let msg = PtpMsg::parse(&data).unwrap();
        let s = format!("{}", msg);
        assert_eq!(s, "Sync 485B39FFFE11A8AB");
    }

    #[test]
    fn create() {
        let data = PtpMsg::build(PtpMsg::DelayReq(PtpData{clockidentity: 0x123, domain: 123}));
        let msg = PtpMsg::parse(&data).unwrap();
        assert!(matches!(msg, PtpMsg::DelayReq(..)));
        if let PtpMsg::DelayReq(x) = msg {
            assert_eq!(x.clockidentity, 0x123);
            assert_eq!(x.domain, 123);
        }
    }
}
