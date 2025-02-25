// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_named_shadows_for_thing::_list_named_shadows_for_thing_output::ListNamedShadowsForThingOutputBuilder;

pub use crate::operation::list_named_shadows_for_thing::_list_named_shadows_for_thing_input::ListNamedShadowsForThingInputBuilder;

impl ListNamedShadowsForThingInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_named_shadows_for_thing::ListNamedShadowsForThingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_named_shadows_for_thing::ListNamedShadowsForThingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_named_shadows_for_thing();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListNamedShadowsForThing`.
///
/// <p>Lists the shadows for the specified thing.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">ListNamedShadowsForThing</a> action.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListNamedShadowsForThingFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_named_shadows_for_thing::builders::ListNamedShadowsForThingInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_named_shadows_for_thing::ListNamedShadowsForThingOutput,
        crate::operation::list_named_shadows_for_thing::ListNamedShadowsForThingError,
    > for ListNamedShadowsForThingFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_named_shadows_for_thing::ListNamedShadowsForThingOutput,
            crate::operation::list_named_shadows_for_thing::ListNamedShadowsForThingError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListNamedShadowsForThingFluentBuilder {
    /// Creates a new `ListNamedShadowsForThing`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListNamedShadowsForThing as a reference.
    pub fn as_input(&self) -> &crate::operation::list_named_shadows_for_thing::builders::ListNamedShadowsForThingInputBuilder {
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
        crate::operation::list_named_shadows_for_thing::ListNamedShadowsForThingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_named_shadows_for_thing::ListNamedShadowsForThingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_named_shadows_for_thing::ListNamedShadowsForThing::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_named_shadows_for_thing::ListNamedShadowsForThing::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_named_shadows_for_thing::ListNamedShadowsForThingOutput,
        crate::operation::list_named_shadows_for_thing::ListNamedShadowsForThingError,
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
    /// <p>The name of the thing.</p>
    pub fn thing_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.thing_name(input.into());
        self
    }
    /// <p>The name of the thing.</p>
    pub fn set_thing_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_thing_name(input);
        self
    }
    /// <p>The name of the thing.</p>
    pub fn get_thing_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_thing_name()
    }
    /// <p>The token to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token to retrieve the next set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The result page size.</p>
    pub fn page_size(mut self, input: i32) -> Self {
        self.inner = self.inner.page_size(input);
        self
    }
    /// <p>The result page size.</p>
    pub fn set_page_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_page_size(input);
        self
    }
    /// <p>The result page size.</p>
    pub fn get_page_size(&self) -> &::std::option::Option<i32> {
        self.inner.get_page_size()
    }
}
