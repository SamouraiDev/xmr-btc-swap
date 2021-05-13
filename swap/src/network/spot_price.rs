use crate::monero;
use crate::network::cbor_request_response::CborCodec;
use libp2p::core::ProtocolName;
use libp2p::request_response::{RequestResponse, RequestResponseEvent, RequestResponseMessage};
use serde::{Deserialize, Serialize};

pub const PROTOCOL: &str = "/comit/xmr/btc/spot-price/1.0.0";
pub type OutEvent = RequestResponseEvent<Request, Response>;
pub type Message = RequestResponseMessage<Request, Response>;

pub type Behaviour = RequestResponse<CborCodec<SpotPriceProtocol, Request, Response>>;

/// The spot price protocol allows parties to **initiate** a trade by requesting
/// a spot price.
///
/// A spot price is binding for both parties, i.e. after the spot-price protocol
/// completes, both parties are expected to follow up with the `execution-setup`
/// protocol.
///
/// If a party wishes to only inquire about the current price, they should use
/// the `quote` protocol instead.
#[derive(Debug, Clone, Copy, Default)]
pub struct SpotPriceProtocol;

impl ProtocolName for SpotPriceProtocol {
    fn protocol_name(&self) -> &[u8] {
        PROTOCOL.as_bytes()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    #[serde(with = "::bitcoin::util::amount::serde::as_sat")]
    pub btc: bitcoin::Amount,
    pub blockchain_network: BlockchainNetwork,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Response {
    Xmr(monero::Amount),
    Error(Error),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Error {
    NoSwapsAccepted,
    AmountBelowMinimum {
        #[serde(with = "::bitcoin::util::amount::serde::as_sat")]
        min: bitcoin::Amount,
        #[serde(with = "::bitcoin::util::amount::serde::as_sat")]
        buy: bitcoin::Amount,
    },
    AmountAboveMaximum {
        #[serde(with = "::bitcoin::util::amount::serde::as_sat")]
        max: bitcoin::Amount,
        #[serde(with = "::bitcoin::util::amount::serde::as_sat")]
        buy: bitcoin::Amount,
    },
    BalanceTooLow {
        #[serde(with = "::bitcoin::util::amount::serde::as_sat")]
        buy: bitcoin::Amount,
    },
    BlockchainNetworkMismatch {
        cli: BlockchainNetwork,
        asb: BlockchainNetwork,
    },
    /// To be used for errors that cannot be explained on the CLI side (e.g.
    /// rate update problems on the seller side)
    Other,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct BlockchainNetwork {
    pub bitcoin: BitcoinNetwork,
    pub monero: MoneroNetwork,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum BitcoinNetwork {
    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum MoneroNetwork {
    Mainnet,
    Stagenet,
    Testnet,
}

impl From<bitcoin::Network> for BitcoinNetwork {
    fn from(network: bitcoin::Network) -> Self {
        match network {
            bitcoin::Network::Bitcoin => BitcoinNetwork::Mainnet,
            bitcoin::Network::Testnet => BitcoinNetwork::Testnet,
            bitcoin::Network::Signet => BitcoinNetwork::Signet,
            bitcoin::Network::Regtest => BitcoinNetwork::Regtest,
        }
    }
}

impl From<::monero::Network> for MoneroNetwork {
    fn from(network: monero::Network) -> Self {
        match network {
            ::monero::Network::Mainnet => MoneroNetwork::Mainnet,
            ::monero::Network::Stagenet => MoneroNetwork::Stagenet,
            ::monero::Network::Testnet => MoneroNetwork::Testnet,
        }
    }
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
#[error("NetworkNotSupported")]
pub struct NetworkNotSupported;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monero;

    #[test]
    fn snapshot_test_serialize() {
        let amount = monero::Amount::from_piconero(100_000u64);
        let xmr = r#"{"Xmr":100000}"#.to_string();
        let serialized = serde_json::to_string(&Response::Xmr(amount)).unwrap();
        assert_eq!(xmr, serialized);

        let error = r#"{"Error":"NoSwapsAccepted"}"#.to_string();
        let serialized = serde_json::to_string(&Response::Error(Error::NoSwapsAccepted)).unwrap();
        assert_eq!(error, serialized);

        let error = r#"{"Error":{"AmountBelowMinimum":{"min":0,"buy":0}}}"#.to_string();
        let serialized = serde_json::to_string(&Response::Error(Error::AmountBelowMinimum {
            min: Default::default(),
            buy: Default::default(),
        }))
        .unwrap();
        assert_eq!(error, serialized);

        let error = r#"{"Error":{"AmountAboveMaximum":{"max":0,"buy":0}}}"#.to_string();
        let serialized = serde_json::to_string(&Response::Error(Error::AmountAboveMaximum {
            max: Default::default(),
            buy: Default::default(),
        }))
        .unwrap();
        assert_eq!(error, serialized);

        let error = r#"{"Error":{"BalanceTooLow":{"buy":0}}}"#.to_string();
        let serialized = serde_json::to_string(&Response::Error(Error::BalanceTooLow {
            buy: Default::default(),
        }))
        .unwrap();
        assert_eq!(error, serialized);

        let error = r#"{"Error":{"BlockchainNetworkMismatch":{"cli":{"bitcoin":"Mainnet","monero":"Mainnet"},"asb":{"bitcoin":"Testnet","monero":"Stagenet"}}}}"#.to_string();
        let serialized =
            serde_json::to_string(&Response::Error(Error::BlockchainNetworkMismatch {
                cli: BlockchainNetwork {
                    bitcoin: BitcoinNetwork::Mainnet,
                    monero: MoneroNetwork::Mainnet,
                },
                asb: BlockchainNetwork {
                    bitcoin: BitcoinNetwork::Testnet,
                    monero: MoneroNetwork::Stagenet,
                },
            }))
            .unwrap();
        assert_eq!(error, serialized);

        let error = r#"{"Error":"Other"}"#.to_string();
        let serialized = serde_json::to_string(&Response::Error(Error::Other)).unwrap();
        assert_eq!(error, serialized);
    }
}
