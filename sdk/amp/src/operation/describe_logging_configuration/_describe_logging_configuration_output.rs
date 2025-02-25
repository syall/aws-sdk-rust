// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Represents the output of a DescribeLoggingConfiguration operation.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeLoggingConfigurationOutput {
    /// Metadata object containing information about the logging configuration of a workspace.
    pub logging_configuration: ::std::option::Option<crate::types::LoggingConfigurationMetadata>,
    _request_id: Option<String>,
}
impl DescribeLoggingConfigurationOutput {
    /// Metadata object containing information about the logging configuration of a workspace.
    pub fn logging_configuration(&self) -> ::std::option::Option<&crate::types::LoggingConfigurationMetadata> {
        self.logging_configuration.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeLoggingConfigurationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeLoggingConfigurationOutput {
    /// Creates a new builder-style object to manufacture [`DescribeLoggingConfigurationOutput`](crate::operation::describe_logging_configuration::DescribeLoggingConfigurationOutput).
    pub fn builder() -> crate::operation::describe_logging_configuration::builders::DescribeLoggingConfigurationOutputBuilder {
        crate::operation::describe_logging_configuration::builders::DescribeLoggingConfigurationOutputBuilder::default()
    }
}

/// A builder for [`DescribeLoggingConfigurationOutput`](crate::operation::describe_logging_configuration::DescribeLoggingConfigurationOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeLoggingConfigurationOutputBuilder {
    pub(crate) logging_configuration: ::std::option::Option<crate::types::LoggingConfigurationMetadata>,
    _request_id: Option<String>,
}
impl DescribeLoggingConfigurationOutputBuilder {
    /// Metadata object containing information about the logging configuration of a workspace.
    /// This field is required.
    pub fn logging_configuration(mut self, input: crate::types::LoggingConfigurationMetadata) -> Self {
        self.logging_configuration = ::std::option::Option::Some(input);
        self
    }
    /// Metadata object containing information about the logging configuration of a workspace.
    pub fn set_logging_configuration(mut self, input: ::std::option::Option<crate::types::LoggingConfigurationMetadata>) -> Self {
        self.logging_configuration = input;
        self
    }
    /// Metadata object containing information about the logging configuration of a workspace.
    pub fn get_logging_configuration(&self) -> &::std::option::Option<crate::types::LoggingConfigurationMetadata> {
        &self.logging_configuration
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeLoggingConfigurationOutput`](crate::operation::describe_logging_configuration::DescribeLoggingConfigurationOutput).
    pub fn build(self) -> crate::operation::describe_logging_configuration::DescribeLoggingConfigurationOutput {
        crate::operation::describe_logging_configuration::DescribeLoggingConfigurationOutput {
            logging_configuration: self.logging_configuration,
            _request_id: self._request_id,
        }
    }
}
