// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_action_identifier(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ActionIdentifier,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("actionType").string(input.action_type.as_str());
    }
    {
        object.key("actionId").string(input.action_id.as_str());
    }
    Ok(())
}
