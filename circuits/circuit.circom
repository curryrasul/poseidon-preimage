pragma circom 2.0.0;

include "../node_modules/circomlib/circuits/poseidon.circom";

template PoseidonPreimageVerifier() {
    signal input preimage;
    signal input image;

    component hasher = Poseidon(1);

    hasher.inputs[0] <== preimage;

    image === hasher.out;
}

component main { public [image] } = PoseidonPreimageVerifier();
