use monero::cryptonote::hash::Hash;
use monero::{Address, Network};
use monero::util::key::PublicKey;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::edwards::CompressedEdwardsY;
use sha2::Sha512;

pub fn gen_burn_addr(tag: &[u8]) -> monero::Address {
    let view = PublicKey::from_private_key(&Hash::hash_to_scalar(&tag));
    let spend_ristretto = RistrettoPoint::hash_from_bytes::<Sha512>(&tag).compress();
    let spend = CompressedEdwardsY::from_slice(spend_ristretto.as_bytes());
    Address::standard(Network::Mainnet, PublicKey { point: spend }, view)
}