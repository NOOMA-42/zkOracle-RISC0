const fetch = require("node-fetch");

module.exports = (req, res, key) =>
  fetch("https://api3.binance.com/api/v3/ticker/price?symbol=ETHUSDT")
    .then(async (r) => await r.json())
    .then((data) => {
      const price = parseFloat(parseFloat(data["price"]).toFixed(2));
      const time = new Date().getTime();
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
