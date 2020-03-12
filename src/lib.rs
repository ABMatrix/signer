#![allow(dead_code)]


mod keypair;
mod transaction;
mod hasher;
mod types;
pub mod eth;
pub mod abos;

pub use keypair::{KeyPair, Keyring, PrivKey, PubKey};
pub use eth::{Eth, EthTransaction};
pub use abos::{Abos, AbosTransaction};
pub use types::*;

use secp256k1::{SecretKey, Message, sign};
pub(crate) fn sign_compose(message: &Message, secret_key: &SecretKey) -> [u8; 65] {
    let (signture, rec_id) = sign(message, &secret_key);
    let mut sgn = [0u8; 65];
    sgn[0..64].copy_from_slice(&signture.serialize());
    sgn[64] = rec_id.into();
    sgn
}