//! Merge multiple sourcemaps.
//!
//! # Installation
//!
//! ```bash
//! cargo add sourcemap merge-source-map
//! ```
//!
//! # Usage
//!
//! ```rust
//! use merge_source_map::merge;
//! use sourcemap::SourceMap;
//!
//! fn main() {
//!     let sourcemap1 = r#"{
//!         "version": 3,
//!         "file": "index.js",
//!         "sourceRoot": "",
//!         "sources": [
//!           "index.ts"
//!         ],
//!         "names": [],
//!         "mappings": "AAAA,SAAS,QAAQ,CAAC,IAAY;IAC5B,OAAO,CAAC,GAAG,CAAC,iBAAU,IAAI,CAAE,CAAC,CAAC;AAChC,CAAC",
//!         "sourcesContent": [
//!           "function sayHello(name: string) {\n  console.log(`Hello, ${name}`);\n}\n"
//!         ]
//!     }"#;
//!     let sourcemap2 = r#"{
//!         "version": 3,
//!         "file": "minify.js",
//!         "sourceRoot": "",
//!         "sources": [
//!           "index.js"
//!         ],
//!         "names": [
//!           "sayHello",
//!           "name",
//!           "console",
//!           "log",
//!           "concat"
//!         ],
//!         "mappings": "AAAA,SAASA,SAASC,CAAI,EAClBC,QAAQC,GAAG,CAAC,UAAUC,MAAM,CAACH,GACjC",
//!         "sourcesContent": [
//!           "function sayHello(name) {\n    console.log(\"Hello, \".concat(name));\n}\n"
//!         ]
//!     }"#;
//!
//!     // merge sourcemap
//!     let merged = merge(vec![
//!         SourceMap::from_reader(sourcemap1.as_bytes()).unwrap(),
//!         SourceMap::from_reader(sourcemap2.as_bytes()).unwrap(),
//!     ]);
//!
//!     let mut buf = vec![];
//!     merged.to_writer(&mut buf).unwrap();
//!     let merged = String::from_utf8(buf).unwrap();
//! }
//! ```
//! Merged sourcemap:
//! ```json
//! {
//!   "version": 3,
//!   "sources": [
//!     "index.ts"
//!   ],
//!   "sourcesContent": [
//!     "function sayHello(name: string) {\n  console.log(`Hello, ${name}`);\n}\n"
//!   ],
//!   "names": [],
//!   "mappings": "AAAA,SAAS,SAAS,CAAY,EAC5B,QAAQ,GAAG,CAAC,UAAA,MAAA,CAAU,GACxB"
//! }
//! ```
//!
//! You can view result [here](https://evanw.github.io/source-map-visualization/#NTQAZnVuY3Rpb24gc2F5SGVsbG8obyl7Y29uc29sZS5sb2coIkhlbGxvLCAiLmNvbmNhdChvKSl9MjU0AHsKICAidmVyc2lvbiI6IDMsCiAgInNvdXJjZXMiOiBbCiAgICAiaW5kZXgudHMiCiAgXSwKICAic291cmNlc0NvbnRlbnQiOiBbCiAgICAiZnVuY3Rpb24gc2F5SGVsbG8obmFtZTogc3RyaW5nKSB7XG4gIGNvbnNvbGUubG9nKGBIZWxsbywgJHtuYW1lfWApO1xufVxuIgogIF0sCiAgIm5hbWVzIjogW10sCiAgIm1hcHBpbmdzIjogIkFBQUEsU0FBUyxTQUFTLENBQVksRUFDNUIsUUFBUSxHQUFHLENBQUMsVUFBQSxNQUFBLENBQVUsR0FDeEIiCn0K).
use sourcemap::{SourceMap, SourceMapBuilder};

pub fn merge(mut maps: Vec<SourceMap>) -> SourceMap {
    let mut builder = SourceMapBuilder::new(None);

    maps = maps
        .into_iter()
        .filter(|map| map.get_token_count() > 0)
        .collect();
    if maps.is_empty() {
        return builder.into_sourcemap();
    }

    maps.reverse();

    let dest_map = &maps[0];

    for token in dest_map.tokens() {
        let mut last_map_token = token;
        let mut completed_trace = true;

        if maps.len() > 1 {
            for map in &maps[1..] {
                if let Some(map_token) = map.lookup_token(token.get_src_line(), token.get_src_col())
                {
                    last_map_token = map_token;
                } else {
                    completed_trace = false;
                    break;
                }
            }
        }

        if !completed_trace {
            continue;
        }

        // add mapping
        let added_token = builder.add(
            token.get_dst_line(),
            token.get_dst_col(),
            last_map_token.get_src_line(),
            last_map_token.get_src_col(),
            last_map_token.get_source(),
            last_map_token.get_name(),
        );

        // add source centent
        if !builder.has_source_contents(added_token.src_id) {
            let source_content = if let Some(view) = last_map_token.get_source_view() {
                Some(view.source())
            } else {
                None
            };

            builder.set_source_contents(added_token.src_id, source_content);
        }
    }

    builder.into_sourcemap()
}

#[cfg(test)]
mod test {
    use sourcemap::SourceMap;

    use crate::merge;

    #[test]
    fn test_merge() {
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

        let merged = merge(vec![
            SourceMap::from_reader(sourcemap1.as_bytes()).unwrap(),
            SourceMap::from_reader(sourcemap2.as_bytes()).unwrap(),
        ]);
        let mut buf = vec![];
        merged.to_writer(&mut buf).unwrap();
        let merged = String::from_utf8(buf).unwrap();

        assert!(merged.eq(r#"{"version":3,"sources":["index.ts"],"sourcesContent":["function sayHello(name: string) {\n  console.log(`Hello, ${name}`);\n}\n"],"names":[],"mappings":"AAAA,SAAS,SAAS,CAAY,EAC5B,QAAQ,GAAG,CAAC,UAAA,MAAA,CAAU,GACxB"}"#));
    }
}
