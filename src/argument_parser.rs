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

pub(crate) fn parse_port_list(port_spec: &str) -> Vec<u16> {
    let mut ports: Vec<u16> = Vec::new();
    for port in port_spec.split(',') {
        ports.push(port.parse().expect("Invalid port list"));
    }
    ports
}

#[test]
fn test_correct_port_list_is_parsed() {
    assert_eq!(
        vec![21, 22, 25, 80, 443],
        parse_port_list("21,22,25,80,443")
    )
}
