// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_model::_start_model_output::StartModelOutputBuilder;

pub use crate::operation::start_model::_start_model_input::StartModelInputBuilder;

impl StartModelInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_model::StartModelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_model::StartModelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_model();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartModel`.
///
/// <p>Starts the running of the version of an Amazon Lookout for Vision model. Starting a model takes a while to complete. To check the current state of the model, use <code>DescribeModel</code>.</p>
/// <p>A model is ready to use when its status is <code>HOSTED</code>.</p>
/// <p>Once the model is running, you can detect custom labels in new images by calling <code>DetectAnomalies</code>.</p> <note>
/// <p>You are charged for the amount of time that the model is running. To stop a running model, call <code>StopModel</code>.</p>
/// </note>
/// <p>This operation requires permissions to perform the <code>lookoutvision:StartModel</code> operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartModelFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_model::builders::StartModelInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_model::StartModelOutput,
        crate::operation::start_model::StartModelError,
    > for StartModelFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_model::StartModelOutput,
            crate::operation::start_model::StartModelError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartModelFluentBuilder {
    /// Creates a new `StartModel`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartModel as a reference.
    pub fn as_input(&self) -> &crate::operation::start_model::builders::StartModelInputBuilder {
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
        crate::operation::start_model::StartModelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_model::StartModelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_model::StartModel::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_model::StartModel::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_model::StartModelOutput,
        crate::operation::start_model::StartModelError,
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
    /// <p>The name of the project that contains the model that you want to start.</p>
    pub fn project_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.project_name(input.into());
        self
    }
    /// <p>The name of the project that contains the model that you want to start.</p>
    pub fn set_project_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_project_name(input);
        self
    }
    /// <p>The name of the project that contains the model that you want to start.</p>
    pub fn get_project_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_project_name()
    }
    /// <p>The version of the model that you want to start.</p>
    pub fn model_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.model_version(input.into());
        self
    }
    /// <p>The version of the model that you want to start.</p>
    pub fn set_model_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_model_version(input);
        self
    }
    /// <p>The version of the model that you want to start.</p>
    pub fn get_model_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_model_version()
    }
    /// <p>The minimum number of inference units to use. A single inference unit represents 1 hour of processing. Use a higher number to increase the TPS throughput of your model. You are charged for the number of inference units that you use. </p>
    pub fn min_inference_units(mut self, input: i32) -> Self {
        self.inner = self.inner.min_inference_units(input);
        self
    }
    /// <p>The minimum number of inference units to use. A single inference unit represents 1 hour of processing. Use a higher number to increase the TPS throughput of your model. You are charged for the number of inference units that you use. </p>
    pub fn set_min_inference_units(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_min_inference_units(input);
        self
    }
    /// <p>The minimum number of inference units to use. A single inference unit represents 1 hour of processing. Use a higher number to increase the TPS throughput of your model. You are charged for the number of inference units that you use. </p>
    pub fn get_min_inference_units(&self) -> &::std::option::Option<i32> {
        self.inner.get_min_inference_units()
    }
    /// <p>ClientToken is an idempotency token that ensures a call to <code>StartModel</code> completes only once. You choose the value to pass. For example, An issue might prevent you from getting a response from <code>StartModel</code>. In this case, safely retry your call to <code>StartModel</code> by using the same <code>ClientToken</code> parameter value. </p>
    /// <p>If you don't supply a value for <code>ClientToken</code>, the AWS SDK you are using inserts a value for you. This prevents retries after a network error from making multiple start requests. You'll need to provide your own value for other use cases. </p>
    /// <p>An error occurs if the other input parameters are not the same as in the first request. Using a different value for <code>ClientToken</code> is considered a new call to <code>StartModel</code>. An idempotency token is active for 8 hours. </p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>ClientToken is an idempotency token that ensures a call to <code>StartModel</code> completes only once. You choose the value to pass. For example, An issue might prevent you from getting a response from <code>StartModel</code>. In this case, safely retry your call to <code>StartModel</code> by using the same <code>ClientToken</code> parameter value. </p>
    /// <p>If you don't supply a value for <code>ClientToken</code>, the AWS SDK you are using inserts a value for you. This prevents retries after a network error from making multiple start requests. You'll need to provide your own value for other use cases. </p>
    /// <p>An error occurs if the other input parameters are not the same as in the first request. Using a different value for <code>ClientToken</code> is considered a new call to <code>StartModel</code>. An idempotency token is active for 8 hours. </p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>ClientToken is an idempotency token that ensures a call to <code>StartModel</code> completes only once. You choose the value to pass. For example, An issue might prevent you from getting a response from <code>StartModel</code>. In this case, safely retry your call to <code>StartModel</code> by using the same <code>ClientToken</code> parameter value. </p>
    /// <p>If you don't supply a value for <code>ClientToken</code>, the AWS SDK you are using inserts a value for you. This prevents retries after a network error from making multiple start requests. You'll need to provide your own value for other use cases. </p>
    /// <p>An error occurs if the other input parameters are not the same as in the first request. Using a different value for <code>ClientToken</code> is considered a new call to <code>StartModel</code>. An idempotency token is active for 8 hours. </p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>The maximum number of inference units to use for auto-scaling the model. If you don't specify a value, Amazon Lookout for Vision doesn't auto-scale the model.</p>
    pub fn max_inference_units(mut self, input: i32) -> Self {
        self.inner = self.inner.max_inference_units(input);
        self
    }
    /// <p>The maximum number of inference units to use for auto-scaling the model. If you don't specify a value, Amazon Lookout for Vision doesn't auto-scale the model.</p>
    pub fn set_max_inference_units(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_inference_units(input);
        self
    }
    /// <p>The maximum number of inference units to use for auto-scaling the model. If you don't specify a value, Amazon Lookout for Vision doesn't auto-scale the model.</p>
    pub fn get_max_inference_units(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_inference_units()
    }
}
