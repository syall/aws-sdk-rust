// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::admin_update_user_attributes::_admin_update_user_attributes_output::AdminUpdateUserAttributesOutputBuilder;

pub use crate::operation::admin_update_user_attributes::_admin_update_user_attributes_input::AdminUpdateUserAttributesInputBuilder;

impl AdminUpdateUserAttributesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::admin_update_user_attributes::AdminUpdateUserAttributesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::admin_update_user_attributes::AdminUpdateUserAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.admin_update_user_attributes();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AdminUpdateUserAttributes`.
///
/// <note>
/// <p>This action might generate an SMS text message. Starting June 1, 2021, US telecom carriers require you to register an origination phone number before you can send SMS messages to US phone numbers. If you use SMS text messages in Amazon Cognito, you must register a phone number with <a href="https://console.aws.amazon.com/pinpoint/home/">Amazon Pinpoint</a>. Amazon Cognito uses the registered number automatically. Otherwise, Amazon Cognito users who must receive SMS messages might not be able to sign up, activate their accounts, or sign in.</p>
/// <p>If you have never used SMS text messages with Amazon Cognito or any other Amazon Web Service, Amazon Simple Notification Service might place your account in the SMS sandbox. In <i> <a href="https://docs.aws.amazon.com/sns/latest/dg/sns-sms-sandbox.html">sandbox mode</a> </i>, you can send messages only to verified phone numbers. After you test your app while in the sandbox environment, you can move out of the sandbox and into production. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-sms-settings.html"> SMS message settings for Amazon Cognito user pools</a> in the <i>Amazon Cognito Developer Guide</i>.</p>
/// </note>
/// <p>Updates the specified user's attributes, including developer attributes, as an administrator. Works on any user. To delete an attribute from your user, submit the attribute in your API request with a blank value.</p>
/// <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
/// <p>In addition to updating user attributes, this API can also be used to mark phone and email as verified.</p> <note>
/// <p>Amazon Cognito evaluates Identity and Access Management (IAM) policies in requests for this API operation. For this operation, you must use IAM credentials to authorize requests, and you must grant yourself the corresponding IAM permission in a policy.</p>
/// <p class="title"> <b>Learn more</b> </p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-signing.html">Signing Amazon Web Services API Requests</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pools-API-operations.html">Using the Amazon Cognito user pools API and user pool endpoints</a> </p> </li>
/// </ul>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AdminUpdateUserAttributesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::admin_update_user_attributes::builders::AdminUpdateUserAttributesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::admin_update_user_attributes::AdminUpdateUserAttributesOutput,
        crate::operation::admin_update_user_attributes::AdminUpdateUserAttributesError,
    > for AdminUpdateUserAttributesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::admin_update_user_attributes::AdminUpdateUserAttributesOutput,
            crate::operation::admin_update_user_attributes::AdminUpdateUserAttributesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AdminUpdateUserAttributesFluentBuilder {
    /// Creates a new `AdminUpdateUserAttributes`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AdminUpdateUserAttributes as a reference.
    pub fn as_input(&self) -> &crate::operation::admin_update_user_attributes::builders::AdminUpdateUserAttributesInputBuilder {
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
        crate::operation::admin_update_user_attributes::AdminUpdateUserAttributesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::admin_update_user_attributes::AdminUpdateUserAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::admin_update_user_attributes::AdminUpdateUserAttributes::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::admin_update_user_attributes::AdminUpdateUserAttributes::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::admin_update_user_attributes::AdminUpdateUserAttributesOutput,
        crate::operation::admin_update_user_attributes::AdminUpdateUserAttributesError,
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
    /// <p>The user pool ID for the user pool where you want to update user attributes.</p>
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_pool_id(input.into());
        self
    }
    /// <p>The user pool ID for the user pool where you want to update user attributes.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_pool_id(input);
        self
    }
    /// <p>The user pool ID for the user pool where you want to update user attributes.</p>
    pub fn get_user_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_pool_id()
    }
    /// <p>The user name of the user for whom you want to update user attributes.</p>
    pub fn username(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.username(input.into());
        self
    }
    /// <p>The user name of the user for whom you want to update user attributes.</p>
    pub fn set_username(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_username(input);
        self
    }
    /// <p>The user name of the user for whom you want to update user attributes.</p>
    pub fn get_username(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_username()
    }
    /// Appends an item to `UserAttributes`.
    ///
    /// To override the contents of this collection use [`set_user_attributes`](Self::set_user_attributes).
    ///
    /// <p>An array of name-value pairs representing user attributes.</p>
    /// <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    /// <p>If your user pool requires verification before Amazon Cognito updates an attribute value that you specify in this request, Amazon Cognito doesn’t immediately update the value of that attribute. After your user receives and responds to a verification message to verify the new value, Amazon Cognito updates the attribute value. Your user can sign in and receive messages with the original attribute value until they verify the new value.</p>
    /// <p>To update the value of an attribute that requires verification in the same API request, include the <code>email_verified</code> or <code>phone_number_verified</code> attribute, with a value of <code>true</code>. If you set the <code>email_verified</code> or <code>phone_number_verified</code> value for an <code>email</code> or <code>phone_number</code> attribute that requires verification to <code>true</code>, Amazon Cognito doesn’t send a verification message to your user.</p>
    pub fn user_attributes(mut self, input: crate::types::AttributeType) -> Self {
        self.inner = self.inner.user_attributes(input);
        self
    }
    /// <p>An array of name-value pairs representing user attributes.</p>
    /// <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    /// <p>If your user pool requires verification before Amazon Cognito updates an attribute value that you specify in this request, Amazon Cognito doesn’t immediately update the value of that attribute. After your user receives and responds to a verification message to verify the new value, Amazon Cognito updates the attribute value. Your user can sign in and receive messages with the original attribute value until they verify the new value.</p>
    /// <p>To update the value of an attribute that requires verification in the same API request, include the <code>email_verified</code> or <code>phone_number_verified</code> attribute, with a value of <code>true</code>. If you set the <code>email_verified</code> or <code>phone_number_verified</code> value for an <code>email</code> or <code>phone_number</code> attribute that requires verification to <code>true</code>, Amazon Cognito doesn’t send a verification message to your user.</p>
    pub fn set_user_attributes(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AttributeType>>) -> Self {
        self.inner = self.inner.set_user_attributes(input);
        self
    }
    /// <p>An array of name-value pairs representing user attributes.</p>
    /// <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    /// <p>If your user pool requires verification before Amazon Cognito updates an attribute value that you specify in this request, Amazon Cognito doesn’t immediately update the value of that attribute. After your user receives and responds to a verification message to verify the new value, Amazon Cognito updates the attribute value. Your user can sign in and receive messages with the original attribute value until they verify the new value.</p>
    /// <p>To update the value of an attribute that requires verification in the same API request, include the <code>email_verified</code> or <code>phone_number_verified</code> attribute, with a value of <code>true</code>. If you set the <code>email_verified</code> or <code>phone_number_verified</code> value for an <code>email</code> or <code>phone_number</code> attribute that requires verification to <code>true</code>, Amazon Cognito doesn’t send a verification message to your user.</p>
    pub fn get_user_attributes(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AttributeType>> {
        self.inner.get_user_attributes()
    }
    /// Adds a key-value pair to `ClientMetadata`.
    ///
    /// To override the contents of this collection use [`set_client_metadata`](Self::set_client_metadata).
    ///
    /// <p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the AdminUpdateUserAttributes API action, Amazon Cognito invokes the function that is assigned to the <i>custom message</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your AdminUpdateUserAttributes request. In your function code in Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Customizing user pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note>
    /// <p>When you use the ClientMetadata parameter, remember that Amazon Cognito won't do the following:</p>
    /// <ul>
    /// <li> <p>Store the ClientMetadata value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the ClientMetadata parameter serves no purpose.</p> </li>
    /// <li> <p>Validate the ClientMetadata value.</p> </li>
    /// <li> <p>Encrypt the ClientMetadata value. Don't use Amazon Cognito to provide sensitive information.</p> </li>
    /// </ul>
    /// </note>
    pub fn client_metadata(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.client_metadata(k.into(), v.into());
        self
    }
    /// <p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the AdminUpdateUserAttributes API action, Amazon Cognito invokes the function that is assigned to the <i>custom message</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your AdminUpdateUserAttributes request. In your function code in Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Customizing user pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note>
    /// <p>When you use the ClientMetadata parameter, remember that Amazon Cognito won't do the following:</p>
    /// <ul>
    /// <li> <p>Store the ClientMetadata value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the ClientMetadata parameter serves no purpose.</p> </li>
    /// <li> <p>Validate the ClientMetadata value.</p> </li>
    /// <li> <p>Encrypt the ClientMetadata value. Don't use Amazon Cognito to provide sensitive information.</p> </li>
    /// </ul>
    /// </note>
    pub fn set_client_metadata(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_client_metadata(input);
        self
    }
    /// <p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the AdminUpdateUserAttributes API action, Amazon Cognito invokes the function that is assigned to the <i>custom message</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your AdminUpdateUserAttributes request. In your function code in Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Customizing user pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note>
    /// <p>When you use the ClientMetadata parameter, remember that Amazon Cognito won't do the following:</p>
    /// <ul>
    /// <li> <p>Store the ClientMetadata value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the ClientMetadata parameter serves no purpose.</p> </li>
    /// <li> <p>Validate the ClientMetadata value.</p> </li>
    /// <li> <p>Encrypt the ClientMetadata value. Don't use Amazon Cognito to provide sensitive information.</p> </li>
    /// </ul>
    /// </note>
    pub fn get_client_metadata(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_client_metadata()
    }
}
