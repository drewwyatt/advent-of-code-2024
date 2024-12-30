use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Block {
    File(usize),
    Free,
}

pub struct Disk {
    blocks: Vec<Block>,
    _last_file_index: Option<usize>,
    last_compacted_index: usize,
}

impl Disk {
    pub fn defrag(&mut self) -> anyhow::Result<()> {
        let mut counter = 100_000;
        while self.is_fragmented() {
            self.move_next()?;
            counter -= 1;
            if counter == 0 {
                return Err(anyhow::anyhow!("This is taking too long."));
            }
        }

        Ok(())
    }

    pub fn checksum(&self) -> i64 {
        self.blocks
            .iter()
            .enumerate()
            .fold(0, |acc, (position, block)| match block {
                Block::File(id) => acc + (position * id) as i64,
                Block::Free => acc,
            })
    }

    fn move_next(&mut self) -> anyhow::Result<()> {
        let last_file_index = self.last_file_index();
        let first_free_index = self
            .first_free_index()
            .ok_or(anyhow::anyhow!("Could not find free index"))?;

        self.blocks[first_free_index] = self.blocks[last_file_index];
        self.blocks[last_file_index] = Block::Free;
        self.last_compacted_index = first_free_index;
        self._last_file_index = None;

        Ok(())
    }

    fn first_free_index(&self) -> Option<usize> {
        self.blocks[self.last_compacted_index..]
            .iter()
            .position(|b| matches!(b, Block::Free))
            .map(|pos| pos + self.last_compacted_index)
    }

    fn is_fragmented(&mut self) -> bool {
        let last_file_index = self.last_file_index();
        let last_compacted_index = self.last_compacted_index;

        if last_file_index > last_compacted_index {
            for (index, block) in self.blocks[last_compacted_index..=last_file_index]
                .iter()
                .enumerate()
            {
                if matches!(block, Block::Free) {
                    return true;
                }
                self.last_compacted_index = last_compacted_index + index + 1;
            }
        }

        false
    }

    fn last_file_index(&mut self) -> usize {
        self._last_file_index
            .get_or_insert_with(|| {
                self.blocks
                    .iter()
                    .rposition(|b| matches!(b, Block::File(_)))
                    .expect("Could not find file index") // TODO: I mean this is definitely impossible, but it would still be nice to get rid of this...
            })
            .to_owned()
    }
}

impl fmt::Debug for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.blocks)
    }
}

impl FromStr for Disk {
    type Err = anyhow::Error;

    fn from_str(disk_map: &str) -> Result<Self, Self::Err> {
        let mut id = 0;
        let mut is_file = true;
        const RADIX: u32 = 10;

        let blocks = disk_map.trim().chars().fold(vec![], |mut acc, char| {
            let digit = char
                .to_digit(RADIX)
                .expect(&format!("Cannot parse char '{}' as digit", char)); // TODO: can we avoid this expect?
            let next_block = if is_file {
                let next_id = id;
                id += 1;
                Block::File(next_id)
            } else {
                Block::Free
            };
            is_file = !is_file;

            for _ in 0..digit {
                acc.push(next_block);
            }

            acc
        });

        Ok(Disk {
            blocks,
            _last_file_index: None,
            last_compacted_index: 0,
        })
    }
}
