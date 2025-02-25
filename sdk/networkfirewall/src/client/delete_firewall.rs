// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteFirewall`](crate::operation::delete_firewall::builders::DeleteFirewallFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`firewall_name(impl Into<String>)`](crate::operation::delete_firewall::builders::DeleteFirewallFluentBuilder::firewall_name) / [`set_firewall_name(Option<String>)`](crate::operation::delete_firewall::builders::DeleteFirewallFluentBuilder::set_firewall_name):<br>required: **false**<br><p>The descriptive name of the firewall. You can't change the name of a firewall after you create it.</p>  <p>You must specify the ARN or the name, and you can specify both. </p><br>
    ///   - [`firewall_arn(impl Into<String>)`](crate::operation::delete_firewall::builders::DeleteFirewallFluentBuilder::firewall_arn) / [`set_firewall_arn(Option<String>)`](crate::operation::delete_firewall::builders::DeleteFirewallFluentBuilder::set_firewall_arn):<br>required: **false**<br><p>The Amazon Resource Name (ARN) of the firewall.</p>  <p>You must specify the ARN or the name, and you can specify both. </p><br>
    /// - On success, responds with [`DeleteFirewallOutput`](crate::operation::delete_firewall::DeleteFirewallOutput) with field(s):
    ///   - [`firewall(Option<Firewall>)`](crate::operation::delete_firewall::DeleteFirewallOutput::firewall): <p>The firewall defines the configuration settings for an Network Firewall firewall. These settings include the firewall policy, the subnets in your VPC to use for the firewall endpoints, and any tags that are attached to the firewall Amazon Web Services resource. </p>  <p>The status of the firewall, for example whether it's ready to filter network traffic, is provided in the corresponding <code>FirewallStatus</code>. You can retrieve both objects by calling <code>DescribeFirewall</code>.</p>
    ///   - [`firewall_status(Option<FirewallStatus>)`](crate::operation::delete_firewall::DeleteFirewallOutput::firewall_status): <p>Detailed information about the current status of a <code>Firewall</code>. You can retrieve this for a firewall by calling <code>DescribeFirewall</code> and providing the firewall name and ARN.</p>
    /// - On failure, responds with [`SdkError<DeleteFirewallError>`](crate::operation::delete_firewall::DeleteFirewallError)
    pub fn delete_firewall(&self) -> crate::operation::delete_firewall::builders::DeleteFirewallFluentBuilder {
        crate::operation::delete_firewall::builders::DeleteFirewallFluentBuilder::new(self.handle.clone())
    }
}
