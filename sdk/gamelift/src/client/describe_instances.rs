// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeInstances`](crate::operation::describe_instances::builders::DescribeInstancesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_instances::builders::DescribeInstancesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`fleet_id(impl Into<String>)`](crate::operation::describe_instances::builders::DescribeInstancesFluentBuilder::fleet_id) / [`set_fleet_id(Option<String>)`](crate::operation::describe_instances::builders::DescribeInstancesFluentBuilder::set_fleet_id):<br>required: **true**<br><p>A unique identifier for the fleet to retrieve instance information for. You can use either the fleet ID or ARN value.</p><br>
    ///   - [`instance_id(impl Into<String>)`](crate::operation::describe_instances::builders::DescribeInstancesFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::describe_instances::builders::DescribeInstancesFluentBuilder::set_instance_id):<br>required: **false**<br><p>A unique identifier for an instance to retrieve. Specify an instance ID or leave blank to retrieve all instances in the fleet.</p><br>
    ///   - [`limit(i32)`](crate::operation::describe_instances::builders::DescribeInstancesFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::describe_instances::builders::DescribeInstancesFluentBuilder::set_limit):<br>required: **false**<br><p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_instances::builders::DescribeInstancesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_instances::builders::DescribeInstancesFluentBuilder::set_next_token):<br>required: **false**<br><p>A token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p><br>
    ///   - [`location(impl Into<String>)`](crate::operation::describe_instances::builders::DescribeInstancesFluentBuilder::location) / [`set_location(Option<String>)`](crate::operation::describe_instances::builders::DescribeInstancesFluentBuilder::set_location):<br>required: **false**<br><p>The name of a location to retrieve instance information for, in the form of an Amazon Web Services Region code such as <code>us-west-2</code>. </p><br>
    /// - On success, responds with [`DescribeInstancesOutput`](crate::operation::describe_instances::DescribeInstancesOutput) with field(s):
    ///   - [`instances(Option<Vec::<Instance>>)`](crate::operation::describe_instances::DescribeInstancesOutput::instances): <p>A collection of objects containing properties for each instance returned.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_instances::DescribeInstancesOutput::next_token): <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    /// - On failure, responds with [`SdkError<DescribeInstancesError>`](crate::operation::describe_instances::DescribeInstancesError)
    pub fn describe_instances(&self) -> crate::operation::describe_instances::builders::DescribeInstancesFluentBuilder {
        crate::operation::describe_instances::builders::DescribeInstancesFluentBuilder::new(self.handle.clone())
    }
}
