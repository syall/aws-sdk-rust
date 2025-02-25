// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeTableReplicaAutoScaling`](crate::operation::describe_table_replica_auto_scaling::builders::DescribeTableReplicaAutoScalingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`table_name(impl Into<String>)`](crate::operation::describe_table_replica_auto_scaling::builders::DescribeTableReplicaAutoScalingFluentBuilder::table_name) / [`set_table_name(Option<String>)`](crate::operation::describe_table_replica_auto_scaling::builders::DescribeTableReplicaAutoScalingFluentBuilder::set_table_name):<br>required: **true**<br><p>The name of the table.</p><br>
    /// - On success, responds with [`DescribeTableReplicaAutoScalingOutput`](crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingOutput) with field(s):
    ///   - [`table_auto_scaling_description(Option<TableAutoScalingDescription>)`](crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingOutput::table_auto_scaling_description): <p>Represents the auto scaling properties of the table.</p>
    /// - On failure, responds with [`SdkError<DescribeTableReplicaAutoScalingError>`](crate::operation::describe_table_replica_auto_scaling::DescribeTableReplicaAutoScalingError)
    pub fn describe_table_replica_auto_scaling(
        &self,
    ) -> crate::operation::describe_table_replica_auto_scaling::builders::DescribeTableReplicaAutoScalingFluentBuilder {
        crate::operation::describe_table_replica_auto_scaling::builders::DescribeTableReplicaAutoScalingFluentBuilder::new(self.handle.clone())
    }
}
