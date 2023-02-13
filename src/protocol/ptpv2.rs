use deku::prelude::*;

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

        let (_rest, val) = PtpAnnouceProtocol::from_bytes(rest).unwrap();
        assert_eq!(val.priority1, 127);
    }
}
