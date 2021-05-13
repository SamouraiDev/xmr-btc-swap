use crate::bitcoin::{CancelTimelock, PunishTimelock};
use std::cmp::max;
use std::time::Duration;
use time::NumericalStdDurationShort;

#[derive(Debug, Copy, Clone)]
pub struct Config {
    pub bitcoin_lock_confirmed_timeout: Duration,
    pub bitcoin_finality_confirmations: u32,
    pub bitcoin_avg_block_time: Duration,
    pub bitcoin_cancel_timelock: CancelTimelock,
    pub bitcoin_punish_timelock: PunishTimelock,
    pub bitcoin_network: bitcoin::Network,
    pub monero_avg_block_time: Duration,
    pub monero_finality_confirmations: u64,
    pub monero_network: monero::Network,
}

impl Config {
    pub fn bitcoin_sync_interval(&self) -> Duration {
        sync_interval(self.bitcoin_avg_block_time)
    }

    pub fn monero_sync_interval(&self) -> Duration {
        sync_interval(self.monero_avg_block_time)
    }

    pub fn with_bitcoin_finality_confirmations(
        mut self,
        bitcoin_finality_confirmations: u32,
    ) -> Config {
        self.bitcoin_finality_confirmations = bitcoin_finality_confirmations;
        self
    }

    pub fn with_monero_finality_confirmations(
        mut self,
        monero_finality_confirmations: u64,
    ) -> Config {
        self.monero_finality_confirmations = monero_finality_confirmations;
        self
    }
}

pub trait GetConfig {
    fn get_config() -> Config;
}

#[derive(Clone, Copy)]
pub struct Mainnet;

#[derive(Clone, Copy)]
pub struct Testnet;

#[derive(Clone, Copy)]
pub struct Regtest;

impl GetConfig for Mainnet {
    fn get_config() -> Config {
        Config {
            bitcoin_lock_confirmed_timeout: 24.hours(),
            bitcoin_finality_confirmations: 3,
            bitcoin_avg_block_time: 10.minutes(),
            bitcoin_cancel_timelock: CancelTimelock::new(72),
            bitcoin_punish_timelock: PunishTimelock::new(72),
            bitcoin_network: bitcoin::Network::Bitcoin,
            monero_avg_block_time: 2.minutes(),
            monero_finality_confirmations: 15,
            monero_network: monero::Network::Mainnet,
        }
    }
}

impl GetConfig for Testnet {
    fn get_config() -> Config {
        Config {
            bitcoin_lock_confirmed_timeout: 12.hours(),
            bitcoin_finality_confirmations: 1,
            bitcoin_avg_block_time: 5.minutes(),
            bitcoin_cancel_timelock: CancelTimelock::new(12),
            bitcoin_punish_timelock: PunishTimelock::new(6),
            bitcoin_network: bitcoin::Network::Testnet,
            monero_avg_block_time: 2.minutes(),
            monero_finality_confirmations: 10,
            monero_network: monero::Network::Stagenet,
        }
    }
}

impl GetConfig for Regtest {
    fn get_config() -> Config {
        Config {
            bitcoin_lock_confirmed_timeout: 1.minutes(),
            bitcoin_finality_confirmations: 1,
            bitcoin_avg_block_time: 5.seconds(),
            bitcoin_cancel_timelock: CancelTimelock::new(100),
            bitcoin_punish_timelock: PunishTimelock::new(50),
            bitcoin_network: bitcoin::Network::Regtest,
            monero_avg_block_time: 1.seconds(),
            monero_finality_confirmations: 10,
            monero_network: monero::Network::Mainnet, // yes this is strange
        }
    }
}

fn sync_interval(avg_block_time: Duration) -> Duration {
    max(avg_block_time / 10, Duration::from_secs(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_interval_is_one_second_if_avg_blocktime_is_one_second() {
        let interval = sync_interval(Duration::from_secs(1));

        assert_eq!(interval, Duration::from_secs(1))
    }

    #[test]
    fn check_interval_is_tenth_of_avg_blocktime() {
        let interval = sync_interval(Duration::from_secs(100));

        assert_eq!(interval, Duration::from_secs(10))
    }
}
