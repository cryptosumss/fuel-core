//! The module for all possible coins.

use crate::{
    entities::Nonce,
    fuel_asm::Word,
    fuel_tx::Address,
    fuel_types::AssetId,
};
use coin::Coin;
use deposit_coin::DepositCoin;
use fuel_vm_private::prelude::UtxoId;

pub mod coin;
pub mod deposit_coin;

/// Whether a coin has been spent or not
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Default, Debug, Copy, Clone, Eq, PartialOrd, PartialEq)]
#[repr(u8)]
pub enum CoinStatus {
    /// Coin has not been spent
    Unspent,
    #[default]
    /// Coin has been spent
    Spent,
}

/// The unique identifier of the coin.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, Eq, PartialOrd, PartialEq, Ord, Hash)]
pub enum CoinId {
    /// The UTXO id of the regular coin.
    Utxo(UtxoId),
    /// The unique `nonce` of the `DepositCoin`.
    Message(Nonce),
}

impl From<UtxoId> for CoinId {
    fn from(id: UtxoId) -> Self {
        CoinId::Utxo(id)
    }
}

impl From<Nonce> for CoinId {
    fn from(id: Nonce) -> Self {
        CoinId::Message(id)
    }
}

/// The enum of all kind of coins.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Coins {
    /// The regular coins generated by the transaction output.
    Coin(Coin),
    /// The bridged coin from the DA layer.
    DepositCoin(DepositCoin),
}

impl Coins {
    /// Returns the coin unique identifier.
    pub fn coin_id(&self) -> CoinId {
        match self {
            Coins::Coin(coin) => CoinId::Utxo(coin.utxo_id),
            Coins::DepositCoin(coin) => CoinId::Message(coin.nonce),
        }
    }

    /// Returns the owner of the coin.
    pub fn owner(&self) -> &Address {
        match self {
            Coins::Coin(coin) => &coin.owner,
            Coins::DepositCoin(coin) => &coin.recipient,
        }
    }

    /// Returns the amount of the asset held by the coin.
    pub fn amount(&self) -> Word {
        match self {
            Coins::Coin(coin) => coin.amount,
            Coins::DepositCoin(coin) => coin.amount,
        }
    }

    /// Returns the asset held by the coin.
    pub fn asset_id(&self) -> &AssetId {
        match self {
            Coins::Coin(coin) => &coin.asset_id,
            Coins::DepositCoin(_) => &AssetId::BASE,
        }
    }

    /// Returns the status of the coin.
    pub fn status(&self) -> CoinStatus {
        match self {
            Coins::Coin(coin) => coin.status,
            Coins::DepositCoin(coin) => coin.status,
        }
    }
}

impl From<Coin> for Coins {
    fn from(coin: Coin) -> Self {
        Coins::Coin(coin)
    }
}

impl From<DepositCoin> for Coins {
    fn from(coin: DepositCoin) -> Self {
        Coins::DepositCoin(coin)
    }
}
