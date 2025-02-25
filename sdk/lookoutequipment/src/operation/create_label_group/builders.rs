// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_label_group::_create_label_group_output::CreateLabelGroupOutputBuilder;

pub use crate::operation::create_label_group::_create_label_group_input::CreateLabelGroupInputBuilder;

impl CreateLabelGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_label_group::CreateLabelGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_label_group::CreateLabelGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_label_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateLabelGroup`.
///
/// <p> Creates a group of labels. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateLabelGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_label_group::builders::CreateLabelGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_label_group::CreateLabelGroupOutput,
        crate::operation::create_label_group::CreateLabelGroupError,
    > for CreateLabelGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_label_group::CreateLabelGroupOutput,
            crate::operation::create_label_group::CreateLabelGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateLabelGroupFluentBuilder {
    /// Creates a new `CreateLabelGroup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateLabelGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::create_label_group::builders::CreateLabelGroupInputBuilder {
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
        crate::operation::create_label_group::CreateLabelGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_label_group::CreateLabelGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_label_group::CreateLabelGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_label_group::CreateLabelGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_label_group::CreateLabelGroupOutput,
        crate::operation::create_label_group::CreateLabelGroupError,
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
    /// <p> Names a group of labels.</p>
    /// <p>Data in this field will be retained for service usage. Follow best practices for the security of your data. </p>
    pub fn label_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.label_group_name(input.into());
        self
    }
    /// <p> Names a group of labels.</p>
    /// <p>Data in this field will be retained for service usage. Follow best practices for the security of your data. </p>
    pub fn set_label_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_label_group_name(input);
        self
    }
    /// <p> Names a group of labels.</p>
    /// <p>Data in this field will be retained for service usage. Follow best practices for the security of your data. </p>
    pub fn get_label_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_label_group_name()
    }
    /// Appends an item to `FaultCodes`.
    ///
    /// To override the contents of this collection use [`set_fault_codes`](Self::set_fault_codes).
    ///
    /// <p> The acceptable fault codes (indicating the type of anomaly associated with the label) that can be used with this label group.</p>
    /// <p>Data in this field will be retained for service usage. Follow best practices for the security of your data.</p>
    pub fn fault_codes(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.fault_codes(input.into());
        self
    }
    /// <p> The acceptable fault codes (indicating the type of anomaly associated with the label) that can be used with this label group.</p>
    /// <p>Data in this field will be retained for service usage. Follow best practices for the security of your data.</p>
    pub fn set_fault_codes(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_fault_codes(input);
        self
    }
    /// <p> The acceptable fault codes (indicating the type of anomaly associated with the label) that can be used with this label group.</p>
    /// <p>Data in this field will be retained for service usage. Follow best practices for the security of your data.</p>
    pub fn get_fault_codes(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_fault_codes()
    }
    /// <p> A unique identifier for the request to create a label group. If you do not set the client request token, Lookout for Equipment generates one. </p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p> A unique identifier for the request to create a label group. If you do not set the client request token, Lookout for Equipment generates one. </p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p> A unique identifier for the request to create a label group. If you do not set the client request token, Lookout for Equipment generates one. </p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p> Tags that provide metadata about the label group you are creating. </p>
    /// <p>Data in this field will be retained for service usage. Follow best practices for the security of your data.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p> Tags that provide metadata about the label group you are creating. </p>
    /// <p>Data in this field will be retained for service usage. Follow best practices for the security of your data.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p> Tags that provide metadata about the label group you are creating. </p>
    /// <p>Data in this field will be retained for service usage. Follow best practices for the security of your data.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
