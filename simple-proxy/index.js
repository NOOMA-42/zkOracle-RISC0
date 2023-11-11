const express = require("express");
const fs = require("fs");
const rsa = require("node-rsa");
const fetch = require("node-fetch");
const contentType = require("content-type");

const app = express();
const port = 3000;

const key = new rsa(fs.readFileSync("./key", "utf8"));

app.get("/", (req, res) => {
  if (!req.query.url) return res.status(400).send({ error: "No url provided" });
  fetch(req.query.url)
    .then(async (r) => {
      if (
        r.headers.get("content-type") &&
        contentType.parse(r.headers.get("content-type")).type ===
          "application/json"
      ) {
        return await r.json();
      }
      return {
        data: await r.text(),
      };
    })
    .then((data) => {
      return {
        ...data,
        ["proxy.url"]: req.query.url,
      };
    })
    .then((data) => {
      const sig = key.sign(JSON.stringify(data), "hex", "utf8");
      res.send({
        ...data,
        ["proxy.signature"]: sig,
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
