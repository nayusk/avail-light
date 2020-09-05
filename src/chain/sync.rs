//! Syncing, short for synchronizing, consists in synchronizing the state of a local chain with
//! the state of the chain of other machines, called *remotes* or *sources*.
//!
//! > **Note**: While the above summary is a bit abstract, in practice it is almost always
//! >           done by exchanging messages through a peer-to-peer network.
//!
//! Multiple strategies exist for syncing, and which one to employ depends on the amount of
//! information that is desired (e.g. is it required to know the header and/or body of every
//! single block, or can some blocks be skipped?) and the distance between the highest block of
//! the local chain and the highest block on the rest of the network.
//!
//! # About safety
//!
//! While there exists various trade-offs between syncing strategies, safety is never part of
//! these trade-offs. All syncing strategies are safe, in the sense that malicious remotes cannot
//! corrupt the state of the local chain.
//!
//! It is possible, however, for a malicious remote to omit some information, such as the
//! existence of a specific block. If all the remotes that are being synchronized from are
//! malicious and collude to omit the same information, there is no way for the local node to
//! know about this information or even to be aware that it is missing an information. This is
//! called an **eclipse attack**.
//!
//! For this reason, it is important to ensure a large number and a good distribution of the
//! sources. In the context of a peer-to-peer network, a minimum threshold of around 7 peers is
//! generally considered acceptable. Similarly, in the context of a peer-to-peer network, it is
//! important to establish outgoing connections to other nodes and not only rely on incoming
//! connections, as there is otherwise the possibility of a single actor controlling all said
//! incoming connections.

pub mod headers_optimistic;

mod optimistic;
