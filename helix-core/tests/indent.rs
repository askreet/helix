use helix_core::{
    indent::{treesitter_indent_for_pos, IndentStyle},
    syntax::Loader,
    Syntax,
};
use std::path::PathBuf;

#[test]
fn test_treesitter_indent_rust() {
    test_treesitter_indent("rust.rs", "source.rust");
}
#[test]
fn test_treesitter_indent_rust_2() {
    test_treesitter_indent("indent.rs", "source.rust");
    // TODO Use commands.rs as indentation test.
    // Currently this fails because we can't align the parameters of a closure yet
    // test_treesitter_indent("commands.rs", "source.rust");
}
#[test]
fn test_treesitter_indent_go() {
    test_treesitter_indent("indent.go", "source.go");
}

fn test_treesitter_indent(file_name: &str, lang_scope: &str) {
    let mut test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    test_dir.push("tests/data/indent");

    let mut test_file = test_dir.clone();
    test_file.push(file_name);
    let test_file = std::fs::File::open(test_file).unwrap();
    let doc = ropey::Rope::from_reader(test_file).unwrap();

    let mut config_file = test_dir;
    config_file.push("languages.toml");
    let config = std::fs::read(config_file).unwrap();
    let config = toml::from_slice(&config).unwrap();
    let loader = Loader::new(config);

    // set runtime path so we can find the queries
    let mut runtime = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    runtime.push("../runtime");
    std::env::set_var("HELIX_RUNTIME", runtime.to_str().unwrap());

    let language_config = loader
        .language_config_for_scope(lang_scope)
        .expect(&format!("unknown language scope {}", lang_scope));
    let highlight_config = language_config.highlight_config(&[]).unwrap();
    let syntax = Syntax::new(&doc, highlight_config, std::sync::Arc::new(loader));
    let indent_query = language_config.indent_query().unwrap();
    let text = doc.slice(..);

    let tab_width = language_config.indent.as_ref().unwrap().tab_width;
    let indent_style = match language_config.indent.as_ref().unwrap().unit.as_str() {
        "\t" => IndentStyle::Tabs,
        _ => IndentStyle::Spaces(tab_width as u8),
    };

    for line_idx in 0..doc.len_lines() {
        let line = text.line(line_idx);
        println!("analyzing line {}: '{}'", line_idx + 1, line);
        if let Some(pos) = helix_core::find_first_non_whitespace_char(line) {
            let suggested_indent = treesitter_indent_for_pos(
                indent_query,
                &syntax,
                &indent_style,
                tab_width,
                text,
                line_idx,
                text.line_to_char(line_idx) + pos,
                false,
            )
            .unwrap();
            assert!(
                line.get_slice(..pos).map_or(false, |s| s == suggested_indent),
                "Wrong indentation on line {}:\n\"{}\" (original line)\n\"{}\" (suggested indentation)\n",
                line_idx+1,
                line.slice(..line.len_chars()-1),
                suggested_indent,
            );
        }
    }
}
