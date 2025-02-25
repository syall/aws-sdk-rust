// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateChannel`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel(impl Into<String>)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::channel) / [`set_channel(Option<String>)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::set_channel):<br>required: **true**<br><p>The ARN or ID (the ARN suffix) of the channel that you want to update.</p><br>
    ///   - [`destinations(Destination)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::destinations) / [`set_destinations(Option<Vec::<Destination>>)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::set_destinations):<br>required: **false**<br><p>The ARNs of event data stores that you want to log events arriving through the channel.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::set_name):<br>required: **false**<br><p> Changes the name of the channel. </p><br>
    /// - On success, responds with [`UpdateChannelOutput`](crate::operation::update_channel::UpdateChannelOutput) with field(s):
    ///   - [`channel_arn(Option<String>)`](crate::operation::update_channel::UpdateChannelOutput::channel_arn): <p>The ARN of the channel that was updated.</p>
    ///   - [`name(Option<String>)`](crate::operation::update_channel::UpdateChannelOutput::name): <p>The name of the channel that was updated.</p>
    ///   - [`source(Option<String>)`](crate::operation::update_channel::UpdateChannelOutput::source): <p>The event source of the channel that was updated.</p>
    ///   - [`destinations(Option<Vec::<Destination>>)`](crate::operation::update_channel::UpdateChannelOutput::destinations): <p>The event data stores that log events arriving through the channel.</p>
    /// - On failure, responds with [`SdkError<UpdateChannelError>`](crate::operation::update_channel::UpdateChannelError)
    pub fn update_channel(&self) -> crate::operation::update_channel::builders::UpdateChannelFluentBuilder {
        crate::operation::update_channel::builders::UpdateChannelFluentBuilder::new(self.handle.clone())
    }
}
