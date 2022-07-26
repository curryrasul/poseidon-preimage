use electron_rs::verifier::near::*;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, PanicOnDefault};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    pvk: PreparedVerifyingKey,
    image: String,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(vk: String, image: String) -> Self {
        let vk = parse_verification_key(vk).unwrap();
        let pvk = get_prepared_verifying_key(vk);

        Self { pvk, image }
    }

    pub fn verify(&self, proof: String) -> bool {
        verify_proof(self.pvk.clone(), proof, self.image.clone()).unwrap()
    }
}
