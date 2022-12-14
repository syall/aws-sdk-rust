// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AddTagsToCertificate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`add_tags_to_certificate`](crate::client::Client::add_tags_to_certificate).
///
/// See [`crate::client::fluent_builders::AddTagsToCertificate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AddTagsToCertificate {
    _private: (),
}
impl AddTagsToCertificate {
    /// Creates a new builder-style object to manufacture [`AddTagsToCertificateInput`](crate::input::AddTagsToCertificateInput).
    pub fn builder() -> crate::input::add_tags_to_certificate_input::Builder {
        crate::input::add_tags_to_certificate_input::Builder::default()
    }
    /// Creates a new `AddTagsToCertificate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AddTagsToCertificate {
    type Output = std::result::Result<
        crate::output::AddTagsToCertificateOutput,
        crate::error::AddTagsToCertificateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_add_tags_to_certificate_error(response)
        } else {
            crate::operation_deser::parse_add_tags_to_certificate_response(response)
        }
    }
}

/// Operation shape for `DeleteCertificate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_certificate`](crate::client::Client::delete_certificate).
///
/// See [`crate::client::fluent_builders::DeleteCertificate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteCertificate {
    _private: (),
}
impl DeleteCertificate {
    /// Creates a new builder-style object to manufacture [`DeleteCertificateInput`](crate::input::DeleteCertificateInput).
    pub fn builder() -> crate::input::delete_certificate_input::Builder {
        crate::input::delete_certificate_input::Builder::default()
    }
    /// Creates a new `DeleteCertificate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteCertificate {
    type Output = std::result::Result<
        crate::output::DeleteCertificateOutput,
        crate::error::DeleteCertificateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_certificate_error(response)
        } else {
            crate::operation_deser::parse_delete_certificate_response(response)
        }
    }
}

/// Operation shape for `DescribeCertificate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_certificate`](crate::client::Client::describe_certificate).
///
/// See [`crate::client::fluent_builders::DescribeCertificate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeCertificate {
    _private: (),
}
impl DescribeCertificate {
    /// Creates a new builder-style object to manufacture [`DescribeCertificateInput`](crate::input::DescribeCertificateInput).
    pub fn builder() -> crate::input::describe_certificate_input::Builder {
        crate::input::describe_certificate_input::Builder::default()
    }
    /// Creates a new `DescribeCertificate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeCertificate {
    type Output = std::result::Result<
        crate::output::DescribeCertificateOutput,
        crate::error::DescribeCertificateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_certificate_error(response)
        } else {
            crate::operation_deser::parse_describe_certificate_response(response)
        }
    }
}

/// Operation shape for `ExportCertificate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`export_certificate`](crate::client::Client::export_certificate).
///
/// See [`crate::client::fluent_builders::ExportCertificate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ExportCertificate {
    _private: (),
}
impl ExportCertificate {
    /// Creates a new builder-style object to manufacture [`ExportCertificateInput`](crate::input::ExportCertificateInput).
    pub fn builder() -> crate::input::export_certificate_input::Builder {
        crate::input::export_certificate_input::Builder::default()
    }
    /// Creates a new `ExportCertificate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ExportCertificate {
    type Output = std::result::Result<
        crate::output::ExportCertificateOutput,
        crate::error::ExportCertificateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_export_certificate_error(response)
        } else {
            crate::operation_deser::parse_export_certificate_response(response)
        }
    }
}

/// Operation shape for `GetAccountConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_account_configuration`](crate::client::Client::get_account_configuration).
///
/// See [`crate::client::fluent_builders::GetAccountConfiguration`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetAccountConfiguration {
    _private: (),
}
impl GetAccountConfiguration {
    /// Creates a new builder-style object to manufacture [`GetAccountConfigurationInput`](crate::input::GetAccountConfigurationInput).
    pub fn builder() -> crate::input::get_account_configuration_input::Builder {
        crate::input::get_account_configuration_input::Builder::default()
    }
    /// Creates a new `GetAccountConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAccountConfiguration {
    type Output = std::result::Result<
        crate::output::GetAccountConfigurationOutput,
        crate::error::GetAccountConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_account_configuration_error(response)
        } else {
            crate::operation_deser::parse_get_account_configuration_response(response)
        }
    }
}

