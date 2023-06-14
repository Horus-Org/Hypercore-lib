use std::io::{Read, Write};
use hypercore::Feed;

pub struct HypercoreLibrary {
    feed: Feed,
}

impl HypercoreLibrary {
    pub fn new(feed: Feed) -> Self {
        HypercoreLibrary { feed }
    }

    pub fn read_data(&mut self, start: usize, length: usize) -> Result<Vec<u8>, hypercore::Error> {
        let mut buf = vec![0u8; length];
        self.feed.read(start, &mut buf)?;
        Ok(buf)
    }

    pub fn append_data(&mut self, data: &[u8]) -> Result<(), hypercore::Error> {
        self.feed.append(data)?;
        Ok(())
    }
}

