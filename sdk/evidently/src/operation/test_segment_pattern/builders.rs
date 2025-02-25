// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::test_segment_pattern::_test_segment_pattern_output::TestSegmentPatternOutputBuilder;

pub use crate::operation::test_segment_pattern::_test_segment_pattern_input::TestSegmentPatternInputBuilder;

impl TestSegmentPatternInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::test_segment_pattern::TestSegmentPatternOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::test_segment_pattern::TestSegmentPatternError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.test_segment_pattern();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `TestSegmentPattern`.
///
/// <p>Use this operation to test a rules pattern that you plan to use to create an audience segment. For more information about segments, see <a href="https://docs.aws.amazon.com/cloudwatchevidently/latest/APIReference/API_CreateSegment.html">CreateSegment</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct TestSegmentPatternFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::test_segment_pattern::builders::TestSegmentPatternInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::test_segment_pattern::TestSegmentPatternOutput,
        crate::operation::test_segment_pattern::TestSegmentPatternError,
    > for TestSegmentPatternFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::test_segment_pattern::TestSegmentPatternOutput,
            crate::operation::test_segment_pattern::TestSegmentPatternError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl TestSegmentPatternFluentBuilder {
    /// Creates a new `TestSegmentPattern`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the TestSegmentPattern as a reference.
    pub fn as_input(&self) -> &crate::operation::test_segment_pattern::builders::TestSegmentPatternInputBuilder {
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
        crate::operation::test_segment_pattern::TestSegmentPatternOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::test_segment_pattern::TestSegmentPatternError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::test_segment_pattern::TestSegmentPattern::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::test_segment_pattern::TestSegmentPattern::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::test_segment_pattern::TestSegmentPatternOutput,
        crate::operation::test_segment_pattern::TestSegmentPatternError,
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
    /// <p>The pattern to test.</p>
    pub fn pattern(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.pattern(input.into());
        self
    }
    /// <p>The pattern to test.</p>
    pub fn set_pattern(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_pattern(input);
        self
    }
    /// <p>The pattern to test.</p>
    pub fn get_pattern(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_pattern()
    }
    /// <p>A sample <code>evaluationContext</code> JSON block to test against the specified pattern.</p>
    pub fn payload(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.payload(input.into());
        self
    }
    /// <p>A sample <code>evaluationContext</code> JSON block to test against the specified pattern.</p>
    pub fn set_payload(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_payload(input);
        self
    }
    /// <p>A sample <code>evaluationContext</code> JSON block to test against the specified pattern.</p>
    pub fn get_payload(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_payload()
    }
}
