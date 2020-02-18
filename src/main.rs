use monero::cryptonote::hash::Hash;
use monero::{Address, Network};
use monero::util::key::KeyPair;

extern crate hex;

fn main() {
    let tag = hex::decode("0badcafe").expect("invalid tag");
    let view = Hash::hash_to_scalar(&tag);
    let spend = Hash::hash_to_scalar(&tag);
    println!("{}", Address::from_keypair(Network::Mainnet, &KeyPair { view, spend }));
}
