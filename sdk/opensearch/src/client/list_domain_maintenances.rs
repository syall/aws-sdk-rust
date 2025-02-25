// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListDomainMaintenances`](crate::operation::list_domain_maintenances::builders::ListDomainMaintenancesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_domain_maintenances::builders::ListDomainMaintenancesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::operation::list_domain_maintenances::builders::ListDomainMaintenancesFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::list_domain_maintenances::builders::ListDomainMaintenancesFluentBuilder::set_domain_name):<br>required: **true**<br><p>The name of the domain.</p><br>
    ///   - [`action(MaintenanceType)`](crate::operation::list_domain_maintenances::builders::ListDomainMaintenancesFluentBuilder::action) / [`set_action(Option<MaintenanceType>)`](crate::operation::list_domain_maintenances::builders::ListDomainMaintenancesFluentBuilder::set_action):<br>required: **false**<br><p>The name of the action.</p><br>
    ///   - [`status(MaintenanceStatus)`](crate::operation::list_domain_maintenances::builders::ListDomainMaintenancesFluentBuilder::status) / [`set_status(Option<MaintenanceStatus>)`](crate::operation::list_domain_maintenances::builders::ListDomainMaintenancesFluentBuilder::set_status):<br>required: **false**<br><p>The status of the action.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_domain_maintenances::builders::ListDomainMaintenancesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_domain_maintenances::builders::ListDomainMaintenancesFluentBuilder::set_max_results):<br>required: **false**<br><p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_domain_maintenances::builders::ListDomainMaintenancesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_domain_maintenances::builders::ListDomainMaintenancesFluentBuilder::set_next_token):<br>required: **false**<br><p>If your initial <code>ListDomainMaintenances</code> operation returns a <code>nextToken</code>, include the returned <code>nextToken</code> in subsequent <code>ListDomainMaintenances</code> operations, which returns results in the next page.</p><br>
    /// - On success, responds with [`ListDomainMaintenancesOutput`](crate::operation::list_domain_maintenances::ListDomainMaintenancesOutput) with field(s):
    ///   - [`domain_maintenances(Option<Vec::<DomainMaintenanceDetails>>)`](crate::operation::list_domain_maintenances::ListDomainMaintenancesOutput::domain_maintenances): <p>A list of the submitted maintenance actions.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_domain_maintenances::ListDomainMaintenancesOutput::next_token): <p>When <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page.</p>
    /// - On failure, responds with [`SdkError<ListDomainMaintenancesError>`](crate::operation::list_domain_maintenances::ListDomainMaintenancesError)
    pub fn list_domain_maintenances(&self) -> crate::operation::list_domain_maintenances::builders::ListDomainMaintenancesFluentBuilder {
        crate::operation::list_domain_maintenances::builders::ListDomainMaintenancesFluentBuilder::new(self.handle.clone())
    }
}
