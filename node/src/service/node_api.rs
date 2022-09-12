use std::str::FromStr;
use std::sync::Arc;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json};
use nmos_rs_model::resource::{
    DeviceJson, FlowJson, NodeJson, ReceiverJson, SenderJson, SourceJson,
};
use nmos_rs_model::version::is_04::V1_0;
use nmos_rs_model::version::APIVersion;
use nmos_rs_model::Model;
use uuid::Uuid;

use super::ServiceError;

const SUPPORTED_API_VERSIONS: &[APIVersion] = &[V1_0];

fn parse_api_version(api: &str) -> Result<APIVersion, ServiceError> {
    let api = match APIVersion::from_str(api) {
        Ok(api) => api,
        Err(_) => {
            return Err(ServiceError::new(
                StatusCode::BAD_REQUEST,
                Some(String::from("API version badly formed")),
            ))
        }
    };

    if !SUPPORTED_API_VERSIONS.contains(&api) {
        return Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            Some(format!("Unsupported API: {}", api)),
        ));
    }

    Ok(api)
}

pub async fn get_self(
    Path(api): Path<String>,
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<NodeJson>, ServiceError> {
    let api = parse_api_version(&api)?;

    let nodes = model.nodes().await;

    let node = nodes
        .iter()
        .next()
        .expect("Missing self resource")
        .1
        .to_json(&api);

    Ok(Json(node))
}

pub async fn get_devices(
    Path(api): Path<String>,
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<Vec<DeviceJson>>, ServiceError> {
    let api = parse_api_version(&api)?;

    let devices = model.devices().await;

    let devices: Vec<_> = devices
        .iter()
        .map(|(_, device)| device.to_json(&api))
        .collect();

    Ok(Json(devices))
}

pub async fn get_device(
    Path((api, id)): Path<(String, Uuid)>,
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<DeviceJson>, ServiceError> {
    let api = parse_api_version(&api)?;

    let devices = model.devices().await;

    let device = match devices.get(&id) {
        Some(d) => d.to_json(&api),

        None => {
            return Err(ServiceError::new(
                StatusCode::NOT_FOUND,
                Some(format!("Device {} does not exist", id)),
            ))
        }
    };

    Ok(Json(device))
}

pub async fn get_receivers(
    Path(api): Path<String>,
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<Vec<ReceiverJson>>, ServiceError> {
    let api = parse_api_version(&api)?;

    let receivers = model.receivers().await;

    let receivers: Vec<_> = receivers
        .iter()
        .map(|(_, receiver)| receiver.to_json(&api))
        .collect();

    Ok(Json(receivers))
}

pub async fn get_receiver(
    Path((api, id)): Path<(String, Uuid)>,
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<ReceiverJson>, ServiceError> {
    let api = parse_api_version(&api)?;

    let receivers = model.receivers().await;

    let receiver = match receivers.get(&id) {
        Some(r) => r.to_json(&api),
        None => {
            return Err(ServiceError::new(
                StatusCode::NOT_FOUND,
                Some(format!("Receiver {} does not exist", id)),
            ))
        }
    };

    Ok(Json(receiver))
}

pub async fn get_senders(
    Path(api): Path<String>,
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<Vec<SenderJson>>, ServiceError> {
    let api = parse_api_version(&api)?;

    let senders = model.senders().await;

    let senders: Vec<_> = senders
        .iter()
        .map(|(_, sender)| sender.to_json(&api))
        .collect();

    Ok(Json(senders))
}

pub async fn get_sender(
    Path((api, id)): Path<(String, Uuid)>,
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<SenderJson>, ServiceError> {
    let api = parse_api_version(&api)?;

    let senders = model.senders().await;

    let sender = match senders.get(&id) {
        Some(s) => s.to_json(&api),
        None => {
            return Err(ServiceError::new(
                StatusCode::NOT_FOUND,
                Some(format!("Sender {} does not exist", id)),
            ))
        }
    };

    Ok(Json(sender))
}

pub async fn get_sources(
    Path(api): Path<String>,
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<Vec<SourceJson>>, ServiceError> {
    let api = parse_api_version(&api)?;

    let sources = model.sources().await;

    let sources: Vec<_> = sources
        .iter()
        .map(|(_, source)| source.to_json(&api))
        .collect();

    Ok(Json(sources))
}

pub async fn get_source(
    Path((api, id)): Path<(String, Uuid)>,
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<SourceJson>, ServiceError> {
    let api = parse_api_version(&api)?;

    let sources = model.sources().await;

    let source = match sources.get(&id) {
        Some(s) => s.to_json(&api),
        None => {
            return Err(ServiceError::new(
                StatusCode::NOT_FOUND,
                Some(format!("Source {} does not exist", id)),
            ))
        }
    };

    Ok(Json(source))
}

pub async fn get_flows(
    Path(api): Path<String>,
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<Vec<FlowJson>>, ServiceError> {
    let api = parse_api_version(&api)?;

    let flows = model.flows().await;

    let flows: Vec<_> = flows.iter().map(|(_, flow)| flow.to_json(&api)).collect();

    Ok(Json(flows))
}

pub async fn get_flow(
    Path((api, id)): Path<(String, Uuid)>,
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<FlowJson>, ServiceError> {
    let api = parse_api_version(&api)?;

    let flows = model.flows().await;

    let flow = match flows.get(&id) {
        Some(f) => f.to_json(&api),
        None => {
            return Err(ServiceError::new(
                StatusCode::NOT_FOUND,
                Some(format!("Flow {} does not exist", id)),
            ))
        }
    };

    Ok(Json(flow))
}
