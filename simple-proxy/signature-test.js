// signature-test.js
const EC = require('elliptic').ec;
const ec = new EC('secp256k1');

function verifySignature(publicKeyHex, signatureHex, message) {
    const key = ec.keyFromPublic(publicKeyHex, 'hex');
    const messageHash = ec.hash().update(message).digest();
    return key.verify(messageHash, signatureHex);
}

module.exports = {
    verifySignature,
};
