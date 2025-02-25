// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetEnabledStandards`](crate::operation::get_enabled_standards::builders::GetEnabledStandardsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_enabled_standards::builders::GetEnabledStandardsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`standards_subscription_arns(impl Into<String>)`](crate::operation::get_enabled_standards::builders::GetEnabledStandardsFluentBuilder::standards_subscription_arns) / [`set_standards_subscription_arns(Option<Vec::<String>>)`](crate::operation::get_enabled_standards::builders::GetEnabledStandardsFluentBuilder::set_standards_subscription_arns):<br>required: **false**<br><p>The list of the standards subscription ARNs for the standards to retrieve.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::get_enabled_standards::builders::GetEnabledStandardsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_enabled_standards::builders::GetEnabledStandardsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token that is required for pagination. On your first call to the <code>GetEnabledStandards</code> operation, set the value of this parameter to <code>NULL</code>.</p>  <p>For subsequent calls to the operation, to continue listing data, set the value of this parameter to the value returned from the previous response.</p><br>
    ///   - [`max_results(i32)`](crate::operation::get_enabled_standards::builders::GetEnabledStandardsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_enabled_standards::builders::GetEnabledStandardsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return in the response.</p><br>
    /// - On success, responds with [`GetEnabledStandardsOutput`](crate::operation::get_enabled_standards::GetEnabledStandardsOutput) with field(s):
    ///   - [`standards_subscriptions(Option<Vec::<StandardsSubscription>>)`](crate::operation::get_enabled_standards::GetEnabledStandardsOutput::standards_subscriptions): <p>The list of <code>StandardsSubscriptions</code> objects that include information about the enabled standards.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_enabled_standards::GetEnabledStandardsOutput::next_token): <p>The pagination token to use to request the next page of results.</p>
    /// - On failure, responds with [`SdkError<GetEnabledStandardsError>`](crate::operation::get_enabled_standards::GetEnabledStandardsError)
    pub fn get_enabled_standards(&self) -> crate::operation::get_enabled_standards::builders::GetEnabledStandardsFluentBuilder {
        crate::operation::get_enabled_standards::builders::GetEnabledStandardsFluentBuilder::new(self.handle.clone())
    }
}
