use nidrs::Inject;
use nidrs_macro::{injectable, on_module_init};
use crate::user::service::UserService;

#[injectable()]
#[derive(Clone, Debug, Default)]
pub struct AppService{
    user_service: Inject<UserService>
}

impl AppService {
    pub fn get_hello_world(&self) -> String {
        self.user_service.extract().get_hello_world()
    }

    pub fn get_hello_world2(&self) -> String {
        "Hello, nidrs2xx333!".to_string()
    }
}