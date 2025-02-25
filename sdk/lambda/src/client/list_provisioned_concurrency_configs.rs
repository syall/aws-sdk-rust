// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListProvisionedConcurrencyConfigs`](crate::operation::list_provisioned_concurrency_configs::builders::ListProvisionedConcurrencyConfigsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_provisioned_concurrency_configs::builders::ListProvisionedConcurrencyConfigsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`function_name(impl Into<String>)`](crate::operation::list_provisioned_concurrency_configs::builders::ListProvisionedConcurrencyConfigsFluentBuilder::function_name) / [`set_function_name(Option<String>)`](crate::operation::list_provisioned_concurrency_configs::builders::ListProvisionedConcurrencyConfigsFluentBuilder::set_function_name):<br>required: **true**<br><p>The name of the Lambda function.</p>  <p class="title"> <b>Name formats</b> </p>  <ul>   <li> <p> <b>Function name</b> – <code>my-function</code>.</p> </li>   <li> <p> <b>Function ARN</b> – <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li>   <li> <p> <b>Partial ARN</b> – <code>123456789012:function:my-function</code>.</p> </li>  </ul>  <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p><br>
    ///   - [`marker(impl Into<String>)`](crate::operation::list_provisioned_concurrency_configs::builders::ListProvisionedConcurrencyConfigsFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::list_provisioned_concurrency_configs::builders::ListProvisionedConcurrencyConfigsFluentBuilder::set_marker):<br>required: **false**<br><p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p><br>
    ///   - [`max_items(i32)`](crate::operation::list_provisioned_concurrency_configs::builders::ListProvisionedConcurrencyConfigsFluentBuilder::max_items) / [`set_max_items(Option<i32>)`](crate::operation::list_provisioned_concurrency_configs::builders::ListProvisionedConcurrencyConfigsFluentBuilder::set_max_items):<br>required: **false**<br><p>Specify a number to limit the number of configurations returned.</p><br>
    /// - On success, responds with [`ListProvisionedConcurrencyConfigsOutput`](crate::operation::list_provisioned_concurrency_configs::ListProvisionedConcurrencyConfigsOutput) with field(s):
    ///   - [`provisioned_concurrency_configs(Option<Vec::<ProvisionedConcurrencyConfigListItem>>)`](crate::operation::list_provisioned_concurrency_configs::ListProvisionedConcurrencyConfigsOutput::provisioned_concurrency_configs): <p>A list of provisioned concurrency configurations.</p>
    ///   - [`next_marker(Option<String>)`](crate::operation::list_provisioned_concurrency_configs::ListProvisionedConcurrencyConfigsOutput::next_marker): <p>The pagination token that's included if more results are available.</p>
    /// - On failure, responds with [`SdkError<ListProvisionedConcurrencyConfigsError>`](crate::operation::list_provisioned_concurrency_configs::ListProvisionedConcurrencyConfigsError)
    pub fn list_provisioned_concurrency_configs(
        &self,
    ) -> crate::operation::list_provisioned_concurrency_configs::builders::ListProvisionedConcurrencyConfigsFluentBuilder {
        crate::operation::list_provisioned_concurrency_configs::builders::ListProvisionedConcurrencyConfigsFluentBuilder::new(self.handle.clone())
    }
}
