// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_job_bookmark::_get_job_bookmark_output::GetJobBookmarkOutputBuilder;

pub use crate::operation::get_job_bookmark::_get_job_bookmark_input::GetJobBookmarkInputBuilder;

impl GetJobBookmarkInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_job_bookmark::GetJobBookmarkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_job_bookmark::GetJobBookmarkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_job_bookmark();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetJobBookmark`.
///
/// <p>Returns information on a job bookmark entry.</p>
/// <p>For more information about enabling and using job bookmarks, see:</p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/glue/latest/dg/monitor-continuations.html">Tracking processed data using job bookmarks</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-programming-etl-glue-arguments.html">Job parameters used by Glue</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/glue/latest/dg/aws-glue-api-jobs-job.html#aws-glue-api-jobs-job-Job">Job structure</a> </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetJobBookmarkFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_job_bookmark::builders::GetJobBookmarkInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_job_bookmark::GetJobBookmarkOutput,
        crate::operation::get_job_bookmark::GetJobBookmarkError,
    > for GetJobBookmarkFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_job_bookmark::GetJobBookmarkOutput,
            crate::operation::get_job_bookmark::GetJobBookmarkError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetJobBookmarkFluentBuilder {
    /// Creates a new `GetJobBookmark`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetJobBookmark as a reference.
    pub fn as_input(&self) -> &crate::operation::get_job_bookmark::builders::GetJobBookmarkInputBuilder {
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
        crate::operation::get_job_bookmark::GetJobBookmarkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_job_bookmark::GetJobBookmarkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_job_bookmark::GetJobBookmark::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_job_bookmark::GetJobBookmark::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_job_bookmark::GetJobBookmarkOutput,
        crate::operation::get_job_bookmark::GetJobBookmarkError,
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
    /// <p>The name of the job in question.</p>
    pub fn job_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.job_name(input.into());
        self
    }
    /// <p>The name of the job in question.</p>
    pub fn set_job_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_job_name(input);
        self
    }
    /// <p>The name of the job in question.</p>
    pub fn get_job_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_job_name()
    }
    /// <p>The unique run identifier associated with this job run.</p>
    pub fn run_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.run_id(input.into());
        self
    }
    /// <p>The unique run identifier associated with this job run.</p>
    pub fn set_run_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_run_id(input);
        self
    }
    /// <p>The unique run identifier associated with this job run.</p>
    pub fn get_run_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_run_id()
    }
}
