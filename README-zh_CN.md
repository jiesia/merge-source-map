# merge-source-map

[English](./README.md) | 中文

合并多个 sourcemap。

## 安装

```bash
cargo add sourcemap merge-source-map
```

## 如何使用

这里将通过一个案例让你了解如何使用 merge-source-map。

### 案例描述

假设现在有一个 `index.ts` 文件：

```typescript
function sayHello(name: string) {
  console.log(`Hello, ${name}`);
}
```

先使用 `tsc`（开启 sourceMap 和 inlineSources 选项）将其编译为 `index.js` 文件：

```javascript
function sayHello(name) {
  console.log("Hello, ".concat(name));
}
```

与此同时会得到一份 `index.js.map` 文件：

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

接着将 `index.js` 交给 swc 进行压缩，得到压缩后的产物 `minify.js` 以及另外一份 `minify.js.map` 文件：

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

那么如何将两份 sourcemap 合并起来呢？

### 合并 sourcemap

```rs
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
    let merged = merge(
        vec![
            SourceMap::from_reader(sourcemap1.as_bytes()).unwrap(),
            SourceMap::from_reader(sourcemap2.as_bytes()).unwrap(),
        ],
        Default::default(),
    );
    let mut buf = vec![];
    merged.to_writer(&mut buf).unwrap();
    let merged = String::from_utf8(buf).unwrap();
}
```

合并后的 sourcemap：

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

你可以在[这里](https://evanw.github.io/source-map-visualization/#NTQAZnVuY3Rpb24gc2F5SGVsbG8obyl7Y29uc29sZS5sb2coIkhlbGxvLCAiLmNvbmNhdChvKSl9MjU0AHsKICAidmVyc2lvbiI6IDMsCiAgInNvdXJjZXMiOiBbCiAgICAiaW5kZXgudHMiCiAgXSwKICAic291cmNlc0NvbnRlbnQiOiBbCiAgICAiZnVuY3Rpb24gc2F5SGVsbG8obmFtZTogc3RyaW5nKSB7XG4gIGNvbnNvbGUubG9nKGBIZWxsbywgJHtuYW1lfWApO1xufVxuIgogIF0sCiAgIm5hbWVzIjogW10sCiAgIm1hcHBpbmdzIjogIkFBQUEsU0FBUyxTQUFTLENBQVksRUFDNUIsUUFBUSxHQUFHLENBQUMsVUFBQSxNQUFBLENBQVUsR0FDeEIiCn0K)查看可视化结果。

## License

MIT
