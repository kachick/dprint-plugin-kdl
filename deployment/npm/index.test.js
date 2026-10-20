// @ts-check
const assert = require("assert");
const createFromBuffer = require("@dprint/formatter").createFromBuffer;
const getBuffer = require("./index").getBuffer;

const formatter = createFromBuffer(getBuffer());
const result = formatter.formatText({
  filePath: "file.kdl",
  fileText: "node   key=\"value\"",
});

assert.strictEqual(result, "node key=value\n");
