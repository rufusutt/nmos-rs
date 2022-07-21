use serde::{Deserialize, Serialize};

schemafy::schemafy!(root: DeviceJson "schemas/device.json");
schemafy::schemafy!(root: DevicesJson "schemas/devices.json");
schemafy::schemafy!(root: ErrorJson "schemas/error.json");
schemafy::schemafy!(root: FlowJson "schemas/flow.json");
schemafy::schemafy!(root: FlowsJson "schemas/flows.json");
schemafy::schemafy!(root: NodeJson "schemas/node.json");
schemafy::schemafy!(root: NodesJson "schemas/nodes.json");
schemafy::schemafy!(root: ReceiverJson "schemas/receiver.json");
schemafy::schemafy!(root: ReceiversJson "schemas/receivers.json");
schemafy::schemafy!(root: SenderJson "schemas/sender.json");
schemafy::schemafy!(root: SendersJson "schemas/senders.json");
schemafy::schemafy!(root: SourceJson "schemas/source.json");
schemafy::schemafy!(root: SourcesJson "schemas/sources.json");

pub mod nodeapi {
    use super::SenderJson;
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: BaseJson "schemas/nodeapi-base.json");
    schemafy::schemafy!(root: ReceiverTargetJson "schemas/nodeapi-receiver-target.json");
}

pub mod queryapi {
    use super::{DeviceJson, FlowJson, NodeJson, ReceiverJson, SenderJson, SourceJson};
    use serde::{Deserialize, Serialize};

    type QueryapiSubscriptionResponseJson = SubscriptionResponseJson;

    schemafy::schemafy!(root: BaseJson "schemas/queryapi-base.json");
    schemafy::schemafy!(root: SubscriptionResponseJson "schemas/queryapi-subscription-response.json");
    schemafy::schemafy!(root: SubscriptionsResponseJson "schemas/queryapi-subscriptions-response.json");
    schemafy::schemafy!(root: SubscriptionsPostRequestJson "schemas/queryapi-v1.0-subscriptions-post-request.json");
    schemafy::schemafy!(root: SubscriptionsWebsocketJson "schemas/queryapi-v1.0-subscriptions-websocket.json");
}

pub mod registrationapi {
    use super::{DeviceJson, FlowJson, NodeJson, ReceiverJson, SenderJson, SourceJson};
    use serde::{Deserialize, Serialize};

    schemafy::schemafy!(root: BaseJson "schemas/registrationapi-base.json");
    schemafy::schemafy!(root: HealthResponseJson "schemas/registrationapi-health-response.json");
    schemafy::schemafy!(root: ResourceResponseJson "schemas/registrationapi-resource-response.json");
    schemafy::schemafy!(root: ResourcePostRequestJson "schemas/registrationapi-v1.0-resource-post-request.json");
}
