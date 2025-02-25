// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_network_profile::_update_network_profile_output::UpdateNetworkProfileOutputBuilder;

pub use crate::operation::update_network_profile::_update_network_profile_input::UpdateNetworkProfileInputBuilder;

impl UpdateNetworkProfileInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_network_profile::UpdateNetworkProfileOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_network_profile::UpdateNetworkProfileError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_network_profile();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateNetworkProfile`.
///
/// <p>Updates a network profile by the network profile ARN.</p>
#[deprecated(note = "Alexa For Business is no longer supported")]
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateNetworkProfileFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_network_profile::builders::UpdateNetworkProfileInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_network_profile::UpdateNetworkProfileOutput,
        crate::operation::update_network_profile::UpdateNetworkProfileError,
    > for UpdateNetworkProfileFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_network_profile::UpdateNetworkProfileOutput,
            crate::operation::update_network_profile::UpdateNetworkProfileError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateNetworkProfileFluentBuilder {
    /// Creates a new `UpdateNetworkProfile`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateNetworkProfile as a reference.
    pub fn as_input(&self) -> &crate::operation::update_network_profile::builders::UpdateNetworkProfileInputBuilder {
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
        crate::operation::update_network_profile::UpdateNetworkProfileOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_network_profile::UpdateNetworkProfileError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_network_profile::UpdateNetworkProfile::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_network_profile::UpdateNetworkProfile::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_network_profile::UpdateNetworkProfileOutput,
        crate::operation::update_network_profile::UpdateNetworkProfileError,
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
    /// <p>The ARN of the network profile associated with a device.</p>
    pub fn network_profile_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.network_profile_arn(input.into());
        self
    }
    /// <p>The ARN of the network profile associated with a device.</p>
    pub fn set_network_profile_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_network_profile_arn(input);
        self
    }
    /// <p>The ARN of the network profile associated with a device.</p>
    pub fn get_network_profile_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_network_profile_arn()
    }
    /// <p>The name of the network profile associated with a device.</p>
    pub fn network_profile_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.network_profile_name(input.into());
        self
    }
    /// <p>The name of the network profile associated with a device.</p>
    pub fn set_network_profile_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_network_profile_name(input);
        self
    }
    /// <p>The name of the network profile associated with a device.</p>
    pub fn get_network_profile_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_network_profile_name()
    }
    /// <p>Detailed information about a device's network profile.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>Detailed information about a device's network profile.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>Detailed information about a device's network profile.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The current password of the Wi-Fi network.</p>
    pub fn current_password(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.current_password(input.into());
        self
    }
    /// <p>The current password of the Wi-Fi network.</p>
    pub fn set_current_password(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_current_password(input);
        self
    }
    /// <p>The current password of the Wi-Fi network.</p>
    pub fn get_current_password(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_current_password()
    }
    /// <p>The next, or subsequent, password of the Wi-Fi network. This password is asynchronously transmitted to the device and is used when the password of the network changes to NextPassword. </p>
    pub fn next_password(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_password(input.into());
        self
    }
    /// <p>The next, or subsequent, password of the Wi-Fi network. This password is asynchronously transmitted to the device and is used when the password of the network changes to NextPassword. </p>
    pub fn set_next_password(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_password(input);
        self
    }
    /// <p>The next, or subsequent, password of the Wi-Fi network. This password is asynchronously transmitted to the device and is used when the password of the network changes to NextPassword. </p>
    pub fn get_next_password(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_password()
    }
    /// <p>The ARN of the Private Certificate Authority (PCA) created in AWS Certificate Manager (ACM). This is used to issue certificates to the devices. </p>
    pub fn certificate_authority_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.certificate_authority_arn(input.into());
        self
    }
    /// <p>The ARN of the Private Certificate Authority (PCA) created in AWS Certificate Manager (ACM). This is used to issue certificates to the devices. </p>
    pub fn set_certificate_authority_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_certificate_authority_arn(input);
        self
    }
    /// <p>The ARN of the Private Certificate Authority (PCA) created in AWS Certificate Manager (ACM). This is used to issue certificates to the devices. </p>
    pub fn get_certificate_authority_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_certificate_authority_arn()
    }
    /// Appends an item to `TrustAnchors`.
    ///
    /// To override the contents of this collection use [`set_trust_anchors`](Self::set_trust_anchors).
    ///
    /// <p>The root certificate(s) of your authentication server that will be installed on your devices and used to trust your authentication server during EAP negotiation. </p>
    pub fn trust_anchors(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.trust_anchors(input.into());
        self
    }
    /// <p>The root certificate(s) of your authentication server that will be installed on your devices and used to trust your authentication server during EAP negotiation. </p>
    pub fn set_trust_anchors(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_trust_anchors(input);
        self
    }
    /// <p>The root certificate(s) of your authentication server that will be installed on your devices and used to trust your authentication server during EAP negotiation. </p>
    pub fn get_trust_anchors(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_trust_anchors()
    }
}
