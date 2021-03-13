const rescueWasm = require("./pkg/rescue-wasm.js");
const input = new TextEncoder().encode("迟迟钟鼓初长夜，耿耿星河欲曙天。");
function evenCharHex(n) {
  var hex = n.toString(16);
  if (hex.length % 2 == 1) {
    hex = "0" + hex;
  }
  return hex;
}
function rescueHashHex(inputs) {
  let hexStrings = inputs.map(evenCharHex);
  return BigInt(rescueWasm.rescueHashHex(hexStrings));
}
// 16571241020258333354093353159575087257074492169409232867884029018069038774606
console.log(rescueHashHex([28829699159647608n, 7521419745152037748n]));
function rescueHashBigInt(input) {
  let resultBytes = rescueWasm.rescueHash(input);
  let resultBigInt = "0x" + Array.from(resultBytes).map(evenCharHex).join("");
  return BigInt(resultBigInt);
}
// 15131965683816686492029126038145678019083347981596432597977339723207837174957
console.log(rescueHashBigInt(input));
