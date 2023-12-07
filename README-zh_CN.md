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

假设现在有一个 `index.less` 文件：

```less
h1 {
  color: red;

  .blue {
    color: blue;
  }
}
```

先使用 `less` 将其编译为 css：

```css
h1 {
  color: red;
}
h1 .blue {
  color: blue;
}
```

与此同时会得到一份 sourcemap：

```json
{
  "version": 3,
  "sources": [
    "/Users/jess/Code/Rust/mako-demo/src/index.less"
  ],
  "names": [],
  "mappings": "AAAA;EACE,UAAA;;AADF,EAGE;EACE,WAAA",
  "sourcesContent": [
    "h1 {\n  color: red;\n\n  .blue {\n    color: blue;\n  }\n}\n"
  ]
}

```

接着将 css 编译产物交给 swc 执行压缩，得到压缩后的产物以及另外一份 sourcemap：

```css
h1{color:red}h1 .blue{color:blue}
```

```json
{
  "version": 3,
  "sources": [
    "src/index.less"
  ],
  "sourcesContent": [
    "h1 {\n  color: red;\n}\nh1 .blue {\n  color: blue;\n}\n/*# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi9Vc2Vycy9qZXNzL0NvZGUvUnVzdC9tYWtvLWRlbW8vc3JjL2luZGV4Lmxlc3MiXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IkFBQUE7RUFDRSxVQUFBOztBQURGLEVBR0U7RUFDRSxXQUFBIn0= */"
  ],
  "names": [],
  "mappings": "AAAA,EAAE,AAAC,CAAC,AACF,KAAK,CAAE,GAAG,AACZ,CAAC,AACD,EAAE,CAAC,CAAC,IAAI,AAAC,CAAC,AACR,KAAK,CAAE,IAAI,AACb,CAAC"
}

```

那么如何将两份 sourcemap 合并起来呢？

### 合并 sourcemap

```rs
use merge_source_map::merge;
use sourcemap::SourceMap;

let less2css_sourcemap = r#"{
    "version": 3,
    "sources": [
      "input"
    ],
    "names": [],
    "mappings": "AAAA;EACE,UAAA;;AADF,EAGE;EACE,WAAA",
    "sourcesContent": [
      "h1 {\n  color: red;\n\n  .blue {\n    color: blue;\n  }\n}\n"
    ]
}"#;
let css_minify_sourcemap = r#"{
    "version": 3,
    "sources": [
      "src/index.less"
    ],
    "sourcesContent": [
      "h1 {\n  color: red;\n}\nh1 .blue {\n  color: blue;\n}\n/*# sourceMappingURL=data:application/json;base64,eyJ2ZXJzaW9uIjozLCJzb3VyY2VzIjpbIi9Vc2Vycy9qZXNzL0NvZGUvUnVzdC9tYWtvLWRlbW8vc3JjL2luZGV4Lmxlc3MiXSwibmFtZXMiOltdLCJtYXBwaW5ncyI6IkFBQUE7RUFDRSxVQUFBOztBQURGLEVBR0U7RUFDRSxXQUFBIn0= */"
    ],
    "names": [],
    "mappings": "AAAA,EAAE,AAAC,CAAC,AACF,KAAK,CAAE,GAAG,AACZ,CAAC,AACD,EAAE,CAAC,CAAC,IAAI,AAAC,CAAC,AACR,KAAK,CAAE,IAAI,AACb,CAAC"
}"#;

// merge sourcemaps
let merged_sourcemap = merge(vec![
    SourceMap::from_reader(less2css_sourcemap.as_bytes()).unwrap(),
    SourceMap::from_reader(css_minify_sourcemap.as_bytes()).unwrap(),
]);

let mut buf = vec![];
merged_sourcemap.to_writer(&mut buf).unwrap();
let merged_sourcemap = String::from_utf8(buf).unwrap();
```

合并后的 sourcemap：

```json
{
  "version": 3,
  "sources": [
    "input"
  ],
  "sourcesContent": [
    "h1 {\n  color: red;\n\n  .blue {\n    color: blue;\n  }\n}\n"
  ],
  "names": [],
  "mappings": "AAAA,EAAA,CAAA,AACE,KAAA,CAAA,GAAA,CAAA,AADF,EAGE,CAAA,CAAA,IAAA,CAAA,AACE,KAAA,CAAA,IAAA,CAAA"
}
```

你可以在[这里](https://evanw.github.io/source-map-visualization/#NzAAaDF7Y29sb3I6cmVkfWgxIC5ibHVle2NvbG9yOmJsdWV9Ci8qIyBzb3VyY2VNYXBwaW5nVVJMPWluZGV4LmNzcy5tYXAqLzI3MgB7CiAgInZlcnNpb24iOiAzLAogICJzb3VyY2VzIjogWwogICAgImlucHV0IgogIF0sCiAgInNvdXJjZXNDb250ZW50IjogWwogICAgImgxIHtcbiAgY29sb3I6IHJlZDtcblxuICAuYmx1ZSB7XG4gICAgY29sb3I6IGJsdWU7XG4gIH1cbn1cbiIKICBdLAogICJuYW1lcyI6IFtdLAogICJtYXBwaW5ncyI6ICJBQUFBLEVBQUEsQ0FBQSxBQUNFLEtBQUEsQ0FBQSxHQUFBLENBQUEsQUFERixFQUdFLENBQUEsQ0FBQSxJQUFBLENBQUEsQUFDRSxLQUFBLENBQUEsSUFBQSxDQUFBIgp9Cg==)查看可视化结果。

## License

MIT
