// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_pipeline::_create_pipeline_output::CreatePipelineOutputBuilder;

pub use crate::operation::create_pipeline::_create_pipeline_input::CreatePipelineInputBuilder;

impl CreatePipelineInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_pipeline::CreatePipelineOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_pipeline::CreatePipelineError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_pipeline();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreatePipeline`.
///
/// <p>Creates a pipeline.</p> <note>
/// <p>In the pipeline structure, you must include either <code>artifactStore</code> or <code>artifactStores</code> in your pipeline, but you cannot use both. If you create a cross-region action in your pipeline, you must use <code>artifactStores</code>.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreatePipelineFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_pipeline::builders::CreatePipelineInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_pipeline::CreatePipelineOutput,
        crate::operation::create_pipeline::CreatePipelineError,
    > for CreatePipelineFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_pipeline::CreatePipelineOutput,
            crate::operation::create_pipeline::CreatePipelineError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreatePipelineFluentBuilder {
    /// Creates a new `CreatePipeline`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreatePipeline as a reference.
    pub fn as_input(&self) -> &crate::operation::create_pipeline::builders::CreatePipelineInputBuilder {
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
        crate::operation::create_pipeline::CreatePipelineOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_pipeline::CreatePipelineError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_pipeline::CreatePipeline::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_pipeline::CreatePipeline::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_pipeline::CreatePipelineOutput,
        crate::operation::create_pipeline::CreatePipelineError,
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
    /// <p>Represents the structure of actions and stages to be performed in the pipeline. </p>
    pub fn pipeline(mut self, input: crate::types::PipelineDeclaration) -> Self {
        self.inner = self.inner.pipeline(input);
        self
    }
    /// <p>Represents the structure of actions and stages to be performed in the pipeline. </p>
    pub fn set_pipeline(mut self, input: ::std::option::Option<crate::types::PipelineDeclaration>) -> Self {
        self.inner = self.inner.set_pipeline(input);
        self
    }
    /// <p>Represents the structure of actions and stages to be performed in the pipeline. </p>
    pub fn get_pipeline(&self) -> &::std::option::Option<crate::types::PipelineDeclaration> {
        self.inner.get_pipeline()
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags for the pipeline.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags for the pipeline.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags for the pipeline.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
