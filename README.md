# merge-source-map

English | [中文](./README-zh_CN.md)

Merge multiple sourcemaps。

## Install

```bash
cargo add sourcemap merge-source-map
```

## Usage

Here I will use a case to let you know how to use it.

### Requirement

Suppose you now have an `index.less` file:

```less
h1 {
  color: red;

  .blue {
    color: blue;
  }
}
```

First use `less` to compile it to css:

```css
h1 {
  color: red;
}
h1 .blue {
  color: blue;
}
```

At the same time, a sourcemap will be obtained:

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

Then hand the css compiled product to swc for compression, and get the compressed product and another sourcemap:

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

So how to merge two sourcemaps?

### Merge sourcemaps

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

Merged sourcemap:

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

You can view result [here](https://evanw.github.io/source-map-visualization/#NzAAaDF7Y29sb3I6cmVkfWgxIC5ibHVle2NvbG9yOmJsdWV9Ci8qIyBzb3VyY2VNYXBwaW5nVVJMPWluZGV4LmNzcy5tYXAqLzI3MgB7CiAgInZlcnNpb24iOiAzLAogICJzb3VyY2VzIjogWwogICAgImlucHV0IgogIF0sCiAgInNvdXJjZXNDb250ZW50IjogWwogICAgImgxIHtcbiAgY29sb3I6IHJlZDtcblxuICAuYmx1ZSB7XG4gICAgY29sb3I6IGJsdWU7XG4gIH1cbn1cbiIKICBdLAogICJuYW1lcyI6IFtdLAogICJtYXBwaW5ncyI6ICJBQUFBLEVBQUEsQ0FBQSxBQUNFLEtBQUEsQ0FBQSxHQUFBLENBQUEsQUFERixFQUdFLENBQUEsQ0FBQSxJQUFBLENBQUEsQUFDRSxLQUFBLENBQUEsSUFBQSxDQUFBIgp9Cg==)。

## License

MIT
