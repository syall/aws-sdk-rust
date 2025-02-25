// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeDomainControllers`](crate::operation::describe_domain_controllers::builders::DescribeDomainControllersFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_domain_controllers::builders::DescribeDomainControllersFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_id(impl Into<String>)`](crate::operation::describe_domain_controllers::builders::DescribeDomainControllersFluentBuilder::directory_id) / [`set_directory_id(Option<String>)`](crate::operation::describe_domain_controllers::builders::DescribeDomainControllersFluentBuilder::set_directory_id):<br>required: **true**<br><p>Identifier of the directory for which to retrieve the domain controller information.</p><br>
    ///   - [`domain_controller_ids(impl Into<String>)`](crate::operation::describe_domain_controllers::builders::DescribeDomainControllersFluentBuilder::domain_controller_ids) / [`set_domain_controller_ids(Option<Vec::<String>>)`](crate::operation::describe_domain_controllers::builders::DescribeDomainControllersFluentBuilder::set_domain_controller_ids):<br>required: **false**<br><p>A list of identifiers for the domain controllers whose information will be provided.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_domain_controllers::builders::DescribeDomainControllersFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_domain_controllers::builders::DescribeDomainControllersFluentBuilder::set_next_token):<br>required: **false**<br><p>The <i>DescribeDomainControllers.NextToken</i> value from a previous call to <code>DescribeDomainControllers</code>. Pass null if this is the first call. </p><br>
    ///   - [`limit(i32)`](crate::operation::describe_domain_controllers::builders::DescribeDomainControllersFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::describe_domain_controllers::builders::DescribeDomainControllersFluentBuilder::set_limit):<br>required: **false**<br><p>The maximum number of items to return.</p><br>
    /// - On success, responds with [`DescribeDomainControllersOutput`](crate::operation::describe_domain_controllers::DescribeDomainControllersOutput) with field(s):
    ///   - [`domain_controllers(Option<Vec::<DomainController>>)`](crate::operation::describe_domain_controllers::DescribeDomainControllersOutput::domain_controllers): <p>List of the <code>DomainController</code> objects that were retrieved.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_domain_controllers::DescribeDomainControllersOutput::next_token): <p>If not null, more results are available. Pass this value for the <code>NextToken</code> parameter in a subsequent call to <code>DescribeDomainControllers</code> retrieve the next set of items.</p>
    /// - On failure, responds with [`SdkError<DescribeDomainControllersError>`](crate::operation::describe_domain_controllers::DescribeDomainControllersError)
    pub fn describe_domain_controllers(&self) -> crate::operation::describe_domain_controllers::builders::DescribeDomainControllersFluentBuilder {
        crate::operation::describe_domain_controllers::builders::DescribeDomainControllersFluentBuilder::new(self.handle.clone())
    }
}
