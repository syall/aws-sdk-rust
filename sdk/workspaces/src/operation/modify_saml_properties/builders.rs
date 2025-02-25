// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_saml_properties::_modify_saml_properties_output::ModifySamlPropertiesOutputBuilder;

pub use crate::operation::modify_saml_properties::_modify_saml_properties_input::ModifySamlPropertiesInputBuilder;

impl ModifySamlPropertiesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_saml_properties::ModifySamlPropertiesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_saml_properties::ModifySamlPropertiesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_saml_properties();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifySamlProperties`.
///
/// <p>Modifies multiple properties related to SAML 2.0 authentication, including the enablement status, user access URL, and relay state parameter name that are used for configuring federation with an SAML 2.0 identity provider.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifySamlPropertiesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_saml_properties::builders::ModifySamlPropertiesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_saml_properties::ModifySamlPropertiesOutput,
        crate::operation::modify_saml_properties::ModifySamlPropertiesError,
    > for ModifySamlPropertiesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_saml_properties::ModifySamlPropertiesOutput,
            crate::operation::modify_saml_properties::ModifySamlPropertiesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifySamlPropertiesFluentBuilder {
    /// Creates a new `ModifySamlProperties`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifySamlProperties as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_saml_properties::builders::ModifySamlPropertiesInputBuilder {
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
        crate::operation::modify_saml_properties::ModifySamlPropertiesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_saml_properties::ModifySamlPropertiesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_saml_properties::ModifySamlProperties::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_saml_properties::ModifySamlProperties::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_saml_properties::ModifySamlPropertiesOutput,
        crate::operation::modify_saml_properties::ModifySamlPropertiesError,
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
    /// <p>The directory identifier for which you want to configure SAML properties.</p>
    pub fn resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_id(input.into());
        self
    }
    /// <p>The directory identifier for which you want to configure SAML properties.</p>
    pub fn set_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_id(input);
        self
    }
    /// <p>The directory identifier for which you want to configure SAML properties.</p>
    pub fn get_resource_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_id()
    }
    /// <p>The properties for configuring SAML 2.0 authentication.</p>
    pub fn saml_properties(mut self, input: crate::types::SamlProperties) -> Self {
        self.inner = self.inner.saml_properties(input);
        self
    }
    /// <p>The properties for configuring SAML 2.0 authentication.</p>
    pub fn set_saml_properties(mut self, input: ::std::option::Option<crate::types::SamlProperties>) -> Self {
        self.inner = self.inner.set_saml_properties(input);
        self
    }
    /// <p>The properties for configuring SAML 2.0 authentication.</p>
    pub fn get_saml_properties(&self) -> &::std::option::Option<crate::types::SamlProperties> {
        self.inner.get_saml_properties()
    }
    /// Appends an item to `PropertiesToDelete`.
    ///
    /// To override the contents of this collection use [`set_properties_to_delete`](Self::set_properties_to_delete).
    ///
    /// <p>The SAML properties to delete as part of your request.</p>
    /// <p>Specify one of the following options:</p>
    /// <ul>
    /// <li> <p> <code>SAML_PROPERTIES_USER_ACCESS_URL</code> to delete the user access URL.</p> </li>
    /// <li> <p> <code>SAML_PROPERTIES_RELAY_STATE_PARAMETER_NAME</code> to delete the relay state parameter name.</p> </li>
    /// </ul>
    pub fn properties_to_delete(mut self, input: crate::types::DeletableSamlProperty) -> Self {
        self.inner = self.inner.properties_to_delete(input);
        self
    }
    /// <p>The SAML properties to delete as part of your request.</p>
    /// <p>Specify one of the following options:</p>
    /// <ul>
    /// <li> <p> <code>SAML_PROPERTIES_USER_ACCESS_URL</code> to delete the user access URL.</p> </li>
    /// <li> <p> <code>SAML_PROPERTIES_RELAY_STATE_PARAMETER_NAME</code> to delete the relay state parameter name.</p> </li>
    /// </ul>
    pub fn set_properties_to_delete(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DeletableSamlProperty>>) -> Self {
        self.inner = self.inner.set_properties_to_delete(input);
        self
    }
    /// <p>The SAML properties to delete as part of your request.</p>
    /// <p>Specify one of the following options:</p>
    /// <ul>
    /// <li> <p> <code>SAML_PROPERTIES_USER_ACCESS_URL</code> to delete the user access URL.</p> </li>
    /// <li> <p> <code>SAML_PROPERTIES_RELAY_STATE_PARAMETER_NAME</code> to delete the relay state parameter name.</p> </li>
    /// </ul>
    pub fn get_properties_to_delete(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DeletableSamlProperty>> {
        self.inner.get_properties_to_delete()
    }
}
