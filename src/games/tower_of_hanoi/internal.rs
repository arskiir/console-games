use std::{
    error::Error,
    io::{stdin, stdout, Write},
};
mod error;

const POLE_COUNT: usize = 3;
const MAX_DISK_COUNT: usize = 3;

struct Disk {
    size: usize,
}

struct Pole {
    disks: Vec<Disk>,
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

pub(super) struct TowerOfHanoi {
    poles: [Pole; POLE_COUNT],
}

pub(super) struct PromptDiskMoveResult {
    pub(super) from: usize,
    pub(super) to: usize,
}

impl TowerOfHanoi {
    /// print the poles and pole numbers
    pub(super) fn render(&self) {
        // print to poles
        for i in (0..MAX_DISK_COUNT).rev() {
            for pole in self.poles.iter() {
                if let Some(disk) = pole.disks.get(i) {
                    print!("|{}|", disk.size);
                } else {
                    print!("| |");
                }
            }
            println!();
        }
        // print pole numbers
        for i in 0..POLE_COUNT {
            print!(" {} ", i + 1);
        }
        println!();
    }

    pub(super) fn prompt_disk_move(&mut self) -> Result<PromptDiskMoveResult, Box<dyn Error>> {
        let mut input = String::new();
        print!("From: ");
        stdout().flush()?;
        stdin().read_line(&mut input)?;
        let mut from: usize = input.trim().parse()?;
        from -= 1;
        if self.poles.get(from).is_none() {
            return Err(Box::new(error::ParseDiskMoveError::new(from)));
        }

        input.clear();

        print!("To: ");
        stdout().flush()?;
        stdin().read_line(&mut input)?;
        let mut to: usize = input.trim().parse()?;
        to -= 1;
        if self.poles.get(to).is_none() {
            Err(Box::new(error::ParseDiskMoveError::new(to)))
        } else {
            Ok(PromptDiskMoveResult { from, to })
        }
    }

    /// move a disk from a pole to another pole
    pub(super) fn move_disk(&mut self, from: usize, to: usize) -> Result<(), &'static str> {
        let poles = self.poles.as_mut();
        if let Some(disk) = poles[from].disks.pop() {
            poles[to].disks.push(disk);
            Ok(())
        } else {
            Err("This pole has no disk")
        }
    }

    pub(super) fn win(&self) -> bool {
        let disks = &self.poles[POLE_COUNT - 1].disks;
        disks.len() == MAX_DISK_COUNT && disks.windows(2).all(|w| w[0].size > w[1].size)
    }
}

impl Default for TowerOfHanoi {
    fn default() -> Self {
        Self {
            poles: [Pole::build(MAX_DISK_COUNT), Pole::build(0), Pole::build(0)],
        }
    }
}
