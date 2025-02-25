// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_custom_metric::_update_custom_metric_output::UpdateCustomMetricOutputBuilder;

pub use crate::operation::update_custom_metric::_update_custom_metric_input::UpdateCustomMetricInputBuilder;

impl UpdateCustomMetricInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_custom_metric::UpdateCustomMetricOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_custom_metric::UpdateCustomMetricError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_custom_metric();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateCustomMetric`.
///
/// <p>Updates a Device Defender detect custom metric. </p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">UpdateCustomMetric</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateCustomMetricFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_custom_metric::builders::UpdateCustomMetricInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_custom_metric::UpdateCustomMetricOutput,
        crate::operation::update_custom_metric::UpdateCustomMetricError,
    > for UpdateCustomMetricFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_custom_metric::UpdateCustomMetricOutput,
            crate::operation::update_custom_metric::UpdateCustomMetricError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateCustomMetricFluentBuilder {
    /// Creates a new `UpdateCustomMetric`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateCustomMetric as a reference.
    pub fn as_input(&self) -> &crate::operation::update_custom_metric::builders::UpdateCustomMetricInputBuilder {
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
        crate::operation::update_custom_metric::UpdateCustomMetricOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_custom_metric::UpdateCustomMetricError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_custom_metric::UpdateCustomMetric::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_custom_metric::UpdateCustomMetric::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_custom_metric::UpdateCustomMetricOutput,
        crate::operation::update_custom_metric::UpdateCustomMetricError,
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
    /// <p> The name of the custom metric. Cannot be updated. </p>
    pub fn metric_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.metric_name(input.into());
        self
    }
    /// <p> The name of the custom metric. Cannot be updated. </p>
    pub fn set_metric_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_metric_name(input);
        self
    }
    /// <p> The name of the custom metric. Cannot be updated. </p>
    pub fn get_metric_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_metric_name()
    }
    /// <p> Field represents a friendly name in the console for the custom metric, it doesn't have to be unique. Don't use this name as the metric identifier in the device metric report. Can be updated. </p>
    pub fn display_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.display_name(input.into());
        self
    }
    /// <p> Field represents a friendly name in the console for the custom metric, it doesn't have to be unique. Don't use this name as the metric identifier in the device metric report. Can be updated. </p>
    pub fn set_display_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_display_name(input);
        self
    }
    /// <p> Field represents a friendly name in the console for the custom metric, it doesn't have to be unique. Don't use this name as the metric identifier in the device metric report. Can be updated. </p>
    pub fn get_display_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_display_name()
    }
}
