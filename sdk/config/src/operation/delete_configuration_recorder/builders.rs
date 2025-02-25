// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_configuration_recorder::_delete_configuration_recorder_output::DeleteConfigurationRecorderOutputBuilder;

pub use crate::operation::delete_configuration_recorder::_delete_configuration_recorder_input::DeleteConfigurationRecorderInputBuilder;

impl DeleteConfigurationRecorderInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_configuration_recorder::DeleteConfigurationRecorderOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_configuration_recorder::DeleteConfigurationRecorderError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_configuration_recorder();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteConfigurationRecorder`.
///
/// <p>Deletes the configuration recorder.</p>
/// <p>After the configuration recorder is deleted, Config will not record resource configuration changes until you create a new configuration recorder.</p>
/// <p>This action does not delete the configuration information that was previously recorded. You will be able to access the previously recorded information by using the <code>GetResourceConfigHistory</code> action, but you will not be able to access this information in the Config console until you create a new configuration recorder.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteConfigurationRecorderFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_configuration_recorder::builders::DeleteConfigurationRecorderInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_configuration_recorder::DeleteConfigurationRecorderOutput,
        crate::operation::delete_configuration_recorder::DeleteConfigurationRecorderError,
    > for DeleteConfigurationRecorderFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_configuration_recorder::DeleteConfigurationRecorderOutput,
            crate::operation::delete_configuration_recorder::DeleteConfigurationRecorderError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteConfigurationRecorderFluentBuilder {
    /// Creates a new `DeleteConfigurationRecorder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteConfigurationRecorder as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_configuration_recorder::builders::DeleteConfigurationRecorderInputBuilder {
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
        crate::operation::delete_configuration_recorder::DeleteConfigurationRecorderOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_configuration_recorder::DeleteConfigurationRecorderError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_configuration_recorder::DeleteConfigurationRecorder::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_configuration_recorder::DeleteConfigurationRecorder::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_configuration_recorder::DeleteConfigurationRecorderOutput,
        crate::operation::delete_configuration_recorder::DeleteConfigurationRecorderError,
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
    /// <p>The name of the configuration recorder to be deleted. You can retrieve the name of your configuration recorder by using the <code>DescribeConfigurationRecorders</code> action.</p>
    pub fn configuration_recorder_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.configuration_recorder_name(input.into());
        self
    }
    /// <p>The name of the configuration recorder to be deleted. You can retrieve the name of your configuration recorder by using the <code>DescribeConfigurationRecorders</code> action.</p>
    pub fn set_configuration_recorder_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_configuration_recorder_name(input);
        self
    }
    /// <p>The name of the configuration recorder to be deleted. You can retrieve the name of your configuration recorder by using the <code>DescribeConfigurationRecorders</code> action.</p>
    pub fn get_configuration_recorder_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_configuration_recorder_name()
    }
}
