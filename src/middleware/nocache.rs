use actix_web::dev::Service;
use actix_web::http::header::{self, HeaderValue};
use actix_web::{
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse},
    Error,
};

pub async fn nocache(
    req: ServiceRequest,
    next: impl Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let mut response = next.call(req).await?;
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store, max-age=0, must-revalidate"),
    );
    Ok(response)
}
