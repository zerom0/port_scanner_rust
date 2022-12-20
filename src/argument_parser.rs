pub fn parse_port_range(port_spec: &str) -> Vec<u16> {
    let (from, to) = port_spec.split_once('-').expect("Invalid port range");
    let from = from.parse::<u16>().expect("Invalid port range (from)");
    let to = to.parse::<u16>().expect("Invalid port range (to)");
    (from..=to).collect()
}

#[test]
fn test_correct_port_range_is_parsed() {
    assert_eq!((20..=25).collect::<Vec<_>>(), parse_port_range("20-25"));
}
