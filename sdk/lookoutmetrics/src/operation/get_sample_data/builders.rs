// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_sample_data::_get_sample_data_output::GetSampleDataOutputBuilder;

pub use crate::operation::get_sample_data::_get_sample_data_input::GetSampleDataInputBuilder;

impl GetSampleDataInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_sample_data::GetSampleDataOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_sample_data::GetSampleDataError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_sample_data();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetSampleData`.
///
/// <p>Returns a selection of sample records from an Amazon S3 datasource.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetSampleDataFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_sample_data::builders::GetSampleDataInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_sample_data::GetSampleDataOutput,
        crate::operation::get_sample_data::GetSampleDataError,
    > for GetSampleDataFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_sample_data::GetSampleDataOutput,
            crate::operation::get_sample_data::GetSampleDataError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetSampleDataFluentBuilder {
    /// Creates a new `GetSampleData`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetSampleData as a reference.
    pub fn as_input(&self) -> &crate::operation::get_sample_data::builders::GetSampleDataInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_sample_data::GetSampleDataOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_sample_data::GetSampleDataError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_sample_data::GetSampleData::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_sample_data::GetSampleData::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_sample_data::GetSampleDataOutput,
        crate::operation::get_sample_data::GetSampleDataError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>A datasource bucket in Amazon S3.</p>
    pub fn s3_source_config(mut self, input: crate::types::SampleDataS3SourceConfig) -> Self {
        self.inner = self.inner.s3_source_config(input);
        self
    }
    /// <p>A datasource bucket in Amazon S3.</p>
    pub fn set_s3_source_config(mut self, input: ::std::option::Option<crate::types::SampleDataS3SourceConfig>) -> Self {
        self.inner = self.inner.set_s3_source_config(input);
        self
    }
    /// <p>A datasource bucket in Amazon S3.</p>
    pub fn get_s3_source_config(&self) -> &::std::option::Option<crate::types::SampleDataS3SourceConfig> {
        self.inner.get_s3_source_config()
    }
}
