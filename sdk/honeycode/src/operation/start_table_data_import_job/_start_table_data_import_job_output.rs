// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartTableDataImportJobOutput {
    /// <p> The id that is assigned to this import job. Future requests to find out the status of this import job need to send this id in the appropriate parameter in the request. </p>
    pub job_id: ::std::string::String,
    /// <p> The status of the import job immediately after submitting the request. </p>
    pub job_status: crate::types::TableDataImportJobStatus,
    _request_id: Option<String>,
}
impl StartTableDataImportJobOutput {
    /// <p> The id that is assigned to this import job. Future requests to find out the status of this import job need to send this id in the appropriate parameter in the request. </p>
    pub fn job_id(&self) -> &str {
        use std::ops::Deref;
        self.job_id.deref()
    }
    /// <p> The status of the import job immediately after submitting the request. </p>
    pub fn job_status(&self) -> &crate::types::TableDataImportJobStatus {
        &self.job_status
    }
}
impl ::aws_http::request_id::RequestId for StartTableDataImportJobOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StartTableDataImportJobOutput {
    /// Creates a new builder-style object to manufacture [`StartTableDataImportJobOutput`](crate::operation::start_table_data_import_job::StartTableDataImportJobOutput).
    pub fn builder() -> crate::operation::start_table_data_import_job::builders::StartTableDataImportJobOutputBuilder {
        crate::operation::start_table_data_import_job::builders::StartTableDataImportJobOutputBuilder::default()
    }
}

/// A builder for [`StartTableDataImportJobOutput`](crate::operation::start_table_data_import_job::StartTableDataImportJobOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct StartTableDataImportJobOutputBuilder {
    pub(crate) job_id: ::std::option::Option<::std::string::String>,
    pub(crate) job_status: ::std::option::Option<crate::types::TableDataImportJobStatus>,
    _request_id: Option<String>,
}
impl StartTableDataImportJobOutputBuilder {
    /// <p> The id that is assigned to this import job. Future requests to find out the status of this import job need to send this id in the appropriate parameter in the request. </p>
    /// This field is required.
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The id that is assigned to this import job. Future requests to find out the status of this import job need to send this id in the appropriate parameter in the request. </p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_id = input;
        self
    }
    /// <p> The id that is assigned to this import job. Future requests to find out the status of this import job need to send this id in the appropriate parameter in the request. </p>
    pub fn get_job_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.job_id
    }
    /// <p> The status of the import job immediately after submitting the request. </p>
    /// This field is required.
    pub fn job_status(mut self, input: crate::types::TableDataImportJobStatus) -> Self {
        self.job_status = ::std::option::Option::Some(input);
        self
    }
    /// <p> The status of the import job immediately after submitting the request. </p>
    pub fn set_job_status(mut self, input: ::std::option::Option<crate::types::TableDataImportJobStatus>) -> Self {
        self.job_status = input;
        self
    }
    /// <p> The status of the import job immediately after submitting the request. </p>
    pub fn get_job_status(&self) -> &::std::option::Option<crate::types::TableDataImportJobStatus> {
        &self.job_status
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`StartTableDataImportJobOutput`](crate::operation::start_table_data_import_job::StartTableDataImportJobOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`job_id`](crate::operation::start_table_data_import_job::builders::StartTableDataImportJobOutputBuilder::job_id)
    /// - [`job_status`](crate::operation::start_table_data_import_job::builders::StartTableDataImportJobOutputBuilder::job_status)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_table_data_import_job::StartTableDataImportJobOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::start_table_data_import_job::StartTableDataImportJobOutput {
            job_id: self.job_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "job_id",
                    "job_id was not specified but it is required when building StartTableDataImportJobOutput",
                )
            })?,
            job_status: self.job_status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "job_status",
                    "job_status was not specified but it is required when building StartTableDataImportJobOutput",
                )
            })?,
            _request_id: self._request_id,
        })
    }
}
