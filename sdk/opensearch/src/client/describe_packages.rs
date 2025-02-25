// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribePackages`](crate::operation::describe_packages::builders::DescribePackagesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_packages::builders::DescribePackagesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(DescribePackagesFilter)`](crate::operation::describe_packages::builders::DescribePackagesFluentBuilder::filters) / [`set_filters(Option<Vec::<DescribePackagesFilter>>)`](crate::operation::describe_packages::builders::DescribePackagesFluentBuilder::set_filters):<br>required: **false**<br><p>Only returns packages that match the <code>DescribePackagesFilterList</code> values.</p><br>
    ///   - [`max_results(i32)`](crate::operation::describe_packages::builders::DescribePackagesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_packages::builders::DescribePackagesFluentBuilder::set_max_results):<br>required: **false**<br><p>An optional parameter that specifies the maximum number of results to return. You can use <code>nextToken</code> to get the next page of results.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_packages::builders::DescribePackagesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_packages::builders::DescribePackagesFluentBuilder::set_next_token):<br>required: **false**<br><p>If your initial <code>DescribePackageFilters</code> operation returns a <code>nextToken</code>, you can include the returned <code>nextToken</code> in subsequent <code>DescribePackageFilters</code> operations, which returns results in the next page.</p><br>
    /// - On success, responds with [`DescribePackagesOutput`](crate::operation::describe_packages::DescribePackagesOutput) with field(s):
    ///   - [`package_details_list(Option<Vec::<PackageDetails>>)`](crate::operation::describe_packages::DescribePackagesOutput::package_details_list): <p>Basic information about a package.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_packages::DescribePackagesOutput::next_token): <p>When <code>nextToken</code> is returned, there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. Make the call again using the returned token to retrieve the next page.</p>
    /// - On failure, responds with [`SdkError<DescribePackagesError>`](crate::operation::describe_packages::DescribePackagesError)
    pub fn describe_packages(&self) -> crate::operation::describe_packages::builders::DescribePackagesFluentBuilder {
        crate::operation::describe_packages::builders::DescribePackagesFluentBuilder::new(self.handle.clone())
    }
}
