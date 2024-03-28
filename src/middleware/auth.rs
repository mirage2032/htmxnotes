use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, Error, HttpMessage, web};
use actix_web::dev::Service;
use crate::db::DBPool;
use crate::db::sessions::Session;

pub async fn authenticate(
    req: ServiceRequest,
    next: impl Service<ServiceRequest, Response=ServiceResponse, Error=Error>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let db = req.app_data::<web::Data<DBPool>>().expect("No DBPool in app data");
    if let Some(cookie) = req.cookie("SESSION") {
        if let Ok(user) = Session::from_token(db.as_ref(), cookie.value()){
            req.extensions_mut().insert(user);
        }
    }
    next.call(req).await
}