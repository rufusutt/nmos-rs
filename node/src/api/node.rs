use std::str::FromStr;
use std::sync::Arc;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json};
use nmos_model::resource::{DeviceJson, FlowJson, NodeJson, ReceiverJson, SenderJson, SourceJson};
use nmos_model::version::is_04::V1_0;
use nmos_model::version::APIVersion;
use nmos_model::Model;
use tokio::sync::RwLock;
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
    Extension(model): Extension<Arc<RwLock<Model>>>,
) -> Result<Json<NodeJson>, ServiceError> {
    let api = parse_api_version(&api)?;

    let model = model.read().await;

    let node = model
        .nodes
        .values()
        .next()
        .expect("Missing self resource")
        .to_json(&api);

    Ok(Json(node))
}

pub async fn get_devices(
    Path(api): Path<String>,
    Extension(model): Extension<Arc<RwLock<Model>>>,
) -> Result<Json<Vec<DeviceJson>>, ServiceError> {
    let api = parse_api_version(&api)?;

    let model = model.read().await;

    let devices: Vec<_> = model
        .devices
        .values()
        .map(|device| device.to_json(&api))
        .collect();

    Ok(Json(devices))
}

pub async fn get_device(
    Path((api, id)): Path<(String, Uuid)>,
    Extension(model): Extension<Arc<RwLock<Model>>>,
) -> Result<Json<DeviceJson>, ServiceError> {
    let api = parse_api_version(&api)?;

    let model = model.read().await;

    let device = match model.devices.get(&id) {
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
    Extension(model): Extension<Arc<RwLock<Model>>>,
) -> Result<Json<Vec<ReceiverJson>>, ServiceError> {
    let api = parse_api_version(&api)?;

    let model = model.read().await;

    let receivers: Vec<_> = model
        .receivers
        .values()
        .map(|receiver| receiver.to_json(&api))
        .collect();

    Ok(Json(receivers))
}

pub async fn get_receiver(
    Path((api, id)): Path<(String, Uuid)>,
    Extension(model): Extension<Arc<RwLock<Model>>>,
) -> Result<Json<ReceiverJson>, ServiceError> {
    let api = parse_api_version(&api)?;

    let model = model.read().await;

    let receiver = match model.receivers.get(&id) {
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
    Extension(model): Extension<Arc<RwLock<Model>>>,
) -> Result<Json<Vec<SenderJson>>, ServiceError> {
    let api = parse_api_version(&api)?;

    let model = model.read().await;

    let senders: Vec<_> = model
        .senders
        .values()
        .map(|sender| sender.to_json(&api))
        .collect();

    Ok(Json(senders))
}

pub async fn get_sender(
    Path((api, id)): Path<(String, Uuid)>,
    Extension(model): Extension<Arc<RwLock<Model>>>,
) -> Result<Json<SenderJson>, ServiceError> {
    let api = parse_api_version(&api)?;

    let model = model.read().await;

    let sender = match model.senders.get(&id) {
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
    Extension(model): Extension<Arc<RwLock<Model>>>,
) -> Result<Json<Vec<SourceJson>>, ServiceError> {
    let api = parse_api_version(&api)?;

    let model = model.read().await;

    let sources: Vec<_> = model
        .sources
        .values()
        .map(|source| source.to_json(&api))
        .collect();

    Ok(Json(sources))
}

pub async fn get_source(
    Path((api, id)): Path<(String, Uuid)>,
    Extension(model): Extension<Arc<RwLock<Model>>>,
) -> Result<Json<SourceJson>, ServiceError> {
    let api = parse_api_version(&api)?;

    let model = model.read().await;

    let source = match model.sources.get(&id) {
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
    Extension(model): Extension<Arc<RwLock<Model>>>,
) -> Result<Json<Vec<FlowJson>>, ServiceError> {
    let api = parse_api_version(&api)?;

    let model = model.read().await;

    let flows: Vec<_> = model
        .flows
        .values()
        .map(|flow| flow.to_json(&api))
        .collect();

    Ok(Json(flows))
}

pub async fn get_flow(
    Path((api, id)): Path<(String, Uuid)>,
    Extension(model): Extension<Arc<RwLock<Model>>>,
) -> Result<Json<FlowJson>, ServiceError> {
    let api = parse_api_version(&api)?;

    let model = model.read().await;

    let flow = match model.flows.get(&id) {
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
