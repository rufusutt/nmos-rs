use nmos_rs_schema::is_04;

pub fn json<S: serde::Serialize>(json: &S) -> Response<Body> {
    let json = serde_json::to_string(json).unwrap();
    let body = Body::from(json);

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/json")
        .body(body)
        .unwrap()
}

fn error_resp(code: StatusCode, debug: Option<String>) -> Response<Body> {
    let error = is_04::v1_0_x::ErrorJson {
        code: code.as_u16() as i64,
        debug: debug.map(|d| d.into()),
        error: code.to_string(),
    };

    json(&error)
}

pub fn not_found() -> Result<Response<Body>, hyper::Error> {
    Ok(error_resp(StatusCode::NOT_FOUND, None))
}

pub fn method_not_allowed() -> Result<Response<Body>, hyper::Error> {
    Ok(error_resp(StatusCode::METHOD_NOT_ALLOWED, None))
}
