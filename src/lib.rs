use monero::cryptonote::hash::Hash;
use monero::{Address, Network};
use monero::util::key::PublicKey;
use num_bigint::BigUint;

const SPEND_TAG: &str = "Whereof one cannot speak, thereof one must be silent";

fn pad_u256(num: &[u8]) -> Vec<u8> {
    assert!(num.len() <= 32, "input needs to be a 256-bit bytearray");
    let mut v: Vec<u8> = num.to_vec();
    v.resize(32, 0);
    v
}

fn hash_to_pk(x: &[u8]) -> Option<PublicKey> {
    let hash = Hash::hash(&x);
    let y = BigUint::from_bytes_le(&hash.as_bytes());
    (0u8..=u8::max_value())
        .filter_map(|inc| {
            let encoding = pad_u256(&(&y + inc).to_bytes_le());
            PublicKey::from_slice(&encoding).ok()
        })
        .nth(0)
}

pub fn gen_burn_addr(network: Network, tag: &[u8]) -> monero::Address {
    let view = PublicKey::from_private_key(&Hash::hash_to_scalar(&tag));
    // TODO: Avoid this hash_to_pk call by having the result be a constant.
    Address::standard(network, hash_to_pk(SPEND_TAG.as_bytes()).unwrap(), view)
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
        fn hash_to_pk_terminates(inp: Vec<u8>) -> bool {
            hash_to_pk(&inp).is_some()
        }

        fn valid_spend(tag: Vec<u8>) -> bool {
            gen_burn_addr(Network::Mainnet, &tag).public_spend.point.decompress().is_some()
        }

        fn correctness(tag: Vec<u8>) -> bool {
            burn_verify(&tag, gen_burn_addr(Network::Mainnet, &tag))
        }
    }
}