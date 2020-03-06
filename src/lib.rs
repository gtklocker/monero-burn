use monero::cryptonote::hash::Hash;
use monero::{Address, Network};
use monero::util::key::PublicKey;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::edwards::CompressedEdwardsY;
use sha2::Sha512;

pub fn gen_burn_addr(network: Network, tag: &[u8]) -> monero::Address {
    let view = PublicKey::from_private_key(&Hash::hash_to_scalar(&tag));
    let spend_ristretto = RistrettoPoint::hash_from_bytes::<Sha512>(&tag).compress();
    let spend = CompressedEdwardsY::from_slice(spend_ristretto.as_bytes());
    Address::standard(network, PublicKey { point: spend }, view)
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
        fn correctness(tag: Vec<u8>) -> bool {
            burn_verify(&tag, gen_burn_addr(Network::Mainnet, &tag))
        }
    }
}