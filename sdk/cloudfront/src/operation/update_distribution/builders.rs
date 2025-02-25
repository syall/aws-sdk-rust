// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_distribution::_update_distribution_output::UpdateDistributionOutputBuilder;

pub use crate::operation::update_distribution::_update_distribution_input::UpdateDistributionInputBuilder;

impl UpdateDistributionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_distribution::UpdateDistributionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_distribution::UpdateDistributionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_distribution();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateDistribution`.
///
/// <p>Updates the configuration for a CloudFront distribution.</p>
/// <p>The update process includes getting the current distribution configuration, updating it to make your changes, and then submitting an <code>UpdateDistribution</code> request to make the updates.</p>
/// <p> <b>To update a web distribution using the CloudFront API</b> </p>
/// <ol>
/// <li> <p>Use <code>GetDistributionConfig</code> to get the current configuration, including the version identifier (<code>ETag</code>).</p> </li>
/// <li> <p>Update the distribution configuration that was returned in the response. Note the following important requirements and restrictions:</p>
/// <ul>
/// <li> <p>You must rename the <code>ETag</code> field to <code>IfMatch</code>, leaving the value unchanged. (Set the value of <code>IfMatch</code> to the value of <code>ETag</code>, then remove the <code>ETag</code> field.)</p> </li>
/// <li> <p>You can't change the value of <code>CallerReference</code>.</p> </li>
/// </ul> </li>
/// <li> <p>Submit an <code>UpdateDistribution</code> request, providing the distribution configuration. The new configuration replaces the existing configuration. The values that you specify in an <code>UpdateDistribution</code> request are not merged into your existing configuration. Make sure to include all fields: the ones that you modified and also the ones that you didn't.</p> </li>
/// </ol>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateDistributionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_distribution::builders::UpdateDistributionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_distribution::UpdateDistributionOutput,
        crate::operation::update_distribution::UpdateDistributionError,
    > for UpdateDistributionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_distribution::UpdateDistributionOutput,
            crate::operation::update_distribution::UpdateDistributionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateDistributionFluentBuilder {
    /// Creates a new `UpdateDistribution`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateDistribution as a reference.
    pub fn as_input(&self) -> &crate::operation::update_distribution::builders::UpdateDistributionInputBuilder {
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
        crate::operation::update_distribution::UpdateDistributionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_distribution::UpdateDistributionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_distribution::UpdateDistribution::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_distribution::UpdateDistribution::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_distribution::UpdateDistributionOutput,
        crate::operation::update_distribution::UpdateDistributionError,
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
    /// <p>The distribution's configuration information.</p>
    pub fn distribution_config(mut self, input: crate::types::DistributionConfig) -> Self {
        self.inner = self.inner.distribution_config(input);
        self
    }
    /// <p>The distribution's configuration information.</p>
    pub fn set_distribution_config(mut self, input: ::std::option::Option<crate::types::DistributionConfig>) -> Self {
        self.inner = self.inner.set_distribution_config(input);
        self
    }
    /// <p>The distribution's configuration information.</p>
    pub fn get_distribution_config(&self) -> &::std::option::Option<crate::types::DistributionConfig> {
        self.inner.get_distribution_config()
    }
    /// <p>The distribution's id.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The distribution's id.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The distribution's id.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the distribution's configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub fn if_match(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.if_match(input.into());
        self
    }
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the distribution's configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub fn set_if_match(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_if_match(input);
        self
    }
    /// <p>The value of the <code>ETag</code> header that you received when retrieving the distribution's configuration. For example: <code>E2QWRUHAPOMQZL</code>.</p>
    pub fn get_if_match(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_if_match()
    }
}
