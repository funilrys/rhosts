use regex::Regex;


// A place for utility functions
pub fn sep(n: usize) {
    println!("{}", "-".repeat(n));
}

#[allow(dead_code)]
pub fn print_type_of<T>(_: &T) {
    println!("===> {}", std::any::type_name::<T>())
}

pub fn vtrim(v: &mut Vec<String>) -> &mut Vec<String> {
    v.iter_mut()
    .for_each(
        |line| {
            *line = one_space(line.trim())
        }
    );
    v
}

fn one_space(s: &str) -> String {
    let re_tabs = Regex::new(r"\t+").unwrap();
    let re_space = Regex::new(r"\s+").unwrap();
    re_space.replace_all(&re_tabs.replace_all(s, " "), " ").to_string()
}

#[test]
fn test_vtrim() {
    // assert_eq!(2 + 2, 4);
    let mut v = vec![];
    v.push("      Line 1".to_string());
    v.push("   Line 2   ".to_string());
    v.push("Line 3   ".to_string());
    // embedded spaces and tabs
    v.push("  127.0.0.1	10iski.com   ".to_string());

    let foo = vtrim(&mut v);
    assert_eq!(foo[0], "Line 1".to_string());
    assert_eq!(foo[1], "Line 2".to_string());
    assert_eq!(foo[2], "Line 3".to_string());
    assert_eq!(foo[3], "127.0.0.1 10iski.com".to_string());
}

pub fn stripblanklines(v: &mut Vec<String>) -> &mut Vec<String> {
    let mut trimmed = vtrim(v);
    trimmed.retain(|line | line.chars().count() > 0);
    v
}

#[test]
fn test_stripblanklines() {
    // assert_eq!(2 + 2, 4);
    let mut v = vec![];
    v.push("      Line 1".to_string());
    v.push("   Line 2   ".to_string());
    v.push("     ".to_string());
    v.push("Line 3   ".to_string());
    let foo = stripblanklines(&mut v);
    assert_eq!(foo[0], "Line 1".to_string());
    assert_eq!(foo[1], "Line 2".to_string());
    assert_eq!(foo[2], "Line 3".to_string());
}
