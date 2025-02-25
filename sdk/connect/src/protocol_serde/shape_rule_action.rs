// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_rule_action(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RuleAction,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("ActionType").string(input.action_type.as_str());
    }
    if let Some(var_1) = &input.task_action {
        #[allow(unused_mut)]
        let mut object_2 = object.key("TaskAction").start_object();
        crate::protocol_serde::shape_task_action_definition::ser_task_action_definition(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.event_bridge_action {
        #[allow(unused_mut)]
        let mut object_4 = object.key("EventBridgeAction").start_object();
        crate::protocol_serde::shape_event_bridge_action_definition::ser_event_bridge_action_definition(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.assign_contact_category_action {
        #[allow(unused_mut)]
        let mut object_6 = object.key("AssignContactCategoryAction").start_object();
        crate::protocol_serde::shape_assign_contact_category_action_definition::ser_assign_contact_category_action_definition(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.send_notification_action {
        #[allow(unused_mut)]
        let mut object_8 = object.key("SendNotificationAction").start_object();
        crate::protocol_serde::shape_send_notification_action_definition::ser_send_notification_action_definition(&mut object_8, var_7)?;
        object_8.finish();
    }
    Ok(())
}

pub(crate) fn de_rule_action<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::RuleAction>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::RuleActionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "ActionType" => {
                                builder = builder.set_action_type(
                                    ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                        .map(|s| s.to_unescaped().map(|u| crate::types::ActionType::from(u.as_ref())))
                                        .transpose()?,
                                );
                            }
                            "TaskAction" => {
                                builder =
                                    builder.set_task_action(crate::protocol_serde::shape_task_action_definition::de_task_action_definition(tokens)?);
                            }
                            "EventBridgeAction" => {
                                builder = builder.set_event_bridge_action(
                                    crate::protocol_serde::shape_event_bridge_action_definition::de_event_bridge_action_definition(tokens)?,
                                );
                            }
                            "AssignContactCategoryAction" => {
                                builder = builder.set_assign_contact_category_action(
                                    crate::protocol_serde::shape_assign_contact_category_action_definition::de_assign_contact_category_action_definition(tokens)?
                                );
                            }
                            "SendNotificationAction" => {
                                builder = builder.set_send_notification_action(
                                    crate::protocol_serde::shape_send_notification_action_definition::de_send_notification_action_definition(tokens)?,
                                );
                            }
                            _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(crate::serde_util::rule_action_correct_errors(builder).build().map_err(|err| {
                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
            })?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
