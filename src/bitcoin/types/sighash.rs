use near_sdk::borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, BorshSerialize, BorshDeserialize)]
#[near_sdk::near(serializers=[json])]
#[borsh(crate = "near_sdk::borsh", use_discriminant = true)]
pub enum EcdsaSighashType {
    /// 0x1: Sign all outputs.
    All = 0x01,
}
