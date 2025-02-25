// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateGroup`](crate::operation::update_group::builders::UpdateGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`group_name(impl Into<String>)`](crate::operation::update_group::builders::UpdateGroupFluentBuilder::group_name) / [`set_group_name(Option<String>)`](crate::operation::update_group::builders::UpdateGroupFluentBuilder::set_group_name):<br>required: **true**<br><p>Name of the IAM group to update. If you're changing the name of the group, this is the original name.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p><br>
    ///   - [`new_path(impl Into<String>)`](crate::operation::update_group::builders::UpdateGroupFluentBuilder::new_path) / [`set_new_path(Option<String>)`](crate::operation::update_group::builders::UpdateGroupFluentBuilder::set_new_path):<br>required: **false**<br><p>New path for the IAM group. Only include this if changing the group's path.</p>  <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of either a forward slash (/) by itself or a string that must begin and end with forward slashes. In addition, it can contain any ASCII character from the ! (<code>\u0021</code>) through the DEL character (<code>\u007F</code>), including most punctuation characters, digits, and upper and lowercased letters.</p><br>
    ///   - [`new_group_name(impl Into<String>)`](crate::operation::update_group::builders::UpdateGroupFluentBuilder::new_group_name) / [`set_new_group_name(Option<String>)`](crate::operation::update_group::builders::UpdateGroupFluentBuilder::set_new_group_name):<br>required: **false**<br><p>New name for the IAM group. Only include this if changing the group's name.</p>  <p>IAM user, group, role, and policy names must be unique within the account. Names are not distinguished by case. For example, you cannot create resources named both "MyResource" and "myresource".</p><br>
    /// - On success, responds with [`UpdateGroupOutput`](crate::operation::update_group::UpdateGroupOutput)
    /// - On failure, responds with [`SdkError<UpdateGroupError>`](crate::operation::update_group::UpdateGroupError)
    pub fn update_group(&self) -> crate::operation::update_group::builders::UpdateGroupFluentBuilder {
        crate::operation::update_group::builders::UpdateGroupFluentBuilder::new(self.handle.clone())
    }
}
