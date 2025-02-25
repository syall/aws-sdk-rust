// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_disassociate_client_device_from_core_device::_batch_disassociate_client_device_from_core_device_output::BatchDisassociateClientDeviceFromCoreDeviceOutputBuilder;

pub use crate::operation::batch_disassociate_client_device_from_core_device::_batch_disassociate_client_device_from_core_device_input::BatchDisassociateClientDeviceFromCoreDeviceInputBuilder;

impl BatchDisassociateClientDeviceFromCoreDeviceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDeviceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDeviceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.batch_disassociate_client_device_from_core_device();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchDisassociateClientDeviceFromCoreDevice`.
///
/// <p>Disassociates a list of client devices from a core device. After you disassociate a client device from a core device, the client device won't be able to use cloud discovery to retrieve the core device's connectivity information and certificates.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchDisassociateClientDeviceFromCoreDeviceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_disassociate_client_device_from_core_device::builders::BatchDisassociateClientDeviceFromCoreDeviceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDeviceOutput,
        crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDeviceError,
    > for BatchDisassociateClientDeviceFromCoreDeviceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDeviceOutput,
            crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDeviceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl BatchDisassociateClientDeviceFromCoreDeviceFluentBuilder {
    /// Creates a new `BatchDisassociateClientDeviceFromCoreDevice`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the BatchDisassociateClientDeviceFromCoreDevice as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::batch_disassociate_client_device_from_core_device::builders::BatchDisassociateClientDeviceFromCoreDeviceInputBuilder {
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
        crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDeviceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDeviceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDevice::operation_runtime_plugins(
                            self.handle.runtime_plugins.clone(),
                            &self.handle.conf,
                            self.config_override,
                        );
        crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDevice::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDeviceOutput,
        crate::operation::batch_disassociate_client_device_from_core_device::BatchDisassociateClientDeviceFromCoreDeviceError,
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
    /// Appends an item to `entries`.
    ///
    /// To override the contents of this collection use [`set_entries`](Self::set_entries).
    ///
    /// <p>The list of client devices to disassociate.</p>
    pub fn entries(mut self, input: crate::types::DisassociateClientDeviceFromCoreDeviceEntry) -> Self {
        self.inner = self.inner.entries(input);
        self
    }
    /// <p>The list of client devices to disassociate.</p>
    pub fn set_entries(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DisassociateClientDeviceFromCoreDeviceEntry>>) -> Self {
        self.inner = self.inner.set_entries(input);
        self
    }
    /// <p>The list of client devices to disassociate.</p>
    pub fn get_entries(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DisassociateClientDeviceFromCoreDeviceEntry>> {
        self.inner.get_entries()
    }
    /// <p>The name of the core device. This is also the name of the IoT thing.</p>
    pub fn core_device_thing_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.core_device_thing_name(input.into());
        self
    }
    /// <p>The name of the core device. This is also the name of the IoT thing.</p>
    pub fn set_core_device_thing_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_core_device_thing_name(input);
        self
    }
    /// <p>The name of the core device. This is also the name of the IoT thing.</p>
    pub fn get_core_device_thing_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_core_device_thing_name()
    }
}
