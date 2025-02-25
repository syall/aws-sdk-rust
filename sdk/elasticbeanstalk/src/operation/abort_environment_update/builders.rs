// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::abort_environment_update::_abort_environment_update_output::AbortEnvironmentUpdateOutputBuilder;

pub use crate::operation::abort_environment_update::_abort_environment_update_input::AbortEnvironmentUpdateInputBuilder;

impl AbortEnvironmentUpdateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::abort_environment_update::AbortEnvironmentUpdateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::abort_environment_update::AbortEnvironmentUpdateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.abort_environment_update();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AbortEnvironmentUpdate`.
///
/// <p>Cancels in-progress environment configuration update or application version deployment.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AbortEnvironmentUpdateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::abort_environment_update::builders::AbortEnvironmentUpdateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::abort_environment_update::AbortEnvironmentUpdateOutput,
        crate::operation::abort_environment_update::AbortEnvironmentUpdateError,
    > for AbortEnvironmentUpdateFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::abort_environment_update::AbortEnvironmentUpdateOutput,
            crate::operation::abort_environment_update::AbortEnvironmentUpdateError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AbortEnvironmentUpdateFluentBuilder {
    /// Creates a new `AbortEnvironmentUpdate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AbortEnvironmentUpdate as a reference.
    pub fn as_input(&self) -> &crate::operation::abort_environment_update::builders::AbortEnvironmentUpdateInputBuilder {
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
        crate::operation::abort_environment_update::AbortEnvironmentUpdateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::abort_environment_update::AbortEnvironmentUpdateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::abort_environment_update::AbortEnvironmentUpdate::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::abort_environment_update::AbortEnvironmentUpdate::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::abort_environment_update::AbortEnvironmentUpdateOutput,
        crate::operation::abort_environment_update::AbortEnvironmentUpdateError,
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
    /// <p>This specifies the ID of the environment with the in-progress update that you want to cancel.</p>
    pub fn environment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.environment_id(input.into());
        self
    }
    /// <p>This specifies the ID of the environment with the in-progress update that you want to cancel.</p>
    pub fn set_environment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_environment_id(input);
        self
    }
    /// <p>This specifies the ID of the environment with the in-progress update that you want to cancel.</p>
    pub fn get_environment_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_environment_id()
    }
    /// <p>This specifies the name of the environment with the in-progress update that you want to cancel.</p>
    pub fn environment_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.environment_name(input.into());
        self
    }
    /// <p>This specifies the name of the environment with the in-progress update that you want to cancel.</p>
    pub fn set_environment_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_environment_name(input);
        self
    }
    /// <p>This specifies the name of the environment with the in-progress update that you want to cancel.</p>
    pub fn get_environment_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_environment_name()
    }
}