/// Operation shape for `GetCertificate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_certificate`](crate::client::Client::get_certificate).
///
/// See [`crate::client::fluent_builders::GetCertificate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetCertificate {
    _private: (),
}
impl GetCertificate {
    /// Creates a new builder-style object to manufacture [`GetCertificateInput`](crate::input::GetCertificateInput).
    pub fn builder() -> crate::input::get_certificate_input::Builder {
        crate::input::get_certificate_input::Builder::default()
    }
    /// Creates a new `GetCertificate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetCertificate {
    type Output =
        std::result::Result<crate::output::GetCertificateOutput, crate::error::GetCertificateError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_certificate_error(response)
        } else {
            crate::operation_deser::parse_get_certificate_response(response)
        }
    }
}

/// Operation shape for `ImportCertificate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`import_certificate`](crate::client::Client::import_certificate).
///
/// See [`crate::client::fluent_builders::ImportCertificate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ImportCertificate {
    _private: (),
}
impl ImportCertificate {
    /// Creates a new builder-style object to manufacture [`ImportCertificateInput`](crate::input::ImportCertificateInput).
    pub fn builder() -> crate::input::import_certificate_input::Builder {
        crate::input::import_certificate_input::Builder::default()
    }
    /// Creates a new `ImportCertificate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ImportCertificate {
    type Output = std::result::Result<
        crate::output::ImportCertificateOutput,
        crate::error::ImportCertificateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_import_certificate_error(response)
        } else {
            crate::operation_deser::parse_import_certificate_response(response)
        }
    }
}

/// Operation shape for `ListCertificates`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_certificates`](crate::client::Client::list_certificates).
///
/// See [`crate::client::fluent_builders::ListCertificates`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListCertificates {
    _private: (),
}
impl ListCertificates {
    /// Creates a new builder-style object to manufacture [`ListCertificatesInput`](crate::input::ListCertificatesInput).
    pub fn builder() -> crate::input::list_certificates_input::Builder {
        crate::input::list_certificates_input::Builder::default()
    }
    /// Creates a new `ListCertificates` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListCertificates {
    type Output = std::result::Result<
        crate::output::ListCertificatesOutput,
        crate::error::ListCertificatesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_certificates_error(response)
        } else {
            crate::operation_deser::parse_list_certificates_response(response)
        }
    }
}

/// Operation shape for `ListTagsForCertificate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_certificate`](crate::client::Client::list_tags_for_certificate).
///
/// See [`crate::client::fluent_builders::ListTagsForCertificate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTagsForCertificate {
    _private: (),
}
impl ListTagsForCertificate {
    /// Creates a new builder-style object to manufacture [`ListTagsForCertificateInput`](crate::input::ListTagsForCertificateInput).
    pub fn builder() -> crate::input::list_tags_for_certificate_input::Builder {
        crate::input::list_tags_for_certificate_input::Builder::default()
    }
    /// Creates a new `ListTagsForCertificate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForCertificate {
    type Output = std::result::Result<
        crate::output::ListTagsForCertificateOutput,
        crate::error::ListTagsForCertificateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_certificate_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_certificate_response(response)
        }
    }
}

/// Operation shape for `PutAccountConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_account_configuration`](crate::client::Client::put_account_configuration).
///
/// See [`crate::client::fluent_builders::PutAccountConfiguration`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct PutAccountConfiguration {
    _private: (),
}
impl PutAccountConfiguration {
    /// Creates a new builder-style object to manufacture [`PutAccountConfigurationInput`](crate::input::PutAccountConfigurationInput).
    pub fn builder() -> crate::input::put_account_configuration_input::Builder {
        crate::input::put_account_configuration_input::Builder::default()
    }
    /// Creates a new `PutAccountConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutAccountConfiguration {
    type Output = std::result::Result<
        crate::output::PutAccountConfigurationOutput,
        crate::error::PutAccountConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_account_configuration_error(response)
        } else {
            crate::operation_deser::parse_put_account_configuration_response(response)
        }
    }
}

