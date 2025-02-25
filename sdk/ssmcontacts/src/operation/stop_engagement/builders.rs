// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::stop_engagement::_stop_engagement_output::StopEngagementOutputBuilder;

pub use crate::operation::stop_engagement::_stop_engagement_input::StopEngagementInputBuilder;

impl StopEngagementInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::stop_engagement::StopEngagementOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::stop_engagement::StopEngagementError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.stop_engagement();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StopEngagement`.
///
/// <p>Stops an engagement before it finishes the final stage of the escalation plan or engagement plan. Further contacts aren't engaged.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StopEngagementFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::stop_engagement::builders::StopEngagementInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::stop_engagement::StopEngagementOutput,
        crate::operation::stop_engagement::StopEngagementError,
    > for StopEngagementFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::stop_engagement::StopEngagementOutput,
            crate::operation::stop_engagement::StopEngagementError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StopEngagementFluentBuilder {
    /// Creates a new `StopEngagement`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StopEngagement as a reference.
    pub fn as_input(&self) -> &crate::operation::stop_engagement::builders::StopEngagementInputBuilder {
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
        crate::operation::stop_engagement::StopEngagementOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::stop_engagement::StopEngagementError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::stop_engagement::StopEngagement::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::stop_engagement::StopEngagement::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::stop_engagement::StopEngagementOutput,
        crate::operation::stop_engagement::StopEngagementError,
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
    /// <p>The Amazon Resource Name (ARN) of the engagement.</p>
    pub fn engagement_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.engagement_id(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the engagement.</p>
    pub fn set_engagement_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_engagement_id(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the engagement.</p>
    pub fn get_engagement_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_engagement_id()
    }
    /// <p>The reason that you're stopping the engagement.</p>
    pub fn reason(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.reason(input.into());
        self
    }
    /// <p>The reason that you're stopping the engagement.</p>
    pub fn set_reason(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_reason(input);
        self
    }
    /// <p>The reason that you're stopping the engagement.</p>
    pub fn get_reason(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_reason()
    }
}
