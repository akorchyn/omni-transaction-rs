use near_sdk::ext_contract;
use near_sdk::NearToken;

#[derive(Debug, Clone)]
#[near_sdk::near(serializers=[borsh, json])]
pub struct SignatureResponse {
    pub big_r: SerializableAffinePoint,
    pub s: SerializableScalar,
    pub recovery_id: u8,
}

#[derive(Debug, Clone)]
#[near_sdk::near(serializers=[borsh, json])]
pub struct SerializableAffinePoint {
    pub affine_point: String,
}

#[derive(Debug, Clone)]
#[near_sdk::near(serializers=[borsh, json])]
pub struct SerializableScalar {
    pub scalar: String,
}

#[derive(Debug, near_sdk::serde::Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct SignRequest {
    pub payload: [u8; 32],
    pub path: String,
    pub key_version: u32,
}

#[allow(dead_code)]
#[ext_contract(mpc_contract)]
pub trait MPCContract {
    fn sign(&self, request: SignRequest);
    fn experimental_signature_deposit(&self) -> NearToken;
}
