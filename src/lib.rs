use monero::cryptonote::hash::Hash;
use monero::{Address, Network};
use monero::util::key::PublicKey;
use num_bigint::BigUint;

const SPEND_TAG: &str = "Whereof one cannot speak, thereof one must be silent";

fn hash_to_pk(x: &[u8]) -> PublicKey {
    let hash = Hash::hash(&x);
    let y = BigUint::from_bytes_le(&hash.as_bytes());
    (0u8..)
        .filter_map(|inc| PublicKey::from_slice(&(&y + inc).to_bytes_le()).ok())
        .nth(0)
        .unwrap()
}

pub fn gen_burn_addr(network: Network, tag: &[u8]) -> monero::Address {
    let view = PublicKey::from_private_key(&Hash::hash_to_scalar(&tag));
    // TODO: Avoid this hash_to_pk call by having the result be a constant.
    Address::standard(network, hash_to_pk(SPEND_TAG.as_bytes()), view)
}

pub fn burn_verify(tag: &[u8], addr: monero::Address) -> bool {
    gen_burn_addr(addr.network, tag) == addr
}

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

#[cfg(test)]
mod tests {
    use super::*;
    quickcheck! {
        fn valid_spend(tag: Vec<u8>) -> bool {
            gen_burn_addr(Network::Mainnet, &tag).public_spend.point.decompress().is_some()
        }
        fn correctness(tag: Vec<u8>) -> bool {
            burn_verify(&tag, gen_burn_addr(Network::Mainnet, &tag))
        }
    }
}