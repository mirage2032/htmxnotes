use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, Error};
use actix_web::dev::Service;
use actix_web::http::header;
use actix_web::http::header::HeaderValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionCookie {
    pub token: String,
}

pub async fn nocache(
    req: ServiceRequest,
    next: impl Service<ServiceRequest, Response=ServiceResponse, Error=Error>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let mut response = next.call(req).await?;
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store, max-age=0, must-revalidate"),
    );
    Ok(response)
}