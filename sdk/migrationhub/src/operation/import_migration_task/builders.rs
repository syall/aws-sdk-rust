// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::import_migration_task::_import_migration_task_output::ImportMigrationTaskOutputBuilder;

pub use crate::operation::import_migration_task::_import_migration_task_input::ImportMigrationTaskInputBuilder;

impl ImportMigrationTaskInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::import_migration_task::ImportMigrationTaskOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::import_migration_task::ImportMigrationTaskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.import_migration_task();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ImportMigrationTask`.
///
/// <p>Registers a new migration task which represents a server, database, etc., being migrated to AWS by a migration tool.</p>
/// <p>This API is a prerequisite to calling the <code>NotifyMigrationTaskState</code> API as the migration tool must first register the migration task with Migration Hub.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ImportMigrationTaskFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::import_migration_task::builders::ImportMigrationTaskInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::import_migration_task::ImportMigrationTaskOutput,
        crate::operation::import_migration_task::ImportMigrationTaskError,
    > for ImportMigrationTaskFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::import_migration_task::ImportMigrationTaskOutput,
            crate::operation::import_migration_task::ImportMigrationTaskError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ImportMigrationTaskFluentBuilder {
    /// Creates a new `ImportMigrationTask`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ImportMigrationTask as a reference.
    pub fn as_input(&self) -> &crate::operation::import_migration_task::builders::ImportMigrationTaskInputBuilder {
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
        crate::operation::import_migration_task::ImportMigrationTaskOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::import_migration_task::ImportMigrationTaskError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::import_migration_task::ImportMigrationTask::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::import_migration_task::ImportMigrationTask::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::import_migration_task::ImportMigrationTaskOutput,
        crate::operation::import_migration_task::ImportMigrationTaskError,
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
    /// <p>The name of the ProgressUpdateStream. &gt;</p>
    pub fn progress_update_stream(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.progress_update_stream(input.into());
        self
    }
    /// <p>The name of the ProgressUpdateStream. &gt;</p>
    pub fn set_progress_update_stream(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_progress_update_stream(input);
        self
    }
    /// <p>The name of the ProgressUpdateStream. &gt;</p>
    pub fn get_progress_update_stream(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_progress_update_stream()
    }
    /// <p>Unique identifier that references the migration task. <i>Do not store personal data in this field.</i> </p>
    pub fn migration_task_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.migration_task_name(input.into());
        self
    }
    /// <p>Unique identifier that references the migration task. <i>Do not store personal data in this field.</i> </p>
    pub fn set_migration_task_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_migration_task_name(input);
        self
    }
    /// <p>Unique identifier that references the migration task. <i>Do not store personal data in this field.</i> </p>
    pub fn get_migration_task_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_migration_task_name()
    }
    /// <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
