#[test]
fn test_vulnerability_logic() {
    let rules = vec!["(ALL) NOPASSWD: /usr/bin/find"];
    let vulnerable = rules.iter().any(|&r| r.contains("/usr/bin/find") && r.contains("NOPASSWD"));
    assert!(vulnerable);
}
