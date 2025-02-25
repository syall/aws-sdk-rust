// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeVerifiedAccessTrustProviders`](crate::operation::describe_verified_access_trust_providers::builders::DescribeVerifiedAccessTrustProvidersFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_verified_access_trust_providers::builders::DescribeVerifiedAccessTrustProvidersFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`verified_access_trust_provider_ids(impl Into<String>)`](crate::operation::describe_verified_access_trust_providers::builders::DescribeVerifiedAccessTrustProvidersFluentBuilder::verified_access_trust_provider_ids) / [`set_verified_access_trust_provider_ids(Option<Vec::<String>>)`](crate::operation::describe_verified_access_trust_providers::builders::DescribeVerifiedAccessTrustProvidersFluentBuilder::set_verified_access_trust_provider_ids):<br>required: **false**<br><p>The IDs of the Verified Access trust providers.</p><br>
    ///   - [`max_results(i32)`](crate::operation::describe_verified_access_trust_providers::builders::DescribeVerifiedAccessTrustProvidersFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_verified_access_trust_providers::builders::DescribeVerifiedAccessTrustProvidersFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_verified_access_trust_providers::builders::DescribeVerifiedAccessTrustProvidersFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_verified_access_trust_providers::builders::DescribeVerifiedAccessTrustProvidersFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next page of results.</p><br>
    ///   - [`filters(Filter)`](crate::operation::describe_verified_access_trust_providers::builders::DescribeVerifiedAccessTrustProvidersFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::describe_verified_access_trust_providers::builders::DescribeVerifiedAccessTrustProvidersFluentBuilder::set_filters):<br>required: **false**<br><p>One or more filters. Filter names and values are case-sensitive.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::describe_verified_access_trust_providers::builders::DescribeVerifiedAccessTrustProvidersFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_verified_access_trust_providers::builders::DescribeVerifiedAccessTrustProvidersFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`DescribeVerifiedAccessTrustProvidersOutput`](crate::operation::describe_verified_access_trust_providers::DescribeVerifiedAccessTrustProvidersOutput) with field(s):
    ///   - [`verified_access_trust_providers(Option<Vec::<VerifiedAccessTrustProvider>>)`](crate::operation::describe_verified_access_trust_providers::DescribeVerifiedAccessTrustProvidersOutput::verified_access_trust_providers): <p>The IDs of the Verified Access trust providers.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_verified_access_trust_providers::DescribeVerifiedAccessTrustProvidersOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeVerifiedAccessTrustProvidersError>`](crate::operation::describe_verified_access_trust_providers::DescribeVerifiedAccessTrustProvidersError)
    pub fn describe_verified_access_trust_providers(
        &self,
    ) -> crate::operation::describe_verified_access_trust_providers::builders::DescribeVerifiedAccessTrustProvidersFluentBuilder {
        crate::operation::describe_verified_access_trust_providers::builders::DescribeVerifiedAccessTrustProvidersFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
