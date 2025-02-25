// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_flow::_start_flow_output::StartFlowOutputBuilder;

pub use crate::operation::start_flow::_start_flow_input::StartFlowInputBuilder;

impl StartFlowInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_flow::StartFlowOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_flow::StartFlowError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_flow();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartFlow`.
///
/// <p> Activates an existing flow. For on-demand flows, this operation runs the flow immediately. For schedule and event-triggered flows, this operation activates the flow. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartFlowFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_flow::builders::StartFlowInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::start_flow::StartFlowOutput, crate::operation::start_flow::StartFlowError>
    for StartFlowFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::start_flow::StartFlowOutput, crate::operation::start_flow::StartFlowError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartFlowFluentBuilder {
    /// Creates a new `StartFlow`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartFlow as a reference.
    pub fn as_input(&self) -> &crate::operation::start_flow::builders::StartFlowInputBuilder {
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
        crate::operation::start_flow::StartFlowOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_flow::StartFlowError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_flow::StartFlow::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_flow::StartFlow::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_flow::StartFlowOutput,
        crate::operation::start_flow::StartFlowError,
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
    /// <p> The specified name of the flow. Spaces are not allowed. Use underscores (_) or hyphens (-) only. </p>
    pub fn flow_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.flow_name(input.into());
        self
    }
    /// <p> The specified name of the flow. Spaces are not allowed. Use underscores (_) or hyphens (-) only. </p>
    pub fn set_flow_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_flow_name(input);
        self
    }
    /// <p> The specified name of the flow. Spaces are not allowed. Use underscores (_) or hyphens (-) only. </p>
    pub fn get_flow_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_flow_name()
    }
    /// <p>The <code>clientToken</code> parameter is an idempotency token. It ensures that your <code>StartFlow</code> request completes only once. You choose the value to pass. For example, if you don't receive a response from your request, you can safely retry the request with the same <code>clientToken</code> parameter value.</p>
    /// <p>If you omit a <code>clientToken</code> value, the Amazon Web Services SDK that you are using inserts a value for you. This way, the SDK can safely retry requests multiple times after a network error. You must provide your own value for other use cases.</p>
    /// <p>If you specify input parameters that differ from your first request, an error occurs for flows that run on a schedule or based on an event. However, the error doesn't occur for flows that run on demand. You set the conditions that initiate your flow for the <code>triggerConfig</code> parameter.</p>
    /// <p>If you use a different value for <code>clientToken</code>, Amazon AppFlow considers it a new call to <code>StartFlow</code>. The token is active for 8 hours.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>The <code>clientToken</code> parameter is an idempotency token. It ensures that your <code>StartFlow</code> request completes only once. You choose the value to pass. For example, if you don't receive a response from your request, you can safely retry the request with the same <code>clientToken</code> parameter value.</p>
    /// <p>If you omit a <code>clientToken</code> value, the Amazon Web Services SDK that you are using inserts a value for you. This way, the SDK can safely retry requests multiple times after a network error. You must provide your own value for other use cases.</p>
    /// <p>If you specify input parameters that differ from your first request, an error occurs for flows that run on a schedule or based on an event. However, the error doesn't occur for flows that run on demand. You set the conditions that initiate your flow for the <code>triggerConfig</code> parameter.</p>
    /// <p>If you use a different value for <code>clientToken</code>, Amazon AppFlow considers it a new call to <code>StartFlow</code>. The token is active for 8 hours.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The <code>clientToken</code> parameter is an idempotency token. It ensures that your <code>StartFlow</code> request completes only once. You choose the value to pass. For example, if you don't receive a response from your request, you can safely retry the request with the same <code>clientToken</code> parameter value.</p>
    /// <p>If you omit a <code>clientToken</code> value, the Amazon Web Services SDK that you are using inserts a value for you. This way, the SDK can safely retry requests multiple times after a network error. You must provide your own value for other use cases.</p>
    /// <p>If you specify input parameters that differ from your first request, an error occurs for flows that run on a schedule or based on an event. However, the error doesn't occur for flows that run on demand. You set the conditions that initiate your flow for the <code>triggerConfig</code> parameter.</p>
    /// <p>If you use a different value for <code>clientToken</code>, Amazon AppFlow considers it a new call to <code>StartFlow</code>. The token is active for 8 hours.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
