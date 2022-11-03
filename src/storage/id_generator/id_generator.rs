use std::{time::{SystemTime, UNIX_EPOCH}, cmp::max};


#[derive(Debug)]
pub struct IdGenerator {
    instance: i64,
    sequence: i64,
    timestamp: i64,
}

impl IdGenerator {
    pub fn new(instance: i64) -> Self {
        Self {
            instance: instance & 0x3ff,
            sequence: 0,
            timestamp: 0,
        }
    }

    pub fn create(&mut self) -> i64 {
        let mut selected;

        loop {
            let since_the_epoch = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as i64;

            selected = max(self.timestamp, since_the_epoch);
            
            if selected == self.timestamp {
                self.sequence += 1;
            }
            else {
                self.sequence = 0;
            }

            if self.sequence <= 0xff {
                break;
            }
        }

        let id = (selected << 18) | (self.sequence << 10) | self.instance;

        self.timestamp = selected;
        
        id
    }
}