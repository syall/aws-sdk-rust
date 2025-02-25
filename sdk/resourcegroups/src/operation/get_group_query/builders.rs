// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_group_query::_get_group_query_output::GetGroupQueryOutputBuilder;

pub use crate::operation::get_group_query::_get_group_query_input::GetGroupQueryInputBuilder;

impl GetGroupQueryInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_group_query::GetGroupQueryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_group_query::GetGroupQueryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_group_query();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetGroupQuery`.
///
/// <p>Retrieves the resource query associated with the specified resource group. For more information about resource queries, see <a href="https://docs.aws.amazon.com/ARG/latest/userguide/gettingstarted-query.html#gettingstarted-query-cli-tag">Create a tag-based group in Resource Groups</a>.</p>
/// <p> <b>Minimum permissions</b> </p>
/// <p>To run this command, you must have the following permissions:</p>
/// <ul>
/// <li> <p> <code>resource-groups:GetGroupQuery</code> </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetGroupQueryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_group_query::builders::GetGroupQueryInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_group_query::GetGroupQueryOutput,
        crate::operation::get_group_query::GetGroupQueryError,
    > for GetGroupQueryFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_group_query::GetGroupQueryOutput,
            crate::operation::get_group_query::GetGroupQueryError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetGroupQueryFluentBuilder {
    /// Creates a new `GetGroupQuery`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetGroupQuery as a reference.
    pub fn as_input(&self) -> &crate::operation::get_group_query::builders::GetGroupQueryInputBuilder {
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
        crate::operation::get_group_query::GetGroupQueryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_group_query::GetGroupQueryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_group_query::GetGroupQuery::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_group_query::GetGroupQuery::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_group_query::GetGroupQueryOutput,
        crate::operation::get_group_query::GetGroupQueryError,
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
    /// <p>Don't use this parameter. Use <code>Group</code> instead.</p>
    #[deprecated(note = "This field is deprecated, use Group instead.")]
    pub fn group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.group_name(input.into());
        self
    }
    /// <p>Don't use this parameter. Use <code>Group</code> instead.</p>
    #[deprecated(note = "This field is deprecated, use Group instead.")]
    pub fn set_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_group_name(input);
        self
    }
    /// <p>Don't use this parameter. Use <code>Group</code> instead.</p>
    #[deprecated(note = "This field is deprecated, use Group instead.")]
    pub fn get_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_group_name()
    }
    /// <p>The name or the ARN of the resource group to query.</p>
    pub fn group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.group(input.into());
        self
    }
    /// <p>The name or the ARN of the resource group to query.</p>
    pub fn set_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_group(input);
        self
    }
    /// <p>The name or the ARN of the resource group to query.</p>
    pub fn get_group(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_group()
    }
}
