// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::stop_key_phrases_detection_job::_stop_key_phrases_detection_job_output::StopKeyPhrasesDetectionJobOutputBuilder;

pub use crate::operation::stop_key_phrases_detection_job::_stop_key_phrases_detection_job_input::StopKeyPhrasesDetectionJobInputBuilder;

impl StopKeyPhrasesDetectionJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::stop_key_phrases_detection_job::StopKeyPhrasesDetectionJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::stop_key_phrases_detection_job::StopKeyPhrasesDetectionJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.stop_key_phrases_detection_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StopKeyPhrasesDetectionJob`.
///
/// <p>Stops a key phrases detection job in progress.</p>
/// <p>If the job state is <code>IN_PROGRESS</code> the job is marked for termination and put into the <code>STOP_REQUESTED</code> state. If the job completes before it can be stopped, it is put into the <code>COMPLETED</code> state; otherwise the job is stopped and put into the <code>STOPPED</code> state.</p>
/// <p>If the job is in the <code>COMPLETED</code> or <code>FAILED</code> state when you call the <code>StopDominantLanguageDetectionJob</code> operation, the operation returns a 400 Internal Request Exception. </p>
/// <p>When a job is stopped, any documents already processed are written to the output location.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StopKeyPhrasesDetectionJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::stop_key_phrases_detection_job::builders::StopKeyPhrasesDetectionJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::stop_key_phrases_detection_job::StopKeyPhrasesDetectionJobOutput,
        crate::operation::stop_key_phrases_detection_job::StopKeyPhrasesDetectionJobError,
    > for StopKeyPhrasesDetectionJobFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::stop_key_phrases_detection_job::StopKeyPhrasesDetectionJobOutput,
            crate::operation::stop_key_phrases_detection_job::StopKeyPhrasesDetectionJobError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StopKeyPhrasesDetectionJobFluentBuilder {
    /// Creates a new `StopKeyPhrasesDetectionJob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StopKeyPhrasesDetectionJob as a reference.
    pub fn as_input(&self) -> &crate::operation::stop_key_phrases_detection_job::builders::StopKeyPhrasesDetectionJobInputBuilder {
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
        crate::operation::stop_key_phrases_detection_job::StopKeyPhrasesDetectionJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::stop_key_phrases_detection_job::StopKeyPhrasesDetectionJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::stop_key_phrases_detection_job::StopKeyPhrasesDetectionJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::stop_key_phrases_detection_job::StopKeyPhrasesDetectionJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::stop_key_phrases_detection_job::StopKeyPhrasesDetectionJobOutput,
        crate::operation::stop_key_phrases_detection_job::StopKeyPhrasesDetectionJobError,
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
    /// <p>The identifier of the key phrases detection job to stop.</p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.job_id(input.into());
        self
    }
    /// <p>The identifier of the key phrases detection job to stop.</p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_job_id(input);
        self
    }
    /// <p>The identifier of the key phrases detection job to stop.</p>
    pub fn get_job_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_job_id()
    }
}
