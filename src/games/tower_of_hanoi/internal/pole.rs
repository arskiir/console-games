use super::Disk;
use super::MAX_DISK_COUNT;

pub(super) struct Pole {
    pub(crate) disks: Vec<Disk>,
}

impl Pole {
    pub(super) fn build(disks_count: usize) -> Self {
        assert!(disks_count <= MAX_DISK_COUNT);

        let mut disks = Vec::with_capacity(disks_count);
        for size in (1..=disks_count).rev() {
            disks.push(Disk { size });
        }

        Self { disks }
    }
}