/// Operation shape for `RemoveTagsFromCertificate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`remove_tags_from_certificate`](crate::client::Client::remove_tags_from_certificate).
///
/// See [`crate::client::fluent_builders::RemoveTagsFromCertificate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct RemoveTagsFromCertificate {
    _private: (),
}
impl RemoveTagsFromCertificate {
    /// Creates a new builder-style object to manufacture [`RemoveTagsFromCertificateInput`](crate::input::RemoveTagsFromCertificateInput).
    pub fn builder() -> crate::input::remove_tags_from_certificate_input::Builder {
        crate::input::remove_tags_from_certificate_input::Builder::default()
    }
    /// Creates a new `RemoveTagsFromCertificate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RemoveTagsFromCertificate {
    type Output = std::result::Result<
        crate::output::RemoveTagsFromCertificateOutput,
        crate::error::RemoveTagsFromCertificateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_remove_tags_from_certificate_error(response)
        } else {
            crate::operation_deser::parse_remove_tags_from_certificate_response(response)
        }
    }
}

/// Operation shape for `RenewCertificate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`renew_certificate`](crate::client::Client::renew_certificate).
///
/// See [`crate::client::fluent_builders::RenewCertificate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct RenewCertificate {
    _private: (),
}
impl RenewCertificate {
    /// Creates a new builder-style object to manufacture [`RenewCertificateInput`](crate::input::RenewCertificateInput).
    pub fn builder() -> crate::input::renew_certificate_input::Builder {
        crate::input::renew_certificate_input::Builder::default()
    }
    /// Creates a new `RenewCertificate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RenewCertificate {
    type Output = std::result::Result<
        crate::output::RenewCertificateOutput,
        crate::error::RenewCertificateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_renew_certificate_error(response)
        } else {
            crate::operation_deser::parse_renew_certificate_response(response)
        }
    }
}

/// Operation shape for `RequestCertificate`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`request_certificate`](crate::client::Client::request_certificate).
///
/// See [`crate::client::fluent_builders::RequestCertificate`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct RequestCertificate {
    _private: (),
}
impl RequestCertificate {
    /// Creates a new builder-style object to manufacture [`RequestCertificateInput`](crate::input::RequestCertificateInput).
    pub fn builder() -> crate::input::request_certificate_input::Builder {
        crate::input::request_certificate_input::Builder::default()
    }
    /// Creates a new `RequestCertificate` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RequestCertificate {
    type Output = std::result::Result<
        crate::output::RequestCertificateOutput,
        crate::error::RequestCertificateError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_request_certificate_error(response)
        } else {
            crate::operation_deser::parse_request_certificate_response(response)
        }
    }
}

/// Operation shape for `ResendValidationEmail`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`resend_validation_email`](crate::client::Client::resend_validation_email).
///
/// See [`crate::client::fluent_builders::ResendValidationEmail`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ResendValidationEmail {
    _private: (),
}
impl ResendValidationEmail {
    /// Creates a new builder-style object to manufacture [`ResendValidationEmailInput`](crate::input::ResendValidationEmailInput).
    pub fn builder() -> crate::input::resend_validation_email_input::Builder {
        crate::input::resend_validation_email_input::Builder::default()
    }
    /// Creates a new `ResendValidationEmail` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ResendValidationEmail {
    type Output = std::result::Result<
        crate::output::ResendValidationEmailOutput,
        crate::error::ResendValidationEmailError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_resend_validation_email_error(response)
        } else {
            crate::operation_deser::parse_resend_validation_email_response(response)
        }
    }
}

/// Operation shape for `UpdateCertificateOptions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_certificate_options`](crate::client::Client::update_certificate_options).
///
/// See [`crate::client::fluent_builders::UpdateCertificateOptions`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateCertificateOptions {
    _private: (),
}
impl UpdateCertificateOptions {
    /// Creates a new builder-style object to manufacture [`UpdateCertificateOptionsInput`](crate::input::UpdateCertificateOptionsInput).
    pub fn builder() -> crate::input::update_certificate_options_input::Builder {
        crate::input::update_certificate_options_input::Builder::default()
    }
    /// Creates a new `UpdateCertificateOptions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateCertificateOptions {
    type Output = std::result::Result<
        crate::output::UpdateCertificateOptionsOutput,
        crate::error::UpdateCertificateOptionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_certificate_options_error(response)
        } else {
            crate::operation_deser::parse_update_certificate_options_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
