pub(crate) fn merge_classes(base: &str, custom: &str) -> String {
    let custom = custom.trim();

    if custom.is_empty() {
        base.to_string()
    } else {
        format!("{base} {custom}")
    }
}
