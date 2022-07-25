const hash = require("circomlibjs").poseidon;

const v = hash([10]);

console.log(v);