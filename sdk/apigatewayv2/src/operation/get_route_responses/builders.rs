// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_route_responses::_get_route_responses_output::GetRouteResponsesOutputBuilder;

pub use crate::operation::get_route_responses::_get_route_responses_input::GetRouteResponsesInputBuilder;

impl GetRouteResponsesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_route_responses::GetRouteResponsesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_route_responses::GetRouteResponsesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_route_responses();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetRouteResponses`.
///
/// <p>Gets the RouteResponses for a Route.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetRouteResponsesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_route_responses::builders::GetRouteResponsesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_route_responses::GetRouteResponsesOutput,
        crate::operation::get_route_responses::GetRouteResponsesError,
    > for GetRouteResponsesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_route_responses::GetRouteResponsesOutput,
            crate::operation::get_route_responses::GetRouteResponsesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetRouteResponsesFluentBuilder {
    /// Creates a new `GetRouteResponses`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetRouteResponses as a reference.
    pub fn as_input(&self) -> &crate::operation::get_route_responses::builders::GetRouteResponsesInputBuilder {
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
        crate::operation::get_route_responses::GetRouteResponsesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_route_responses::GetRouteResponsesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_route_responses::GetRouteResponses::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_route_responses::GetRouteResponses::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_route_responses::GetRouteResponsesOutput,
        crate::operation::get_route_responses::GetRouteResponsesError,
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
    /// <p>The API identifier.</p>
    pub fn api_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.api_id(input.into());
        self
    }
    /// <p>The API identifier.</p>
    pub fn set_api_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_api_id(input);
        self
    }
    /// <p>The API identifier.</p>
    pub fn get_api_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_api_id()
    }
    /// <p>The maximum number of elements to be returned for this resource.</p>
    pub fn max_results(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.max_results(input.into());
        self
    }
    /// <p>The maximum number of elements to be returned for this resource.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of elements to be returned for this resource.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_max_results()
    }
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The route ID.</p>
    pub fn route_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.route_id(input.into());
        self
    }
    /// <p>The route ID.</p>
    pub fn set_route_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_route_id(input);
        self
    }
    /// <p>The route ID.</p>
    pub fn get_route_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_route_id()
    }
}
