use nmos_schema::is_04::v1_0_x::Flow;

#[test]
fn parse_flows() {
    let flows = include_str!("examples/is_04/nodeapi-v1.0-flows-get-200.json");

    let flows: Vec<Flow> = serde_json::from_str(flows).unwrap();
}
