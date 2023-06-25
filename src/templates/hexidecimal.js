const toHexidecimal = (bytes) =>
  bytes.reduce((str, byte) => str + byte.toString(16), "");

const hexidecimal = (match) => `0x${match},`;

const fromHexidecimal = (str) =>
  str
    .replaceAll(/-/g, "")
    .replaceAll(/\w{2}/g, hexidecimal)
    .split(",")
    .reduce((a, c) => (c ? a.concat(parseInt(c)) : a), []);

console.log(
  toHexidecimal([
    161, 162, 163, 164, 177, 178, 193, 194, 209, 210, 211, 212, 213, 214, 215,
    216,
  ])
);
