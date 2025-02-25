// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_archive_rule::_get_archive_rule_output::GetArchiveRuleOutputBuilder;

pub use crate::operation::get_archive_rule::_get_archive_rule_input::GetArchiveRuleInputBuilder;

impl GetArchiveRuleInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_archive_rule::GetArchiveRuleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_archive_rule::GetArchiveRuleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_archive_rule();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetArchiveRule`.
///
/// <p>Retrieves information about an archive rule.</p>
/// <p>To learn about filter keys that you can use to create an archive rule, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/access-analyzer-reference-filter-keys.html">IAM Access Analyzer filter keys</a> in the <b>IAM User Guide</b>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetArchiveRuleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_archive_rule::builders::GetArchiveRuleInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_archive_rule::GetArchiveRuleOutput,
        crate::operation::get_archive_rule::GetArchiveRuleError,
    > for GetArchiveRuleFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_archive_rule::GetArchiveRuleOutput,
            crate::operation::get_archive_rule::GetArchiveRuleError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetArchiveRuleFluentBuilder {
    /// Creates a new `GetArchiveRule`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetArchiveRule as a reference.
    pub fn as_input(&self) -> &crate::operation::get_archive_rule::builders::GetArchiveRuleInputBuilder {
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
        crate::operation::get_archive_rule::GetArchiveRuleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_archive_rule::GetArchiveRuleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_archive_rule::GetArchiveRule::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_archive_rule::GetArchiveRule::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_archive_rule::GetArchiveRuleOutput,
        crate::operation::get_archive_rule::GetArchiveRuleError,
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
    /// <p>The name of the analyzer to retrieve rules from.</p>
    pub fn analyzer_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.analyzer_name(input.into());
        self
    }
    /// <p>The name of the analyzer to retrieve rules from.</p>
    pub fn set_analyzer_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_analyzer_name(input);
        self
    }
    /// <p>The name of the analyzer to retrieve rules from.</p>
    pub fn get_analyzer_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_analyzer_name()
    }
    /// <p>The name of the rule to retrieve.</p>
    pub fn rule_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rule_name(input.into());
        self
    }
    /// <p>The name of the rule to retrieve.</p>
    pub fn set_rule_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_rule_name(input);
        self
    }
    /// <p>The name of the rule to retrieve.</p>
    pub fn get_rule_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_rule_name()
    }
}
