// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_bucket_bundle::_update_bucket_bundle_output::UpdateBucketBundleOutputBuilder;

pub use crate::operation::update_bucket_bundle::_update_bucket_bundle_input::UpdateBucketBundleInputBuilder;

impl UpdateBucketBundleInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_bucket_bundle::UpdateBucketBundleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_bucket_bundle::UpdateBucketBundleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_bucket_bundle();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateBucketBundle`.
///
/// <p>Updates the bundle, or storage plan, of an existing Amazon Lightsail bucket.</p>
/// <p>A bucket bundle specifies the monthly cost, storage space, and data transfer quota for a bucket. You can update a bucket's bundle only one time within a monthly Amazon Web Services billing cycle. To determine if you can update a bucket's bundle, use the <a href="https://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetBuckets.html">GetBuckets</a> action. The <code>ableToUpdateBundle</code> parameter in the response will indicate whether you can currently update a bucket's bundle.</p>
/// <p>Update a bucket's bundle if it's consistently going over its storage space or data transfer quota, or if a bucket's usage is consistently in the lower range of its storage space or data transfer quota. Due to the unpredictable usage fluctuations that a bucket might experience, we strongly recommend that you update a bucket's bundle only as a long-term strategy, instead of as a short-term, monthly cost-cutting measure. Choose a bucket bundle that will provide the bucket with ample storage space and data transfer for a long time to come.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateBucketBundleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_bucket_bundle::builders::UpdateBucketBundleInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_bucket_bundle::UpdateBucketBundleOutput,
        crate::operation::update_bucket_bundle::UpdateBucketBundleError,
    > for UpdateBucketBundleFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_bucket_bundle::UpdateBucketBundleOutput,
            crate::operation::update_bucket_bundle::UpdateBucketBundleError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateBucketBundleFluentBuilder {
    /// Creates a new `UpdateBucketBundle`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateBucketBundle as a reference.
    pub fn as_input(&self) -> &crate::operation::update_bucket_bundle::builders::UpdateBucketBundleInputBuilder {
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
        crate::operation::update_bucket_bundle::UpdateBucketBundleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_bucket_bundle::UpdateBucketBundleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_bucket_bundle::UpdateBucketBundle::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_bucket_bundle::UpdateBucketBundle::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_bucket_bundle::UpdateBucketBundleOutput,
        crate::operation::update_bucket_bundle::UpdateBucketBundleError,
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
    /// <p>The name of the bucket for which to update the bundle.</p>
    pub fn bucket_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bucket_name(input.into());
        self
    }
    /// <p>The name of the bucket for which to update the bundle.</p>
    pub fn set_bucket_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bucket_name(input);
        self
    }
    /// <p>The name of the bucket for which to update the bundle.</p>
    pub fn get_bucket_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bucket_name()
    }
    /// <p>The ID of the new bundle to apply to the bucket.</p>
    /// <p>Use the <a href="https://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetBucketBundles.html">GetBucketBundles</a> action to get a list of bundle IDs that you can specify.</p>
    pub fn bundle_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bundle_id(input.into());
        self
    }
    /// <p>The ID of the new bundle to apply to the bucket.</p>
    /// <p>Use the <a href="https://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetBucketBundles.html">GetBucketBundles</a> action to get a list of bundle IDs that you can specify.</p>
    pub fn set_bundle_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bundle_id(input);
        self
    }
    /// <p>The ID of the new bundle to apply to the bucket.</p>
    /// <p>Use the <a href="https://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetBucketBundles.html">GetBucketBundles</a> action to get a list of bundle IDs that you can specify.</p>
    pub fn get_bundle_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bundle_id()
    }
}
