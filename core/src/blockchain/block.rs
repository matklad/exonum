use crypto::Hash;
use consensus::protocol::Precommit;
use consensus::ValidatorId;
use super::Height;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    /// Information schema version.
    schema_version: u16,
    /// Block proposer id.
    proposer_id: ValidatorId,
    /// Height of the committed block
    height: Height,
    /// Number of transactions in block.
    tx_count: u32,
    /// Hash link to the previous block in blockchain.
    prev_hash: Hash,
    /// Root hash of [merkle tree](struct.Schema.html#method.block_txs) of current block
    /// transactions.
    tx_hash: Hash,
    /// Hash of the current `exonum` state after applying transactions in the block.
    state_hash: Hash,
}

impl Block {
    pub fn new(
        schema_version: u16,
        proposer_id: ValidatorId,
        height: Height,
        tx_count: u32,
        prev_hash: Hash,
        tx_hash: Hash,
        state_hash: Hash,
    ) -> Block {
        Block {
            schema_version,
            proposer_id,
            height,
            tx_count,
            prev_hash,
            tx_hash,
            state_hash,
        }
    }

    pub fn hash(&self) -> Hash {
        unimplemented!()
    }
}

/// Block with pre-commits.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockProof {
    /// Block.
    pub block: Block,
    /// List of pre-commits for the block.
    pub precommits: Vec<Precommit>,
}
