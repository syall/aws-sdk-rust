// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetChannelGroup`](crate::operation::get_channel_group::builders::GetChannelGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_group_name(impl Into<String>)`](crate::operation::get_channel_group::builders::GetChannelGroupFluentBuilder::channel_group_name) / [`set_channel_group_name(Option<String>)`](crate::operation::get_channel_group::builders::GetChannelGroupFluentBuilder::set_channel_group_name):<br>required: **true**<br><p>The name that describes the channel group. The name is the primary identifier for the channel group, and must be unique for your account in the AWS Region.</p><br>
    /// - On success, responds with [`GetChannelGroupOutput`](crate::operation::get_channel_group::GetChannelGroupOutput) with field(s):
    ///   - [`channel_group_name(String)`](crate::operation::get_channel_group::GetChannelGroupOutput::channel_group_name): <p>The name that describes the channel group. The name is the primary identifier for the channel group, and must be unique for your account in the AWS Region.</p>
    ///   - [`arn(String)`](crate::operation::get_channel_group::GetChannelGroupOutput::arn): <p>The Amazon Resource Name (ARN) associated with the resource.</p>
    ///   - [`egress_domain(String)`](crate::operation::get_channel_group::GetChannelGroupOutput::egress_domain): <p>The output domain where the source stream should be sent. Integrate the domain with a downstream CDN (such as Amazon CloudFront) or playback device.</p>
    ///   - [`created_at(DateTime)`](crate::operation::get_channel_group::GetChannelGroupOutput::created_at): <p>The date and time the channel group was created.</p>
    ///   - [`modified_at(DateTime)`](crate::operation::get_channel_group::GetChannelGroupOutput::modified_at): <p>The date and time the channel group was modified.</p>
    ///   - [`description(Option<String>)`](crate::operation::get_channel_group::GetChannelGroupOutput::description): <p>The description for your channel group.</p>
    ///   - [`tags(Option<HashMap::<String, String>>)`](crate::operation::get_channel_group::GetChannelGroupOutput::tags): <p>The comma-separated list of tag key:value pairs assigned to the channel group.</p>
    /// - On failure, responds with [`SdkError<GetChannelGroupError>`](crate::operation::get_channel_group::GetChannelGroupError)
    pub fn get_channel_group(&self) -> crate::operation::get_channel_group::builders::GetChannelGroupFluentBuilder {
        crate::operation::get_channel_group::builders::GetChannelGroupFluentBuilder::new(self.handle.clone())
    }
}
