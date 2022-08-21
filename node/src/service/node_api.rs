use std::sync::Arc;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json};
use nmos_rs_model::resource::Resource;
use nmos_rs_model::Model;
use nmos_rs_schema::is_04;
use uuid::Uuid;

use super::ServiceError;

pub async fn get_self(
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<is_04::v1_0_x::NodeJson>, ServiceError> {
    let nodes = model.nodes().await;

    let node = nodes
        .iter()
        .next()
        .expect("Missing self resource")
        .1
        .to_json();

    Ok(Json(node))
}

pub async fn get_devices(
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<is_04::v1_0_x::DevicesJson>, ServiceError> {
    let devices = model.devices().await;

    let devices: Vec<_> = devices.iter().map(|(_, device)| device.to_json()).collect();

    Ok(Json(devices))
}

pub async fn get_device(
    Path(id): Path<Uuid>,
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<is_04::v1_0_x::DeviceJson>, ServiceError> {
    let devices = model.devices().await;

    let device = match devices.get(&id) {
        Some(d) => d.to_json(),
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
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<is_04::v1_0_x::ReceiversJson>, ServiceError> {
    let receivers = model.receivers().await;

    let receivers: Vec<_> = receivers
        .iter()
        .map(|(_, receiver)| receiver.to_json())
        .collect();

    Ok(Json(receivers))
}

pub async fn get_receiver(
    Path(id): Path<Uuid>,
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<is_04::v1_0_x::ReceiverJson>, ServiceError> {
    let receivers = model.receivers().await;

    let receiver = match receivers.get(&id) {
        Some(r) => r.to_json(),
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
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<is_04::v1_0_x::SendersJson>, ServiceError> {
    let senders = model.senders().await;

    let senders: Vec<_> = senders.iter().map(|(_, sender)| sender.to_json()).collect();

    Ok(Json(senders))
}

pub async fn get_sender(
    Path(id): Path<Uuid>,
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<is_04::v1_0_x::SenderJson>, ServiceError> {
    let senders = model.senders().await;

    let sender = match senders.get(&id) {
        Some(s) => s.to_json(),
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
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<is_04::v1_0_x::SourcesJson>, ServiceError> {
    let sources = model.sources().await;

    let sources: Vec<_> = sources.iter().map(|(_, source)| source.to_json()).collect();

    Ok(Json(sources))
}

pub async fn get_source(
    Path(id): Path<Uuid>,
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<is_04::v1_0_x::SourceJson>, ServiceError> {
    let sources = model.sources().await;

    let source = match sources.get(&id) {
        Some(s) => s.to_json(),
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
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<is_04::v1_0_x::FlowsJson>, ServiceError> {
    let flows = model.flows().await;

    let flows: Vec<_> = flows.iter().map(|(_, flow)| flow.to_json()).collect();

    Ok(Json(flows))
}

pub async fn get_flow(
    Path(id): Path<Uuid>,
    Extension(model): Extension<Arc<Model>>,
) -> Result<Json<is_04::v1_0_x::FlowJson>, ServiceError> {
    let flows = model.flows().await;

    let flow = match flows.get(&id) {
        Some(f) => f.to_json(),
        None => {
            return Err(ServiceError::new(
                StatusCode::NOT_FOUND,
                Some(format!("Flow {} does not exist", id)),
            ))
        }
    };

    Ok(Json(flow))
}
