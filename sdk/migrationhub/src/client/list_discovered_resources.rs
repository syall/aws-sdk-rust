// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListDiscoveredResources`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`progress_update_stream(impl Into<String>)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::progress_update_stream) / [`set_progress_update_stream(Option<String>)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::set_progress_update_stream):<br>required: **true**<br><p>The name of the ProgressUpdateStream.</p><br>
    ///   - [`migration_task_name(impl Into<String>)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::migration_task_name) / [`set_migration_task_name(Option<String>)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::set_migration_task_name):<br>required: **true**<br><p>The name of the MigrationTask. <i>Do not store personal data in this field.</i> </p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::set_next_token):<br>required: **false**<br><p>If a <code>NextToken</code> was returned by a previous call, there are more results available. To retrieve the next page of results, make the call again using the returned token in <code>NextToken</code>.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results returned per page.</p><br>
    /// - On success, responds with [`ListDiscoveredResourcesOutput`](crate::operation::list_discovered_resources::ListDiscoveredResourcesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_discovered_resources::ListDiscoveredResourcesOutput::next_token): <p>If there are more discovered resources than the max result, return the next token to be passed to the next call as a bookmark of where to start from.</p>
    ///   - [`discovered_resource_list(Option<Vec::<DiscoveredResource>>)`](crate::operation::list_discovered_resources::ListDiscoveredResourcesOutput::discovered_resource_list): <p>Returned list of discovered resources associated with the given MigrationTask.</p>
    /// - On failure, responds with [`SdkError<ListDiscoveredResourcesError>`](crate::operation::list_discovered_resources::ListDiscoveredResourcesError)
    pub fn list_discovered_resources(&self) -> crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder {
        crate::operation::list_discovered_resources::builders::ListDiscoveredResourcesFluentBuilder::new(self.handle.clone())
    }
}
