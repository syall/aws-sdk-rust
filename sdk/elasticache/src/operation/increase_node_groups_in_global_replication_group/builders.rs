// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::increase_node_groups_in_global_replication_group::_increase_node_groups_in_global_replication_group_output::IncreaseNodeGroupsInGlobalReplicationGroupOutputBuilder;

pub use crate::operation::increase_node_groups_in_global_replication_group::_increase_node_groups_in_global_replication_group_input::IncreaseNodeGroupsInGlobalReplicationGroupInputBuilder;

impl IncreaseNodeGroupsInGlobalReplicationGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::increase_node_groups_in_global_replication_group::IncreaseNodeGroupsInGlobalReplicationGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::increase_node_groups_in_global_replication_group::IncreaseNodeGroupsInGlobalReplicationGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.increase_node_groups_in_global_replication_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `IncreaseNodeGroupsInGlobalReplicationGroup`.
///
/// <p>Increase the number of node groups in the Global datastore</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct IncreaseNodeGroupsInGlobalReplicationGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::increase_node_groups_in_global_replication_group::builders::IncreaseNodeGroupsInGlobalReplicationGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::increase_node_groups_in_global_replication_group::IncreaseNodeGroupsInGlobalReplicationGroupOutput,
        crate::operation::increase_node_groups_in_global_replication_group::IncreaseNodeGroupsInGlobalReplicationGroupError,
    > for IncreaseNodeGroupsInGlobalReplicationGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::increase_node_groups_in_global_replication_group::IncreaseNodeGroupsInGlobalReplicationGroupOutput,
            crate::operation::increase_node_groups_in_global_replication_group::IncreaseNodeGroupsInGlobalReplicationGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl IncreaseNodeGroupsInGlobalReplicationGroupFluentBuilder {
    /// Creates a new `IncreaseNodeGroupsInGlobalReplicationGroup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the IncreaseNodeGroupsInGlobalReplicationGroup as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::increase_node_groups_in_global_replication_group::builders::IncreaseNodeGroupsInGlobalReplicationGroupInputBuilder {
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
        crate::operation::increase_node_groups_in_global_replication_group::IncreaseNodeGroupsInGlobalReplicationGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::increase_node_groups_in_global_replication_group::IncreaseNodeGroupsInGlobalReplicationGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::increase_node_groups_in_global_replication_group::IncreaseNodeGroupsInGlobalReplicationGroup::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::increase_node_groups_in_global_replication_group::IncreaseNodeGroupsInGlobalReplicationGroup::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::increase_node_groups_in_global_replication_group::IncreaseNodeGroupsInGlobalReplicationGroupOutput,
        crate::operation::increase_node_groups_in_global_replication_group::IncreaseNodeGroupsInGlobalReplicationGroupError,
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
    /// <p>The name of the Global datastore</p>
    pub fn global_replication_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.global_replication_group_id(input.into());
        self
    }
    /// <p>The name of the Global datastore</p>
    pub fn set_global_replication_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_global_replication_group_id(input);
        self
    }
    /// <p>The name of the Global datastore</p>
    pub fn get_global_replication_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_global_replication_group_id()
    }
    /// <p>Total number of node groups you want</p>
    pub fn node_group_count(mut self, input: i32) -> Self {
        self.inner = self.inner.node_group_count(input);
        self
    }
    /// <p>Total number of node groups you want</p>
    pub fn set_node_group_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_node_group_count(input);
        self
    }
    /// <p>Total number of node groups you want</p>
    pub fn get_node_group_count(&self) -> &::std::option::Option<i32> {
        self.inner.get_node_group_count()
    }
    /// Appends an item to `RegionalConfigurations`.
    ///
    /// To override the contents of this collection use [`set_regional_configurations`](Self::set_regional_configurations).
    ///
    /// <p>Describes the replication group IDs, the Amazon regions where they are stored and the shard configuration for each that comprise the Global datastore</p>
    pub fn regional_configurations(mut self, input: crate::types::RegionalConfiguration) -> Self {
        self.inner = self.inner.regional_configurations(input);
        self
    }
    /// <p>Describes the replication group IDs, the Amazon regions where they are stored and the shard configuration for each that comprise the Global datastore</p>
    pub fn set_regional_configurations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::RegionalConfiguration>>) -> Self {
        self.inner = self.inner.set_regional_configurations(input);
        self
    }
    /// <p>Describes the replication group IDs, the Amazon regions where they are stored and the shard configuration for each that comprise the Global datastore</p>
    pub fn get_regional_configurations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::RegionalConfiguration>> {
        self.inner.get_regional_configurations()
    }
    /// <p>Indicates that the process begins immediately. At present, the only permitted value for this parameter is true.</p>
    pub fn apply_immediately(mut self, input: bool) -> Self {
        self.inner = self.inner.apply_immediately(input);
        self
    }
    /// <p>Indicates that the process begins immediately. At present, the only permitted value for this parameter is true.</p>
    pub fn set_apply_immediately(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_apply_immediately(input);
        self
    }
    /// <p>Indicates that the process begins immediately. At present, the only permitted value for this parameter is true.</p>
    pub fn get_apply_immediately(&self) -> &::std::option::Option<bool> {
        self.inner.get_apply_immediately()
    }
}
