#[derive(Debug, PartialEq)]
pub struct PtpHost {
    pub clockidentity: u64,
    pub domainnumber: u8,
}

#[cfg(test)]
mod tests {
    #[test]
    fn ptphost() {

    }
}
