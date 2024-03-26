use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, Error, HttpMessage};
use actix_web::dev::Service;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionCookie {
    pub token: String,
}

pub async fn authenticate(
    req: ServiceRequest,
    next: impl Service<ServiceRequest, Response=ServiceResponse, Error=Error>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    //return unauthorized if SESSION not present in cookie
    if let Some(cookie) = req.cookie("SESSION") {
        let user = crate::middleware::nocache::SessionCookie {
            token: cookie.value().to_string(),
        };
        req.extensions_mut().insert(user);
    }
    // else {
    //     // If "SESSION" cookie is not present, return an unauthorized response
    //     // let (http_req, _) = req.into_parts();
    //     // let response = HttpResponse::Unauthorized().finish();
    //     // let rsp: ServiceResponse = ServiceResponse::new(http_req, response);
    //     Ok(rsp)
    // }
    next.call(req).await
}