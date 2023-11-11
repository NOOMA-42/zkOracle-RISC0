const express = require("express");
const fs = require("fs");
const rsa = require("node-rsa");
const fetch = require("node-fetch");

const dataSource = {
  uniswap: require("./uniswap"),
  binance: require("./binance"),
};

const app = express();
const port = 3000;

const key = new rsa(fs.readFileSync("./key", "utf8"));

app.get("/", (req, res) => {
  if (!req.query.source)
    return res.status(400).send({ error: "No source provided" });
  const fn = dataSource[req.query.source];
  if (fn) return fn(req, res, key);
  return res.status(400).send({ error: "Not a valid source" });
});

app.listen(port, () => {
  console.log(`Using RSA key:`);
  console.log(key.exportKey("pkcs1-public"));
  console.log(`Server listening on port ${port}`);
});
