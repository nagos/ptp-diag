use std::collections::{BTreeMap, btree_map};
use crate::ptphost::{PtpHost, PtpHostFlag};

#[derive(Default)]
pub struct Storage {
    storage: BTreeMap<(u64, u8), PtpHost>,
}

impl Storage {
    pub fn add(&mut self, clock:u64, domain: u8, flag: PtpHostFlag) {
        let key = (clock, domain);
        let host = self.storage.entry(key).or_insert(PtpHost::build(clock, domain));
        host.set(flag);
    }
    pub fn into_iter(self) -> btree_map::IntoValues<(u64, u8), PtpHost> {
        self.storage.into_values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ptphost::PtpHostFlag;

    #[test]
    fn storage() {
        let mut storage = Storage::default();
        storage.add(1, 127, PtpHostFlag::Announce);
        storage.add(1, 127, PtpHostFlag::Sync);
        storage.add(2, 127, PtpHostFlag::Announce);

        let mut itr = storage.into_iter();
        let item_1 = itr.next().unwrap();
        assert!(item_1.sync);
        assert!(item_1.announce);
        assert_eq!(item_1.domainnumber, 127);
        assert!(matches!(itr.next(), Some(..)));
        assert!(matches!(itr.next(), None));
    }
}
