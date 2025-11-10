use std::{
    fmt::{Debug, Display},
    panic::UnwindSafe,
};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

pub mod content;

pub type BasicRequest = DtoRequest<()>;
pub type BasicResponse = DtoResponse<()>;

pub trait DtoConstraint:
    Debug + Clone + Default + Serialize + DeserializeOwned + Send + Sync + UnwindSafe + 'static
{
}

impl<T: Debug + Clone + Default + Serialize + DeserializeOwned + Send + Sync + UnwindSafe + 'static>
    DtoConstraint for T
{
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DtoRequest<T> {
    data: T,
}

impl<T: DtoConstraint> DtoRequest<T> {
    pub fn get_data(&self) -> &T {
        &self.data
    }

    pub fn set_data(&mut self, data: T) {
        self.data = data
    }
}

unsafe impl<T: DtoConstraint> Sync for DtoRequest<T> {}
unsafe impl<T: DtoConstraint> Send for DtoRequest<T> {}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DtoResponse<T> {
    status: ResponseStatus,
    data: T,
}

impl<T: DtoConstraint> DtoResponse<T> {
    pub fn get_response_status(&self) -> &ResponseStatus {
        &self.status
    }
    pub fn set_response_status(&mut self, response_status: ResponseStatus) {
        self.status = response_status
    }
    pub fn get_data(&self) -> &T {
        &self.data
    }
    pub fn set_data(&mut self, data: T) {
        self.data = data
    }
}

unsafe impl<T: DtoConstraint> Sync for DtoResponse<T> {}
unsafe impl<T: DtoConstraint> Send for DtoResponse<T> {}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseStatus {
    error_code: String,
    error_message: String,
}

impl ResponseStatus {
    pub fn is_success(&self) -> bool {
        self.error_code.is_empty()
    }

    pub fn is_failure(&self) -> bool {
        !self.is_success()
    }

    pub fn get_error_code(&self) -> &String {
        &self.error_code
    }

    pub fn set_error_code(&mut self, error_code: String) {
        self.error_code = error_code
    }

    pub fn get_error_message(&self) -> &String {
        &self.error_message
    }

    pub fn set_error_message(&mut self, error_message: String) {
        self.error_message = error_message
    }
}

impl Display for ResponseStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ error_code: \"{}\", error_message: \"{}\" }}",
            self.error_code, self.error_message
        )
    }
}

unsafe impl Sync for ResponseStatus {}
unsafe impl Send for ResponseStatus {}
