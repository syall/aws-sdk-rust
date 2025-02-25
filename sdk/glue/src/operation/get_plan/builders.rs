// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_plan::_get_plan_output::GetPlanOutputBuilder;

pub use crate::operation::get_plan::_get_plan_input::GetPlanInputBuilder;

impl GetPlanInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_plan::GetPlanOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_plan::GetPlanError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_plan();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetPlan`.
///
/// <p>Gets code to perform a specified mapping.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetPlanFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_plan::builders::GetPlanInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::get_plan::GetPlanOutput, crate::operation::get_plan::GetPlanError>
    for GetPlanFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::get_plan::GetPlanOutput, crate::operation::get_plan::GetPlanError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetPlanFluentBuilder {
    /// Creates a new `GetPlan`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetPlan as a reference.
    pub fn as_input(&self) -> &crate::operation::get_plan::builders::GetPlanInputBuilder {
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
        crate::operation::get_plan::GetPlanOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_plan::GetPlanError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_plan::GetPlan::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_plan::GetPlan::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<crate::operation::get_plan::GetPlanOutput, crate::operation::get_plan::GetPlanError, Self>
    {
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
    /// Appends an item to `Mapping`.
    ///
    /// To override the contents of this collection use [`set_mapping`](Self::set_mapping).
    ///
    /// <p>The list of mappings from a source table to target tables.</p>
    pub fn mapping(mut self, input: crate::types::MappingEntry) -> Self {
        self.inner = self.inner.mapping(input);
        self
    }
    /// <p>The list of mappings from a source table to target tables.</p>
    pub fn set_mapping(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::MappingEntry>>) -> Self {
        self.inner = self.inner.set_mapping(input);
        self
    }
    /// <p>The list of mappings from a source table to target tables.</p>
    pub fn get_mapping(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::MappingEntry>> {
        self.inner.get_mapping()
    }
    /// <p>The source table.</p>
    pub fn source(mut self, input: crate::types::CatalogEntry) -> Self {
        self.inner = self.inner.source(input);
        self
    }
    /// <p>The source table.</p>
    pub fn set_source(mut self, input: ::std::option::Option<crate::types::CatalogEntry>) -> Self {
        self.inner = self.inner.set_source(input);
        self
    }
    /// <p>The source table.</p>
    pub fn get_source(&self) -> &::std::option::Option<crate::types::CatalogEntry> {
        self.inner.get_source()
    }
    /// Appends an item to `Sinks`.
    ///
    /// To override the contents of this collection use [`set_sinks`](Self::set_sinks).
    ///
    /// <p>The target tables.</p>
    pub fn sinks(mut self, input: crate::types::CatalogEntry) -> Self {
        self.inner = self.inner.sinks(input);
        self
    }
    /// <p>The target tables.</p>
    pub fn set_sinks(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::CatalogEntry>>) -> Self {
        self.inner = self.inner.set_sinks(input);
        self
    }
    /// <p>The target tables.</p>
    pub fn get_sinks(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CatalogEntry>> {
        self.inner.get_sinks()
    }
    /// <p>The parameters for the mapping.</p>
    pub fn location(mut self, input: crate::types::Location) -> Self {
        self.inner = self.inner.location(input);
        self
    }
    /// <p>The parameters for the mapping.</p>
    pub fn set_location(mut self, input: ::std::option::Option<crate::types::Location>) -> Self {
        self.inner = self.inner.set_location(input);
        self
    }
    /// <p>The parameters for the mapping.</p>
    pub fn get_location(&self) -> &::std::option::Option<crate::types::Location> {
        self.inner.get_location()
    }
    /// <p>The programming language of the code to perform the mapping.</p>
    pub fn language(mut self, input: crate::types::Language) -> Self {
        self.inner = self.inner.language(input);
        self
    }
    /// <p>The programming language of the code to perform the mapping.</p>
    pub fn set_language(mut self, input: ::std::option::Option<crate::types::Language>) -> Self {
        self.inner = self.inner.set_language(input);
        self
    }
    /// <p>The programming language of the code to perform the mapping.</p>
    pub fn get_language(&self) -> &::std::option::Option<crate::types::Language> {
        self.inner.get_language()
    }
    /// Adds a key-value pair to `AdditionalPlanOptionsMap`.
    ///
    /// To override the contents of this collection use [`set_additional_plan_options_map`](Self::set_additional_plan_options_map).
    ///
    /// <p>A map to hold additional optional key-value parameters.</p>
    /// <p>Currently, these key-value pairs are supported:</p>
    /// <ul>
    /// <li> <p> <code>inferSchema</code>&nbsp; — &nbsp;Specifies whether to set <code>inferSchema</code> to true or false for the default script generated by an Glue job. For example, to set <code>inferSchema</code> to true, pass the following key value pair:</p> <p> <code>--additional-plan-options-map '{"inferSchema":"true"}'</code> </p> </li>
    /// </ul>
    pub fn additional_plan_options_map(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.additional_plan_options_map(k.into(), v.into());
        self
    }
    /// <p>A map to hold additional optional key-value parameters.</p>
    /// <p>Currently, these key-value pairs are supported:</p>
    /// <ul>
    /// <li> <p> <code>inferSchema</code>&nbsp; — &nbsp;Specifies whether to set <code>inferSchema</code> to true or false for the default script generated by an Glue job. For example, to set <code>inferSchema</code> to true, pass the following key value pair:</p> <p> <code>--additional-plan-options-map '{"inferSchema":"true"}'</code> </p> </li>
    /// </ul>
    pub fn set_additional_plan_options_map(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_additional_plan_options_map(input);
        self
    }
    /// <p>A map to hold additional optional key-value parameters.</p>
    /// <p>Currently, these key-value pairs are supported:</p>
    /// <ul>
    /// <li> <p> <code>inferSchema</code>&nbsp; — &nbsp;Specifies whether to set <code>inferSchema</code> to true or false for the default script generated by an Glue job. For example, to set <code>inferSchema</code> to true, pass the following key value pair:</p> <p> <code>--additional-plan-options-map '{"inferSchema":"true"}'</code> </p> </li>
    /// </ul>
    pub fn get_additional_plan_options_map(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_additional_plan_options_map()
    }
}
