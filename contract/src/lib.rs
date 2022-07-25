use bolt_rs::verifier::near::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    pvk: PreparedVerifyingKey,
    preimage: String,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(vk: String, preimage: String) -> Self {
        let vk = parse_verification_key(vk).unwrap();
        let pvk = get_prepared_verifying_key(vk);

        Self { pvk, preimage }
    }

    pub fn verify(&self, proof: String) -> bool {
        verify_proof(self.pvk.clone(), proof, self.preimage.clone()).unwrap()
    }
}
