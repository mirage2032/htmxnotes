use actix_web::web;
use askama::Template;

mod login;
mod register;

pub struct FormElement<'a> {
    pub label: &'a str,
    pub input_type: &'a str,
    pub name: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "auth/auth.html")]
pub struct AuthTemplate<'a> {
    elements: Vec<FormElement<'a>>,
    button_text: &'a str,
    endpoint: &'a str,
}

impl<'a> AuthTemplate<'a> {
    pub fn new(endpoint: &'a str, button_text: &'a str) -> Self {
        AuthTemplate {
            elements: Vec::new(),
            button_text,
            endpoint,
        }
    }
    pub fn add_element(&mut self, label: &'a str, input_type: &'a str, name: Option<&'a str>) {
        self.elements.push(FormElement {
            label,
            input_type,
            name,
        });
    }
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login::login_get);
    cfg.service(login::login_post);
    cfg.service(register::register_get);
    cfg.service(register::register_post);
}
