use std::collections::{BTreeMap, btree_map};
use std::collections::btree_map::Entry::{Vacant, Occupied};
use crate::ptphost::{PtpHost, PtpHostFlag};

#[derive(Default)]
pub struct Storage {
    storage: BTreeMap<(u64, u8), PtpHost>,
}

impl Storage {
    pub fn add(&mut self, clock:u64, domain: u8, flag: PtpHostFlag) -> Option<u8> {
        let ret;
        let key = (clock, domain);
        let host = match self.storage.entry(key) {
            Vacant(entry) => {
                ret = Some(domain);
                entry.insert(PtpHost::build(clock, domain))
            },
            Occupied(entry) => {
                ret = None;
                entry.into_mut()
            },
        };
        host.set(flag);
        ret
    }
    pub fn into_values(self) -> btree_map::IntoValues<(u64, u8), PtpHost> {
        self.storage.into_values()
    }
    #[allow(dead_code)]
    pub fn values(&self) -> btree_map::Values<'_, (u64, u8), PtpHost> {
        self.storage.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ptphost::PtpHostFlag;

    #[test]
    fn storage() {
        let mut storage = Storage::default();
        let r1 = storage.add(1, 127, PtpHostFlag::Announce);
        let r2 = storage.add(1, 127, PtpHostFlag::Sync);
        let r3 = storage.add(2, 127, PtpHostFlag::Announce);

        assert!(r1.is_some());
        assert_eq!(r1.unwrap(), 127);
        assert!(r2.is_none());
        assert!(r3.is_some());
        let mut itr = storage.into_values();
        let item_1 = itr.next().unwrap();
        assert!(item_1.sync);
        assert!(item_1.announce);
        assert_eq!(item_1.domainnumber, 127);
        assert!(matches!(itr.next(), Some(..)));
        assert!(matches!(itr.next(), None));
    }
}
