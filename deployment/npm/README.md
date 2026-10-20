# @dprint/kdl

npm distribution of [dprint-plugin-kdl](https://github.com/dprint/dprint-plugin-kdl) which is an adapter plugin for [kdl-rs](https://github.com/kdl-org/kdl-rs).

Use this with [@dprint/formatter](https://github.com/dprint/js-formatter) or just use @dprint/formatter and download the [dprint-plugin-kdl Wasm file](https://github.com/dprint/dprint-plugin-kdl/releases).

## Example

```ts
import { createFromBuffer } from "@dprint/formatter";
import { getBuffer } from "@dprint/kdl";

const formatter = createFromBuffer(getBuffer());

console.log(formatter.formatText("file.kdl", "node   key=\"value\""));
```
