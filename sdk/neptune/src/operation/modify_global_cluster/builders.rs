// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_global_cluster::_modify_global_cluster_output::ModifyGlobalClusterOutputBuilder;

pub use crate::operation::modify_global_cluster::_modify_global_cluster_input::ModifyGlobalClusterInputBuilder;

impl ModifyGlobalClusterInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_global_cluster::ModifyGlobalClusterOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_global_cluster::ModifyGlobalClusterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_global_cluster();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyGlobalCluster`.
///
/// <p>Modify a setting for an Amazon Neptune global cluster. You can change one or more database configuration parameters by specifying these parameters and their new values in the request.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyGlobalClusterFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_global_cluster::builders::ModifyGlobalClusterInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_global_cluster::ModifyGlobalClusterOutput,
        crate::operation::modify_global_cluster::ModifyGlobalClusterError,
    > for ModifyGlobalClusterFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_global_cluster::ModifyGlobalClusterOutput,
            crate::operation::modify_global_cluster::ModifyGlobalClusterError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyGlobalClusterFluentBuilder {
    /// Creates a new `ModifyGlobalCluster`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyGlobalCluster as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_global_cluster::builders::ModifyGlobalClusterInputBuilder {
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
        crate::operation::modify_global_cluster::ModifyGlobalClusterOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_global_cluster::ModifyGlobalClusterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_global_cluster::ModifyGlobalCluster::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_global_cluster::ModifyGlobalCluster::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_global_cluster::ModifyGlobalClusterOutput,
        crate::operation::modify_global_cluster::ModifyGlobalClusterError,
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
    /// <p>The DB cluster identifier for the global cluster being modified. This parameter is not case-sensitive.</p>
    /// <p>Constraints: Must match the identifier of an existing global database cluster.</p>
    pub fn global_cluster_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.global_cluster_identifier(input.into());
        self
    }
    /// <p>The DB cluster identifier for the global cluster being modified. This parameter is not case-sensitive.</p>
    /// <p>Constraints: Must match the identifier of an existing global database cluster.</p>
    pub fn set_global_cluster_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_global_cluster_identifier(input);
        self
    }
    /// <p>The DB cluster identifier for the global cluster being modified. This parameter is not case-sensitive.</p>
    /// <p>Constraints: Must match the identifier of an existing global database cluster.</p>
    pub fn get_global_cluster_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_global_cluster_identifier()
    }
    /// <p>A new cluster identifier to assign to the global database. This value is stored as a lowercase string.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li>
    /// <li> <p>The first character must be a letter.</p> </li>
    /// <li> <p>Can't end with a hyphen or contain two consecutive hyphens</p> </li>
    /// </ul>
    /// <p>Example: <code>my-cluster2</code> </p>
    pub fn new_global_cluster_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.new_global_cluster_identifier(input.into());
        self
    }
    /// <p>A new cluster identifier to assign to the global database. This value is stored as a lowercase string.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li>
    /// <li> <p>The first character must be a letter.</p> </li>
    /// <li> <p>Can't end with a hyphen or contain two consecutive hyphens</p> </li>
    /// </ul>
    /// <p>Example: <code>my-cluster2</code> </p>
    pub fn set_new_global_cluster_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_new_global_cluster_identifier(input);
        self
    }
    /// <p>A new cluster identifier to assign to the global database. This value is stored as a lowercase string.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must contain from 1 to 63 letters, numbers, or hyphens.</p> </li>
    /// <li> <p>The first character must be a letter.</p> </li>
    /// <li> <p>Can't end with a hyphen or contain two consecutive hyphens</p> </li>
    /// </ul>
    /// <p>Example: <code>my-cluster2</code> </p>
    pub fn get_new_global_cluster_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_new_global_cluster_identifier()
    }
    /// <p>Indicates whether the global database has deletion protection enabled. The global database cannot be deleted when deletion protection is enabled.</p>
    pub fn deletion_protection(mut self, input: bool) -> Self {
        self.inner = self.inner.deletion_protection(input);
        self
    }
    /// <p>Indicates whether the global database has deletion protection enabled. The global database cannot be deleted when deletion protection is enabled.</p>
    pub fn set_deletion_protection(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_deletion_protection(input);
        self
    }
    /// <p>Indicates whether the global database has deletion protection enabled. The global database cannot be deleted when deletion protection is enabled.</p>
    pub fn get_deletion_protection(&self) -> &::std::option::Option<bool> {
        self.inner.get_deletion_protection()
    }
    /// <p>The version number of the database engine to which you want to upgrade. Changing this parameter will result in an outage. The change is applied during the next maintenance window unless <code>ApplyImmediately</code> is enabled.</p>
    /// <p>To list all of the available Neptune engine versions, use the following command:</p>
    pub fn engine_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.engine_version(input.into());
        self
    }
    /// <p>The version number of the database engine to which you want to upgrade. Changing this parameter will result in an outage. The change is applied during the next maintenance window unless <code>ApplyImmediately</code> is enabled.</p>
    /// <p>To list all of the available Neptune engine versions, use the following command:</p>
    pub fn set_engine_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_engine_version(input);
        self
    }
    /// <p>The version number of the database engine to which you want to upgrade. Changing this parameter will result in an outage. The change is applied during the next maintenance window unless <code>ApplyImmediately</code> is enabled.</p>
    /// <p>To list all of the available Neptune engine versions, use the following command:</p>
    pub fn get_engine_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_engine_version()
    }
    /// <p>A value that indicates whether major version upgrades are allowed.</p>
    /// <p>Constraints: You must allow major version upgrades if you specify a value for the <code>EngineVersion</code> parameter that is a different major version than the DB cluster's current version.</p>
    /// <p>If you upgrade the major version of a global database, the cluster and DB instance parameter groups are set to the default parameter groups for the new version, so you will need to apply any custom parameter groups after completing the upgrade.</p>
    pub fn allow_major_version_upgrade(mut self, input: bool) -> Self {
        self.inner = self.inner.allow_major_version_upgrade(input);
        self
    }
    /// <p>A value that indicates whether major version upgrades are allowed.</p>
    /// <p>Constraints: You must allow major version upgrades if you specify a value for the <code>EngineVersion</code> parameter that is a different major version than the DB cluster's current version.</p>
    /// <p>If you upgrade the major version of a global database, the cluster and DB instance parameter groups are set to the default parameter groups for the new version, so you will need to apply any custom parameter groups after completing the upgrade.</p>
    pub fn set_allow_major_version_upgrade(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_allow_major_version_upgrade(input);
        self
    }
    /// <p>A value that indicates whether major version upgrades are allowed.</p>
    /// <p>Constraints: You must allow major version upgrades if you specify a value for the <code>EngineVersion</code> parameter that is a different major version than the DB cluster's current version.</p>
    /// <p>If you upgrade the major version of a global database, the cluster and DB instance parameter groups are set to the default parameter groups for the new version, so you will need to apply any custom parameter groups after completing the upgrade.</p>
    pub fn get_allow_major_version_upgrade(&self) -> &::std::option::Option<bool> {
        self.inner.get_allow_major_version_upgrade()
    }
}
