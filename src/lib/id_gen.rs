use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/*

bit anatomy (i64):
_

1 bit: signing bit, should always be positive (zero)
_

44 bits: millisenconds since epoch
 - max of aprox 17592186044415 milliseconds, which is about 565 years
 - should work until the year 2535 CE using UNIX_EPOCH
_

17 bits: sequence, max of 131071
_

2 bits: service_id, max of 4 services
 - having the service id as the least significant bits means the snowflake id
 is roughly sortable by creation order
_

that equates to 131071 unique ids per service per millisecond
i.e. over 131 million unique ids per service per second
i.e. over 524 million unique ids per second using 4 services

*/

pub struct ConcurrentSnowflake {
    inner: Arc<Mutex<Snowflake>>,
}

impl ConcurrentSnowflake {
    pub fn new(service_id: u16) -> Result<Self, SnowflakeError> {
        Ok(Self {
            inner: Arc::new(Mutex::new(Snowflake::with_epoch(service_id, UNIX_EPOCH)?)),
        })
    }

    pub fn with_epoch(service_id: u16, epoch: SystemTime) -> Result<Self, SnowflakeError> {
        Ok(Self {
            inner: Arc::new(Mutex::new(Snowflake::with_epoch(service_id, epoch)?)),
        })
    }

    pub fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }

    pub fn gen(&mut self) -> Result<i64, ConcurrentSnowflakeError> {
        Ok(self
            .inner
            .lock()
            .map_err(|_| {
                ConcurrentSnowflakeError::PoisonError(
                    "lock was poisoned during a previous access and can no longer be locked",
                )
            })?
            .gen())
    }
}

#[derive(Debug)]
pub enum ConcurrentSnowflakeError {
    PoisonError(&'static str),
    SnowflakeError(SnowflakeError),
}

impl std::fmt::Display for ConcurrentSnowflakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PoisonError(e) => write!(f, "{:?}", e),
            Self::SnowflakeError(e) => write!(f, "{:?}", e),
        }
    }
}

impl std::error::Error for ConcurrentSnowflakeError {}

#[derive(Debug)]
pub struct Snowflake {
    epoch: SystemTime,
    service_id: u16,
    last_millis: i64,
    seq: u32,
}

impl Snowflake {
    pub fn new(service_id: u16) -> Result<Self, SnowflakeError> {
        Ok(Self::with_epoch(service_id, UNIX_EPOCH)?)
    }

    pub fn with_epoch(service_id: u16, epoch: SystemTime) -> Result<Self, SnowflakeError> {
        if service_id > 4 {
            return Err(SnowflakeError::InvalidServiceIdError(
                "service id must fit in 4 bits",
            ));
        }
        Ok(Self {
            epoch,
            service_id,
            last_millis: 0,
            seq: 0,
        })
    }

    pub fn gen(&mut self) -> i64 {
        let mut millis = self.get_time_millis();
        if self.seq == 0 && millis == self.last_millis {
            // if the sequence looped in the same millisecond, wait a millisecond
            sleep(Duration::from_millis(1));
            millis = self.get_time_millis();
        };
        self.last_millis = millis;
        millis << 19 | ((self.next_seq()) << 2) as i64 | self.service_id as i64
    }

    fn next_seq(&mut self) -> u32 {
        self.seq = (self.seq + 1) % 131071;
        self.seq
    }

    fn get_time_millis(&self) -> i64 {
        SystemTime::now()
            .duration_since(self.epoch)
            .unwrap()
            .as_millis() as i64
    }
}

#[derive(Debug)]
pub enum SnowflakeError {
    InvalidServiceIdError(&'static str),
}

impl std::fmt::Display for SnowflakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidServiceIdError(e) => write!(f, "{:?}", e),
        }
    }
}

impl std::error::Error for SnowflakeError {}

#[cfg(test)]
mod tests {
    use super::*;

    const NUM_IDS: u64 = 1_000_000;

    #[test]
    fn test_snowflake_creates_unique_positive_ids() {
        let mut snowflake = Snowflake::new(0).unwrap();
        let mut ids: Vec<i64> = Vec::new();
        for _ in 0..NUM_IDS {
            ids.push(snowflake.gen());
        }
        ids.sort();
        ids.dedup();
        ids = ids.into_iter().filter(|id| *id > 0).collect();
        assert_eq!(ids.len(), NUM_IDS as usize);
    }

    #[test]
    fn test_snowflake_concurrently_creates_unique_positive_ids() {
        use std::thread::spawn;

        let snowflake = ConcurrentSnowflake::new(0).unwrap();

        let mut clone1 = snowflake.clone();
        let ids_thread_one = spawn(move || {
            let mut ids: Vec<i64> = Vec::new();
            for _ in 0..NUM_IDS {
                ids.push(clone1.gen().unwrap());
            }
            ids
        });

        let mut clone2 = snowflake.clone();
        let ids_thread_two = spawn(move || {
            let mut ids: Vec<i64> = Vec::new();
            for _ in 0..NUM_IDS {
                ids.push(clone2.gen().unwrap());
            }
            ids
        });

        let mut clone3 = snowflake.clone();
        let ids_thread_three = spawn(move || {
            let mut ids: Vec<i64> = Vec::new();
            for _ in 0..NUM_IDS {
                ids.push(clone3.gen().unwrap());
            }
            ids
        });

        let mut clone4 = snowflake.clone();
        let ids_thread_four = spawn(move || {
            let mut ids: Vec<i64> = Vec::new();
            for _ in 0..NUM_IDS {
                ids.push(clone4.gen().unwrap());
            }
            ids
        });

        let mut ids: Vec<i64> = Vec::new();
        ids.extend(ids_thread_one.join().unwrap());
        ids.extend(ids_thread_two.join().unwrap());
        ids.extend(ids_thread_three.join().unwrap());
        ids.extend(ids_thread_four.join().unwrap());

        ids.sort();
        ids.dedup();
        ids = ids.into_iter().filter(|id| *id > 0).collect();
        assert_eq!(ids.len(), (NUM_IDS * 4) as usize);
    }
}
