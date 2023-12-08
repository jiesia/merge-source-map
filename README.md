# merge-source-map

English | [中文](./README-zh_CN.md)

Merge multiple sourcemaps.

## Install

```bash
cargo add sourcemap merge-source-map
```

## Usage

Here I will use a case to let you know how to use it.

### Requirement

Suppose you now have a file named `index.ts`:

```typescript
function sayHello(name: string) {
  console.log(`Hello, ${name}`);
}
```

First use `tsc`(with sourceMap and inlineSources options) to compile it to `index.js`:

```javascript
function sayHello(name) {
  console.log("Hello, ".concat(name));
}
```

At the same time, a file named `index.js.map` will be obtained:

```json
{
  "version": 3,
  "file": "index.js",
  "sourceRoot": "",
  "sources": [
    "index.ts"
  ],
  "names": [],
  "mappings": "AAAA,SAAS,QAAQ,CAAC,IAAY;IAC5B,OAAO,CAAC,GAAG,CAAC,iBAAU,IAAI,CAAE,CAAC,CAAC;AAChC,CAAC",
  "sourcesContent": [
    "function sayHello(name: string) {\n  console.log(`Hello, ${name}`);\n}\n"
  ]
}
```

Then hand the `index.js` compiled product to swc for compression, and get the compressed product and another file named `minify.js.map`:

```javascript
function sayHello(o){console.log("Hello, ".concat(o))}
```

```json
{
  "version": 3,
  "file": "minify.js",
  "sourceRoot": "",
  "sources": [
    "index.js"
  ],
  "names": [
    "sayHello",
    "name",
    "console",
    "log",
    "concat"
  ],
  "mappings": "AAAA,SAASA,SAASC,CAAI,EAClBC,QAAQC,GAAG,CAAC,UAAUC,MAAM,CAACH,GACjC",
  "sourcesContent": [
    "function sayHello(name) {\n    console.log(\"Hello, \".concat(name));\n}\n"
  ]
}
```

So how to merge two sourcemaps?

### Merge sourcemaps

```rust
use merge_source_map::merge;
use sourcemap::SourceMap;

fn main() {
    let sourcemap1 = r#"{
        "version": 3,
        "file": "index.js",
        "sourceRoot": "",
        "sources": [
          "index.ts"
        ],
        "names": [],
        "mappings": "AAAA,SAAS,QAAQ,CAAC,IAAY;IAC5B,OAAO,CAAC,GAAG,CAAC,iBAAU,IAAI,CAAE,CAAC,CAAC;AAChC,CAAC",
        "sourcesContent": [
          "function sayHello(name: string) {\n  console.log(`Hello, ${name}`);\n}\n"
        ]
    }"#;
    let sourcemap2 = r#"{
        "version": 3,
        "file": "minify.js",
        "sourceRoot": "",
        "sources": [
          "index.js"
        ],
        "names": [
          "sayHello",
          "name",
          "console",
          "log",
          "concat"
        ],
        "mappings": "AAAA,SAASA,SAASC,CAAI,EAClBC,QAAQC,GAAG,CAAC,UAAUC,MAAM,CAACH,GACjC",
        "sourcesContent": [
          "function sayHello(name) {\n    console.log(\"Hello, \".concat(name));\n}\n"
        ]
    }"#;

    // merge sourcemap
    let merged = merge(vec![
        SourceMap::from_reader(sourcemap1.as_bytes()).unwrap(),
        SourceMap::from_reader(sourcemap2.as_bytes()).unwrap(),
    ]);
    let mut buf = vec![];
    merged.to_writer(&mut buf).unwrap();
    let merged = String::from_utf8(buf).unwrap();
}
```

Merged sourcemap:

```json
{
  "version": 3,
  "sources": [
    "index.ts"
  ],
  "sourcesContent": [
    "function sayHello(name: string) {\n  console.log(`Hello, ${name}`);\n}\n"
  ],
  "names": [],
  "mappings": "AAAA,SAAS,SAAS,CAAY,EAC5B,QAAQ,GAAG,CAAC,UAAA,MAAA,CAAU,GACxB"
}
```

You can view result [here](https://evanw.github.io/source-map-visualization/#NTQAZnVuY3Rpb24gc2F5SGVsbG8obyl7Y29uc29sZS5sb2coIkhlbGxvLCAiLmNvbmNhdChvKSl9MjU0AHsKICAidmVyc2lvbiI6IDMsCiAgInNvdXJjZXMiOiBbCiAgICAiaW5kZXgudHMiCiAgXSwKICAic291cmNlc0NvbnRlbnQiOiBbCiAgICAiZnVuY3Rpb24gc2F5SGVsbG8obmFtZTogc3RyaW5nKSB7XG4gIGNvbnNvbGUubG9nKGBIZWxsbywgJHtuYW1lfWApO1xufVxuIgogIF0sCiAgIm5hbWVzIjogW10sCiAgIm1hcHBpbmdzIjogIkFBQUEsU0FBUyxTQUFTLENBQVksRUFDNUIsUUFBUSxHQUFHLENBQUMsVUFBQSxNQUFBLENBQVUsR0FDeEIiCn0K).

## License

MIT
