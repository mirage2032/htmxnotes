use crate::db::sessions::Session;
use crate::db::DBPool;
use actix_web::{
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse,Service},
    web, Error, HttpMessage,
};
use serde::{Serialize,Deserialize};

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct AuthToken {
    pub value: String,
}

impl AuthToken {
    pub fn new(value: String) -> Self {
        AuthToken { value }
    }
}

pub async fn authenticate(
    req: ServiceRequest,
    next: impl Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    let db = req
        .app_data::<web::Data<DBPool>>()
        .expect("No DBPool in app data");
    if let Some(cookie) = req.cookie("SESSION") {
        let token = cookie.value();
        if let Ok(user) = Session::from_token(db.as_ref(), token) {
            req.extensions_mut()
                .insert(AuthToken::new(token.to_string()));
            req.extensions_mut().insert(user);
        }
    }
    next.call(req).await
}
