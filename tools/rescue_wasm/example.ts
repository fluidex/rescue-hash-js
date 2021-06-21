const rescueWasm = require("./pkg/rescue-wasm.js");

// ff_ce needs length be even https://github.com/matter-labs/ff/blob/2e40bce7452a2d4249397f0ce6efe16dae86a2b9/src/lib.rs#L603
function evenCharHex(n) {
  var hex = n.toString(16);
  if (hex.length % 2 == 1) {
    hex = "0" + hex;
  }
  return hex;
}

function rescueHashBigIntArray(inputs: Array<bigint>): bigint {
  let hexStrings = inputs.map(evenCharHex);
  return BigInt(rescueWasm.rescueHashHex(hexStrings));
}

function rescueHashBuffer(input: Uint8Array): bigint {
  let resultBytes: Uint8Array = rescueWasm.rescueHash(input);
  let resultBigInt = "0x" + Buffer.from(resultBytes).toString("hex");
  return BigInt(resultBigInt);
}

function benchmarkRescue() {
  for (let i = 0; i < 100; i++) {
    const start = Date.now();
    for (let j = 0; j < 1000; j++) {
      rescueHashBigIntArray([17n, 18n, 19n]);
    }
    const end = Date.now();
    // 2021.03.15(Apple M1): 1000 ops takes 4937ms
    // Rust: 990,152 ns/iter ~= 990ms. Wasm is about 5 times slower than Rust. Reasonable.
    console.log(`${1000} ops takes ${end - start}ms`);
  }
}

function main() {
  // 16571241020258333354093353159575087257074492169409232867884029018069038774606
  console.log(
    rescueHashBigIntArray([28829699159647608n, 7521419745152037748n])
  );

  // 15131965683816686492029126038145678019083347981596432597977339723207837174957

  const input = new TextEncoder().encode("迟迟钟鼓初长夜，耿耿星河欲曙天。");
  console.log(rescueHashBuffer(input));

  console.log("benchmark:");
  benchmarkRescue();
}

main()
