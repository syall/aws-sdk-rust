// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `BatchGetRecord`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_get_record`](crate::client::Client::batch_get_record).
///
/// See [`crate::client::fluent_builders::BatchGetRecord`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchGetRecord {
    _private: (),
}
impl BatchGetRecord {
    /// Creates a new builder-style object to manufacture [`BatchGetRecordInput`](crate::input::BatchGetRecordInput).
    pub fn builder() -> crate::input::batch_get_record_input::Builder {
        crate::input::batch_get_record_input::Builder::default()
    }
    /// Creates a new `BatchGetRecord` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchGetRecord {
    type Output =
        std::result::Result<crate::output::BatchGetRecordOutput, crate::error::BatchGetRecordError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_get_record_error(response)
        } else {
            crate::operation_deser::parse_batch_get_record_response(response)
        }
    }
}

/// Operation shape for `DeleteRecord`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_record`](crate::client::Client::delete_record).
///
/// See [`crate::client::fluent_builders::DeleteRecord`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteRecord {
    _private: (),
}
impl DeleteRecord {
    /// Creates a new builder-style object to manufacture [`DeleteRecordInput`](crate::input::DeleteRecordInput).
    pub fn builder() -> crate::input::delete_record_input::Builder {
        crate::input::delete_record_input::Builder::default()
    }
    /// Creates a new `DeleteRecord` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteRecord {
    type Output =
        std::result::Result<crate::output::DeleteRecordOutput, crate::error::DeleteRecordError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_record_error(response)
        } else {
            crate::operation_deser::parse_delete_record_response(response)
        }
    }
}

/// Operation shape for `GetRecord`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_record`](crate::client::Client::get_record).
///
/// See [`crate::client::fluent_builders::GetRecord`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetRecord {
    _private: (),
}
impl GetRecord {
    /// Creates a new builder-style object to manufacture [`GetRecordInput`](crate::input::GetRecordInput).
    pub fn builder() -> crate::input::get_record_input::Builder {
        crate::input::get_record_input::Builder::default()
    }
    /// Creates a new `GetRecord` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetRecord {
    type Output = std::result::Result<crate::output::GetRecordOutput, crate::error::GetRecordError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_record_error(response)
        } else {
            crate::operation_deser::parse_get_record_response(response)
        }
    }
}

/// Operation shape for `PutRecord`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_record`](crate::client::Client::put_record).
///
/// See [`crate::client::fluent_builders::PutRecord`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutRecord {
    _private: (),
}
impl PutRecord {
    /// Creates a new builder-style object to manufacture [`PutRecordInput`](crate::input::PutRecordInput).
    pub fn builder() -> crate::input::put_record_input::Builder {
        crate::input::put_record_input::Builder::default()
    }
    /// Creates a new `PutRecord` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutRecord {
    type Output = std::result::Result<crate::output::PutRecordOutput, crate::error::PutRecordError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_record_error(response)
        } else {
            crate::operation_deser::parse_put_record_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
