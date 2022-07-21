use super::response;

use std::str::Split;
use std::sync::Arc;

use hyper::header::CONTENT_TYPE;
use hyper::{Body, Method, Request, Response, StatusCode};

use nmos_rs_model::resource::Resource;
use nmos_rs_model::Model;

pub async fn get_self(_: Request<Body>, model: Arc<Model>) -> Result<Response<Body>, hyper::Error> {
    let nodes = model.nodes().await;

    let node = nodes
        .iter()
        .next()
        .expect("Missing self resource")
        .1
        .to_json();
    let self_json = serde_json::to_string(&node).unwrap();

    let body = Body::from(self_json);

    let resp = Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .unwrap();

    Ok(resp)
}

pub async fn get_devices(
    _: Request<Body>,
    model: Arc<Model>,
) -> Result<Response<Body>, hyper::Error> {
    let devices = model.devices().await;

    let devices: Vec<_> = devices.iter().map(|(_, device)| device.to_json()).collect();

    Ok(response::json(&devices))
}

pub async fn get_receivers(
    _: Request<Body>,
    model: Arc<Model>,
) -> Result<Response<Body>, hyper::Error> {
    let receivers = model.receivers().await;

    let receivers: Vec<_> = receivers
        .iter()
        .map(|(_, receiver)| receiver.to_json())
        .collect();

    Ok(response::json(&receivers))
}

pub async fn get_senders(
    _: Request<Body>,
    model: Arc<Model>,
) -> Result<Response<Body>, hyper::Error> {
    let senders = model.senders().await;

    let senders: Vec<_> = senders.iter().map(|(_, sender)| sender.to_json()).collect();

    Ok(response::json(&senders))
}

pub async fn get_sources(
    _: Request<Body>,
    model: Arc<Model>,
) -> Result<Response<Body>, hyper::Error> {
    let sources = model.sources().await;

    let sources: Vec<_> = sources.iter().map(|(_, source)| source.to_json()).collect();

    Ok(response::json(&sources))
}

pub async fn get_flows(
    _: Request<Body>,
    model: Arc<Model>,
) -> Result<Response<Body>, hyper::Error> {
    let flows = model.flows().await;

    let flows: Vec<_> = flows.iter().map(|(_, flow)| flow.to_json()).collect();

    Ok(response::json(&flows))
}

pub async fn respond_v1_0<'a>(
    req: Request<Body>,
    mut split_path: Split<'a, char>,
    model: Arc<Model>,
) -> Result<Response<Body>, hyper::Error> {
    match (split_path.next(), req.method()) {
        // GET resources
        (Some("self"), &Method::GET) => get_self(req, model).await,
        (Some("devices"), &Method::GET) => get_devices(req, model).await,
        (Some("receivers"), &Method::GET) => get_receivers(req, model).await,
        (Some("senders"), &Method::GET) => get_senders(req, model).await,
        (Some("sources"), &Method::GET) => get_sources(req, model).await,
        (Some("flows"), &Method::GET) => get_flows(req, model).await,
        // Unknown path
        (Some(_), _) => response::not_found(),
        // GET root
        (None, &Method::GET) => {
            let body =
                Body::from(r#"["devices/","flows/","receivers/","self/","senders/","sources/"]"#);

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, "application/json")
                .body(body)
                .unwrap())
        }
        (None, _) => response::method_not_allowed(),
    }
}

pub async fn respond<'a>(
    req: Request<Body>,
    mut split_path: Split<'a, char>,
    model: Arc<Model>,
) -> Result<Response<Body>, hyper::Error> {
    match (split_path.next(), req.method()) {
        // Match node API version
        (Some("v1.0"), _) => respond_v1_0(req, split_path, model).await,
        // Unknown path
        (Some(_), _) => response::not_found(),
        // GET root
        (None, &Method::GET) => {
            let body = Body::from(r#"["v1.0/"]"#);

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, "application/json")
                .body(body)
                .unwrap())
        }
        // Method not allowed
        (None, _) => response::method_not_allowed(),
    }
}
