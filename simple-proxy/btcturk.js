const fetch = require("node-fetch");
const { verifySignature } = require('./signature-test');

module.exports = (req, res, key) =>
  fetch("https://api.btcturk.com/api/v2/ticker?pairSymbol=ETHUSDT")
    .then(async (r) => await r.json())
    .then((data) => {
      // Extract the price from the data
      const price = parseFloat(parseFloat(data["data"][0]["last"]).toFixed(2));
      // const time = new Date().getTime();
      
      // ECDSA signing
      // const message = String(price);
      const encoder = new TextEncoder();
      const encoded_price = encoder.encode(price);
      const signature = key.sign(encoded_price).toDER('hex');
      
      res.send({ price, sig: signature });
    })
    .catch((err) => {
      console.error(err);
      res.status(400).send({ error: "Error fetching url" });
    });
