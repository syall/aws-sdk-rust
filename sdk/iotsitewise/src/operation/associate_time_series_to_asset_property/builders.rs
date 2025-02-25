// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_time_series_to_asset_property::_associate_time_series_to_asset_property_output::AssociateTimeSeriesToAssetPropertyOutputBuilder;

pub use crate::operation::associate_time_series_to_asset_property::_associate_time_series_to_asset_property_input::AssociateTimeSeriesToAssetPropertyInputBuilder;

impl AssociateTimeSeriesToAssetPropertyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::associate_time_series_to_asset_property::AssociateTimeSeriesToAssetPropertyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_time_series_to_asset_property::AssociateTimeSeriesToAssetPropertyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.associate_time_series_to_asset_property();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AssociateTimeSeriesToAssetProperty`.
///
/// <p>Associates a time series (data stream) with an asset property.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociateTimeSeriesToAssetPropertyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_time_series_to_asset_property::builders::AssociateTimeSeriesToAssetPropertyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::associate_time_series_to_asset_property::AssociateTimeSeriesToAssetPropertyOutput,
        crate::operation::associate_time_series_to_asset_property::AssociateTimeSeriesToAssetPropertyError,
    > for AssociateTimeSeriesToAssetPropertyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::associate_time_series_to_asset_property::AssociateTimeSeriesToAssetPropertyOutput,
            crate::operation::associate_time_series_to_asset_property::AssociateTimeSeriesToAssetPropertyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AssociateTimeSeriesToAssetPropertyFluentBuilder {
    /// Creates a new `AssociateTimeSeriesToAssetProperty`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AssociateTimeSeriesToAssetProperty as a reference.
    pub fn as_input(&self) -> &crate::operation::associate_time_series_to_asset_property::builders::AssociateTimeSeriesToAssetPropertyInputBuilder {
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
        crate::operation::associate_time_series_to_asset_property::AssociateTimeSeriesToAssetPropertyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_time_series_to_asset_property::AssociateTimeSeriesToAssetPropertyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::associate_time_series_to_asset_property::AssociateTimeSeriesToAssetProperty::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::associate_time_series_to_asset_property::AssociateTimeSeriesToAssetProperty::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::associate_time_series_to_asset_property::AssociateTimeSeriesToAssetPropertyOutput,
        crate::operation::associate_time_series_to_asset_property::AssociateTimeSeriesToAssetPropertyError,
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
    /// <p>The alias that identifies the time series.</p>
    pub fn alias(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.alias(input.into());
        self
    }
    /// <p>The alias that identifies the time series.</p>
    pub fn set_alias(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_alias(input);
        self
    }
    /// <p>The alias that identifies the time series.</p>
    pub fn get_alias(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_alias()
    }
    /// <p>The ID of the asset in which the asset property was created.</p>
    pub fn asset_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.asset_id(input.into());
        self
    }
    /// <p>The ID of the asset in which the asset property was created.</p>
    pub fn set_asset_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_asset_id(input);
        self
    }
    /// <p>The ID of the asset in which the asset property was created.</p>
    pub fn get_asset_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_asset_id()
    }
    /// <p>The ID of the asset property.</p>
    pub fn property_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.property_id(input.into());
        self
    }
    /// <p>The ID of the asset property.</p>
    pub fn set_property_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_property_id(input);
        self
    }
    /// <p>The ID of the asset property.</p>
    pub fn get_property_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_property_id()
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
