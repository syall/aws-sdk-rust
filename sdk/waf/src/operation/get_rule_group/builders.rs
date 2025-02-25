// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_rule_group::_get_rule_group_output::GetRuleGroupOutputBuilder;

pub use crate::operation::get_rule_group::_get_rule_group_input::GetRuleGroupInputBuilder;

impl GetRuleGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_rule_group::GetRuleGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_rule_group::GetRuleGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_rule_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetRuleGroup`.
///
/// <note>
/// <p>This is <b>AWS WAF Classic</b> documentation. For more information, see <a href="https://docs.aws.amazon.com/waf/latest/developerguide/classic-waf-chapter.html">AWS WAF Classic</a> in the developer guide.</p>
/// <p> <b>For the latest version of AWS WAF</b>, use the AWS WAFV2 API and see the <a href="https://docs.aws.amazon.com/waf/latest/developerguide/waf-chapter.html">AWS WAF Developer Guide</a>. With the latest version, AWS WAF has a single set of endpoints for regional and global use. </p>
/// </note>
/// <p>Returns the <code>RuleGroup</code> that is specified by the <code>RuleGroupId</code> that you included in the <code>GetRuleGroup</code> request.</p>
/// <p>To view the rules in a rule group, use <code>ListActivatedRulesInRuleGroup</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetRuleGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_rule_group::builders::GetRuleGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_rule_group::GetRuleGroupOutput,
        crate::operation::get_rule_group::GetRuleGroupError,
    > for GetRuleGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_rule_group::GetRuleGroupOutput,
            crate::operation::get_rule_group::GetRuleGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetRuleGroupFluentBuilder {
    /// Creates a new `GetRuleGroup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetRuleGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::get_rule_group::builders::GetRuleGroupInputBuilder {
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
        crate::operation::get_rule_group::GetRuleGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_rule_group::GetRuleGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_rule_group::GetRuleGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_rule_group::GetRuleGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_rule_group::GetRuleGroupOutput,
        crate::operation::get_rule_group::GetRuleGroupError,
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
    /// <p>The <code>RuleGroupId</code> of the <code>RuleGroup</code> that you want to get. <code>RuleGroupId</code> is returned by <code>CreateRuleGroup</code> and by <code>ListRuleGroups</code>.</p>
    pub fn rule_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rule_group_id(input.into());
        self
    }
    /// <p>The <code>RuleGroupId</code> of the <code>RuleGroup</code> that you want to get. <code>RuleGroupId</code> is returned by <code>CreateRuleGroup</code> and by <code>ListRuleGroups</code>.</p>
    pub fn set_rule_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_rule_group_id(input);
        self
    }
    /// <p>The <code>RuleGroupId</code> of the <code>RuleGroup</code> that you want to get. <code>RuleGroupId</code> is returned by <code>CreateRuleGroup</code> and by <code>ListRuleGroups</code>.</p>
    pub fn get_rule_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_rule_group_id()
    }
}
