const express = require("express");
const fs = require("fs");
const EC = require("elliptic").ec;
const fetch = require("node-fetch");

const dataSource = {
  uniswap: require("./uniswap"),
  btcturk: require("./btcturk"),
  // binance: require("./binance"),
};

const app = express();
const port = 3000;

const ec = new EC('secp256k1');
const key = ec.genKeyPair();

app.get("/", (req, res) => {
  if (!req.query.source)
    return res.status(400).send({ error: "No source provided" });
  const fn = dataSource[req.query.source];
  if (fn) return fn(req, res, key);
  return res.status(400).send({ error: "Not a valid source" });
});

app.listen(port, () => {
  console.log(`Using ECDSA key:`);
  console.log(key.getPublic().encode('hex'));
  console.log(`Server listening on port ${port}`);
});
