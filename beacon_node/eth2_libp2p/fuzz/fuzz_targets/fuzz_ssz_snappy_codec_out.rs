#![no_main]
use libfuzzer_sys::fuzz_target;

use eth2_libp2p::rpc::{codec::SSZSnappyOutboundCodec, protocol::{Protocol, Version, Encoding, ProtocolId}};
use libp2p::bytes::BytesMut;
use tokio_util::codec::Decoder;
use types::{EthSpec, MainnetEthSpec, ForkContext, Hash256};
use std::sync::Arc;

// From beacon_node/eth2-libp2p/src/rpc/protocol.rs
const MAX_RPC_SIZE: usize = 1_048_576; // 1M

fuzz_target!(|wrap: (Protocol, Vec<u8>)| {
    let (status, data) = wrap;

    let protocol = ProtocolId::new(
        status,
        Version::V1,
        Encoding::SSZSnappy,
    );

    let fork_context = Arc::new(ForkContext::new(Hash256::zero(), &MainnetEthSpec::default_spec()));

    let mut codec = SSZSnappyOutboundCodec::<MainnetEthSpec>::new(protocol , MAX_RPC_SIZE, fork_context);

    let mut buffer = BytesMut::from(data.as_slice());

    let _ = codec.decode(&mut buffer);
});
