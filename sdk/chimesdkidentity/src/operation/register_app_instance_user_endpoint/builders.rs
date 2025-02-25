// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::register_app_instance_user_endpoint::_register_app_instance_user_endpoint_output::RegisterAppInstanceUserEndpointOutputBuilder;

pub use crate::operation::register_app_instance_user_endpoint::_register_app_instance_user_endpoint_input::RegisterAppInstanceUserEndpointInputBuilder;

impl RegisterAppInstanceUserEndpointInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::register_app_instance_user_endpoint::RegisterAppInstanceUserEndpointOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::register_app_instance_user_endpoint::RegisterAppInstanceUserEndpointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.register_app_instance_user_endpoint();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RegisterAppInstanceUserEndpoint`.
///
/// <p>Registers an endpoint under an Amazon Chime <code>AppInstanceUser</code>. The endpoint receives messages for a user. For push notifications, the endpoint is a mobile device used to receive mobile push notifications for a user.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RegisterAppInstanceUserEndpointFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::register_app_instance_user_endpoint::builders::RegisterAppInstanceUserEndpointInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::register_app_instance_user_endpoint::RegisterAppInstanceUserEndpointOutput,
        crate::operation::register_app_instance_user_endpoint::RegisterAppInstanceUserEndpointError,
    > for RegisterAppInstanceUserEndpointFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::register_app_instance_user_endpoint::RegisterAppInstanceUserEndpointOutput,
            crate::operation::register_app_instance_user_endpoint::RegisterAppInstanceUserEndpointError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RegisterAppInstanceUserEndpointFluentBuilder {
    /// Creates a new `RegisterAppInstanceUserEndpoint`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RegisterAppInstanceUserEndpoint as a reference.
    pub fn as_input(&self) -> &crate::operation::register_app_instance_user_endpoint::builders::RegisterAppInstanceUserEndpointInputBuilder {
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
        crate::operation::register_app_instance_user_endpoint::RegisterAppInstanceUserEndpointOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::register_app_instance_user_endpoint::RegisterAppInstanceUserEndpointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::register_app_instance_user_endpoint::RegisterAppInstanceUserEndpoint::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::register_app_instance_user_endpoint::RegisterAppInstanceUserEndpoint::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::register_app_instance_user_endpoint::RegisterAppInstanceUserEndpointOutput,
        crate::operation::register_app_instance_user_endpoint::RegisterAppInstanceUserEndpointError,
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
    /// <p>The ARN of the <code>AppInstanceUser</code>.</p>
    pub fn app_instance_user_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.app_instance_user_arn(input.into());
        self
    }
    /// <p>The ARN of the <code>AppInstanceUser</code>.</p>
    pub fn set_app_instance_user_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_app_instance_user_arn(input);
        self
    }
    /// <p>The ARN of the <code>AppInstanceUser</code>.</p>
    pub fn get_app_instance_user_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_app_instance_user_arn()
    }
    /// <p>The name of the <code>AppInstanceUserEndpoint</code>.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the <code>AppInstanceUserEndpoint</code>.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the <code>AppInstanceUserEndpoint</code>.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The type of the <code>AppInstanceUserEndpoint</code>. Supported types:</p>
    /// <ul>
    /// <li> <p> <code>APNS</code>: The mobile notification service for an Apple device.</p> </li>
    /// <li> <p> <code>APNS_SANDBOX</code>: The sandbox environment of the mobile notification service for an Apple device.</p> </li>
    /// <li> <p> <code>GCM</code>: The mobile notification service for an Android device.</p> </li>
    /// </ul>
    /// <p>Populate the <code>ResourceArn</code> value of each type as <code>PinpointAppArn</code>.</p>
    pub fn r#type(mut self, input: crate::types::AppInstanceUserEndpointType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The type of the <code>AppInstanceUserEndpoint</code>. Supported types:</p>
    /// <ul>
    /// <li> <p> <code>APNS</code>: The mobile notification service for an Apple device.</p> </li>
    /// <li> <p> <code>APNS_SANDBOX</code>: The sandbox environment of the mobile notification service for an Apple device.</p> </li>
    /// <li> <p> <code>GCM</code>: The mobile notification service for an Android device.</p> </li>
    /// </ul>
    /// <p>Populate the <code>ResourceArn</code> value of each type as <code>PinpointAppArn</code>.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::AppInstanceUserEndpointType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The type of the <code>AppInstanceUserEndpoint</code>. Supported types:</p>
    /// <ul>
    /// <li> <p> <code>APNS</code>: The mobile notification service for an Apple device.</p> </li>
    /// <li> <p> <code>APNS_SANDBOX</code>: The sandbox environment of the mobile notification service for an Apple device.</p> </li>
    /// <li> <p> <code>GCM</code>: The mobile notification service for an Android device.</p> </li>
    /// </ul>
    /// <p>Populate the <code>ResourceArn</code> value of each type as <code>PinpointAppArn</code>.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::AppInstanceUserEndpointType> {
        self.inner.get_type()
    }
    /// <p>The ARN of the resource to which the endpoint belongs.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>The ARN of the resource to which the endpoint belongs.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
    /// <p>The ARN of the resource to which the endpoint belongs.</p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_arn()
    }
    /// <p>The attributes of an <code>Endpoint</code>.</p>
    pub fn endpoint_attributes(mut self, input: crate::types::EndpointAttributes) -> Self {
        self.inner = self.inner.endpoint_attributes(input);
        self
    }
    /// <p>The attributes of an <code>Endpoint</code>.</p>
    pub fn set_endpoint_attributes(mut self, input: ::std::option::Option<crate::types::EndpointAttributes>) -> Self {
        self.inner = self.inner.set_endpoint_attributes(input);
        self
    }
    /// <p>The attributes of an <code>Endpoint</code>.</p>
    pub fn get_endpoint_attributes(&self) -> &::std::option::Option<crate::types::EndpointAttributes> {
        self.inner.get_endpoint_attributes()
    }
    /// <p>The unique ID assigned to the request. Use different tokens to register other endpoints.</p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>The unique ID assigned to the request. Use different tokens to register other endpoints.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>The unique ID assigned to the request. Use different tokens to register other endpoints.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_request_token()
    }
    /// <p>Boolean that controls whether the AppInstanceUserEndpoint is opted in to receive messages. <code>ALL</code> indicates the endpoint receives all messages. <code>NONE</code> indicates the endpoint receives no messages.</p>
    pub fn allow_messages(mut self, input: crate::types::AllowMessages) -> Self {
        self.inner = self.inner.allow_messages(input);
        self
    }
    /// <p>Boolean that controls whether the AppInstanceUserEndpoint is opted in to receive messages. <code>ALL</code> indicates the endpoint receives all messages. <code>NONE</code> indicates the endpoint receives no messages.</p>
    pub fn set_allow_messages(mut self, input: ::std::option::Option<crate::types::AllowMessages>) -> Self {
        self.inner = self.inner.set_allow_messages(input);
        self
    }
    /// <p>Boolean that controls whether the AppInstanceUserEndpoint is opted in to receive messages. <code>ALL</code> indicates the endpoint receives all messages. <code>NONE</code> indicates the endpoint receives no messages.</p>
    pub fn get_allow_messages(&self) -> &::std::option::Option<crate::types::AllowMessages> {
        self.inner.get_allow_messages()
    }
}
