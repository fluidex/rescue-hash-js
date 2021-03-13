const rescueWasm = require("./pkg/rescue-wasm.js");
const input = new TextEncoder().encode("迟迟钟鼓初长夜，耿耿星河欲曙天。");
function rescueHashBigInt(input) {
  let resultBytes = rescueWasm.rescueHash(input);
  let resultBigInt =
    "0x" +
    Array.from(resultBytes)
      .map((item) => item.toString(16))
      .join("");
  return BigInt(resultBigInt);
}
// 15131965683816686492029126038145678019083347981596432597977339723207837174957
console.log(rescueHashBigInt(input));
