use std::str::FromStr;
use ldk_node::{Builder, Config, NetAddress};
use ldk_node::bitcoin::Network;
use ldk_node::bitcoin::secp256k1::PublicKey;

fn main() {
    let mut builder = Builder::new();

    builder.set_network(Network::Testnet);
    builder.set_esplora_server("https://blockstream.info/testnet/api".to_string());
    builder.set_gossip_source_rgs("https://rapidsync.lightningdevkit.org/testnet/snapshot".to_string());

    let node = builder.build().unwrap();
    node.start().unwrap();

    println!("NODE ID: {}", node.node_id());
    let funding_address = node.new_onchain_address().unwrap();
    println!("FUNDING ADDRESS {}", funding_address);

    let funds = node.total_onchain_balance_sats().unwrap();
    println!("FUNDS {}", funds);

    let spendable = node.spendable_onchain_balance_sats().unwrap();
    println!("Spendable {}", spendable);

    node.stop().unwrap();
}