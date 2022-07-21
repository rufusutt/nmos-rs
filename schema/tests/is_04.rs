use nmos_rs_schema::is_04::v1_0_x::{FlowsJson};

#[test]
fn parse_flows() {
    let flows = include_str!("examples/is_04/nodeapi-v1.0-flows-get-200.json");

    let _flows: FlowsJson = serde_json::from_str(flows).unwrap();
}
