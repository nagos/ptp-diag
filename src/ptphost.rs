#[derive(Debug, PartialEq, Default)]
pub struct PtpHost {
    pub clockidentity: u64,
    pub domainnumber: u8,
    pub announce: bool,
    pub sync: bool,
    pub follow_up: bool,
    pub delay_req: bool,
    pub delay_resp: bool,
}

pub enum PtpHostFlag {
    Announce,
    Sync,
    FollowUp,
    DelayReq,
    DelayResp,
}

impl PtpHost {
    pub fn build(clockidentity: u64, domainnumber: u8) -> PtpHost {
        PtpHost { 
            clockidentity, 
            domainnumber,
            announce: false,
            sync: false, 
            follow_up: false, 
            delay_req: false, 
            delay_resp: false
        }
    }

    pub fn set(&mut self, flag: PtpHostFlag) {
        match flag {
            PtpHostFlag::Announce => self.announce = true,
            PtpHostFlag::Sync => self.sync = true,
            PtpHostFlag::FollowUp => self.follow_up = true,
            PtpHostFlag::DelayReq => self.delay_req = true,
            PtpHostFlag::DelayResp => self.delay_resp = true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ptphost() {
        let mut host = PtpHost::build(0x123456, 1);
        assert_eq!(host.clockidentity, 0x123456);
        assert_eq!(host.domainnumber, 1);
        host.set(PtpHostFlag::Sync);
        assert!(host.sync);
    }
}
