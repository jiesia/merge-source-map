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

        let merged = merge(vec![
            SourceMap::from_reader(less2css_sourcemap.as_bytes()).unwrap(),
            SourceMap::from_reader(css_minify_sourcemap.as_bytes()).unwrap(),
        ]);
        let mut buf = vec![];
        merged.to_writer(&mut buf).unwrap();
        let merged = String::from_utf8(buf).unwrap();
        assert!(merged.eq(r#"{"version":3,"sources":["input"],"sourcesContent":["h1 {\n  color: red;\n\n  .blue {\n    color: blue;\n  }\n}\n"],"names":[],"mappings":"AAAA,EAAA,CAAA,AACE,KAAA,CAAA,GAAA,CAAA,AADF,EAGE,CAAA,CAAA,IAAA,CAAA,AACE,KAAA,CAAA,IAAA,CAAA"}"#));
    }
}
