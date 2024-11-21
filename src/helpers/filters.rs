pub fn pathify<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let s = s.to_string();
    Ok(s.replace(" ", "_").replace('/', "%2F"))
}
