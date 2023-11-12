const { ChainId, Token, WETH9, CurrencyAmount } = require("@uniswap/sdk-core");
const { Route, Pair } = require("@uniswap/v2-sdk");
const { getContract, createPublicClient, http } = require("viem");
const { mainnet } = require("viem/chains");


const client = createPublicClient({
  chain: mainnet,
  transport: http(
    "https://mainnet.infura.io/v3/1b4fd85ec53748feae973ece5bc436bd"
  ),
});
module.exports = async (req, res, key) => {
  const USDT = new Token(
    ChainId.MAINNET,
    "0xdac17f958d2ee523a2206206994597c13d831ec7",
    6
  );

  const pairAddress = Pair.getAddress(USDT, WETH9[USDT.chainId]);

  // Setup provider, import necessary ABI ...
  const pairContract = getContract({
    address: pairAddress,
    abi: [
      {
        constant: true,
        inputs: [],
        name: "getReserves",
        outputs: [
          { internalType: "uint112", name: "_reserve0", type: "uint112" },
          { internalType: "uint112", name: "_reserve1", type: "uint112" },
          {
            internalType: "uint32",
            name: "_blockTimestampLast",
            type: "uint32",
          },
        ],
        payable: false,
        stateMutability: "view",
        type: "function",
      },
    ],
    publicClient: client,
  });
  const reserves = await pairContract.read.getReserves();
  const [reserve0, reserve1] = reserves;

  const tokens = [USDT, WETH9[USDT.chainId]];
  const [token0, token1] = tokens[0].sortsBefore(tokens[1])
    ? tokens
    : [tokens[1], tokens[0]];

  const pair = new Pair(
    CurrencyAmount.fromRawAmount(token0, reserve0.toString()),
    CurrencyAmount.fromRawAmount(token1, reserve1.toString())
  );

  const route = new Route([pair], WETH9[USDT.chainId], USDT);

  const price = parseFloat(route.midPrice.toFixed(2));
  // const time = new Date().getTime();
  
  // ECDSA signing
  //  const message = String(price);
   const signature = key.sign(price).toDER('hex');
  
   res.send({ price, sig: signature })
 };

