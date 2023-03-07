use std::collections::{BTreeMap, btree_map};
use crate::ptphost::PtpHost;

#[derive(Default)]
pub struct Storage {
    storage: BTreeMap<(u64, u8), PtpHost>,
}

impl Storage {
    pub fn add(&mut self, value: PtpHost) {
        let key = (value.clockidentity, value.domainnumber);
        match self.storage.get(&key) {
            Some(_) => {},
            None => {self.storage.insert(key, value);},
        }
    }
    pub fn into_iter(self) -> btree_map::IntoValues<(u64, u8), PtpHost> {
        self.storage.into_values()
    }
}

#[cfg(test)]
mod tests {
    use crate::ptphost::PtpHost;
    use super::*;

    #[test]
    fn storage() {
        let mut storage = Storage::default();
        storage.add(PtpHost{clockidentity: 1, domainnumber: 127});
        storage.add(PtpHost{clockidentity: 2, domainnumber: 127});
        storage.add(PtpHost{clockidentity: 2, domainnumber: 127});

        let mut itr = storage.into_iter(); 
        assert_eq!(itr.next(), Some(PtpHost{clockidentity: 1, domainnumber: 127}));
        assert_eq!(itr.next(), Some(PtpHost{clockidentity: 2, domainnumber: 127}));
        assert_eq!(itr.next(), None);
    }
}
