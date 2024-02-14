use colored::Colorize;

pub fn extract_json_block(source: impl AsRef<str>) -> Option<String> {
    let source = source.as_ref();
    let blocks = markdown::tokenize(source);
    for block in blocks {
        match block {
            markdown::Block::CodeBlock(Some(kind), content) if kind == "json" => {
                return Some(content)
            }
            _ => ()
        }
    }
    None
}