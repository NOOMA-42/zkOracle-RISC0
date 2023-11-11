const express = require("express");
const fs = require("fs");
const rsa = require("node-rsa");
const fetch = require("node-fetch");
const contentType = require("content-type");

const app = express();
const port = 3000;

const key = new rsa(fs.readFileSync("./key", "utf8"));

app.get("/", (req, res) => {
  if (!req.query.source)
    return res.status(400).send({ error: "No source provided" });
  fetch(
    "https://min-api.cryptocompare.com/data/pricemultifull?fsyms=ETH&tsyms=USD"
  )
    .then(async (r) => await r.json())
    .then((data) => {
      const price = +(
        data.RAW.ETH.USD.PRICE *
        (1 + 0.01 * (0.5 - Math.random()))
      ).toFixed(2);
      const time = data.RAW.ETH.USD.LASTUPDATE;
      const sig = key.sign(JSON.stringify({ price, time }), "hex", "utf8");
      res.send({
        price,
        time,
        sig,
      });
    })
    .catch((err) => {
      console.error(err);
      res.status(400).send({ error: "Error fetching url" });
    });
});

app.listen(port, () => {
  console.log(`Using RSA key:`);
  console.log(key.exportKey("pkcs1-public"));
  console.log(`Server listening on port ${port}`);
});
