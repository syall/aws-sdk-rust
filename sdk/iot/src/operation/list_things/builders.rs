// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_things::_list_things_output::ListThingsOutputBuilder;

pub use crate::operation::list_things::_list_things_input::ListThingsInputBuilder;

impl ListThingsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_things::ListThingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_things::ListThingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_things();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListThings`.
///
/// <p>Lists your things. Use the <b>attributeName</b> and <b>attributeValue</b> parameters to filter your things. For example, calling <code>ListThings</code> with attributeName=Color and attributeValue=Red retrieves all things in the registry that contain an attribute <b>Color</b> with the value <b>Red</b>. For more information, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/thing-registry.html#list-things">List Things</a> from the <i>Amazon Web Services IoT Core Developer Guide</i>.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">ListThings</a> action.</p> <note>
/// <p>You will not be charged for calling this API if an <code>Access denied</code> error is returned. You will also not be charged if no attributes or pagination token was provided in request and no pagination token and no results were returned.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListThingsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_things::builders::ListThingsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_things::ListThingsOutput,
        crate::operation::list_things::ListThingsError,
    > for ListThingsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_things::ListThingsOutput,
            crate::operation::list_things::ListThingsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListThingsFluentBuilder {
    /// Creates a new `ListThings`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListThings as a reference.
    pub fn as_input(&self) -> &crate::operation::list_things::builders::ListThingsInputBuilder {
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
        crate::operation::list_things::ListThingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_things::ListThingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_things::ListThings::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_things::ListThings::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_things::ListThingsOutput,
        crate::operation::list_things::ListThingsError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_things::paginator::ListThingsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_things::paginator::ListThingsPaginator {
        crate::operation::list_things::paginator::ListThingsPaginator::new(self.handle, self.inner)
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <b>null</b> to receive the first set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return in this operation.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in this operation.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return in this operation.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The attribute name used to search for things.</p>
    pub fn attribute_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.attribute_name(input.into());
        self
    }
    /// <p>The attribute name used to search for things.</p>
    pub fn set_attribute_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_attribute_name(input);
        self
    }
    /// <p>The attribute name used to search for things.</p>
    pub fn get_attribute_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_attribute_name()
    }
    /// <p>The attribute value used to search for things.</p>
    pub fn attribute_value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.attribute_value(input.into());
        self
    }
    /// <p>The attribute value used to search for things.</p>
    pub fn set_attribute_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_attribute_value(input);
        self
    }
    /// <p>The attribute value used to search for things.</p>
    pub fn get_attribute_value(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_attribute_value()
    }
    /// <p>The name of the thing type used to search for things.</p>
    pub fn thing_type_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.thing_type_name(input.into());
        self
    }
    /// <p>The name of the thing type used to search for things.</p>
    pub fn set_thing_type_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_thing_type_name(input);
        self
    }
    /// <p>The name of the thing type used to search for things.</p>
    pub fn get_thing_type_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_thing_type_name()
    }
    /// <p>When <code>true</code>, the action returns the thing resources with attribute values that start with the <code>attributeValue</code> provided.</p>
    /// <p>When <code>false</code>, or not present, the action returns only the thing resources with attribute values that match the entire <code>attributeValue</code> provided. </p>
    pub fn use_prefix_attribute_value(mut self, input: bool) -> Self {
        self.inner = self.inner.use_prefix_attribute_value(input);
        self
    }
    /// <p>When <code>true</code>, the action returns the thing resources with attribute values that start with the <code>attributeValue</code> provided.</p>
    /// <p>When <code>false</code>, or not present, the action returns only the thing resources with attribute values that match the entire <code>attributeValue</code> provided. </p>
    pub fn set_use_prefix_attribute_value(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_use_prefix_attribute_value(input);
        self
    }
    /// <p>When <code>true</code>, the action returns the thing resources with attribute values that start with the <code>attributeValue</code> provided.</p>
    /// <p>When <code>false</code>, or not present, the action returns only the thing resources with attribute values that match the entire <code>attributeValue</code> provided. </p>
    pub fn get_use_prefix_attribute_value(&self) -> &::std::option::Option<bool> {
        self.inner.get_use_prefix_attribute_value()
    }
}
