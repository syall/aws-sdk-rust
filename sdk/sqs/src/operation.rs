// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AddPermission`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`add_permission`](crate::client::Client::add_permission).
///
/// See [`crate::client::fluent_builders::AddPermission`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AddPermission {
    _private: (),
}
impl AddPermission {
    /// Creates a new builder-style object to manufacture [`AddPermissionInput`](crate::input::AddPermissionInput).
    pub fn builder() -> crate::input::add_permission_input::Builder {
        crate::input::add_permission_input::Builder::default()
    }
    /// Creates a new `AddPermission` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AddPermission {
    type Output =
        std::result::Result<crate::output::AddPermissionOutput, crate::error::AddPermissionError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_add_permission_error(response)
        } else {
            crate::operation_deser::parse_add_permission_response(response)
        }
    }
}

/// Operation shape for `ChangeMessageVisibility`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`change_message_visibility`](crate::client::Client::change_message_visibility).
///
/// See [`crate::client::fluent_builders::ChangeMessageVisibility`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ChangeMessageVisibility {
    _private: (),
}
impl ChangeMessageVisibility {
    /// Creates a new builder-style object to manufacture [`ChangeMessageVisibilityInput`](crate::input::ChangeMessageVisibilityInput).
    pub fn builder() -> crate::input::change_message_visibility_input::Builder {
        crate::input::change_message_visibility_input::Builder::default()
    }
    /// Creates a new `ChangeMessageVisibility` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ChangeMessageVisibility {
    type Output = std::result::Result<
        crate::output::ChangeMessageVisibilityOutput,
        crate::error::ChangeMessageVisibilityError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_change_message_visibility_error(response)
        } else {
            crate::operation_deser::parse_change_message_visibility_response(response)
        }
    }
}
#[cfg(test)]
#[allow(unreachable_code, unused_variables)]
mod change_message_visibility_request_test {
    /// This test case validates a bug found here: https://github.com/aws/aws-sdk-go-v2/issues/1087
    /// Test ID: SqsSetVisibilityZero
    #[tokio::test]
    async fn sqs_set_visibility_zero_request() {
        let builder = crate::config::Config::builder();

        let config = builder.build();
        let input = crate::input::ChangeMessageVisibilityInput::builder()
            .set_queue_url(Some("http://somequeue.amazon.com".to_owned()))
            .set_receipt_handle(Some("handlehandle".to_owned()))
            .set_visibility_timeout(Some(0))
            .build()
            .unwrap()
            .make_operation(&config)
            .await
            .expect("operation failed to build");
        let (http_request, parts) = input.into_request_response().0.into_parts();
        pretty_assertions::assert_eq!(http_request.method(), "POST");
        pretty_assertions::assert_eq!(http_request.uri().path(), "/");
        let body = http_request.body().bytes().expect("body should be strict");
        aws_smithy_protocol_test::assert_ok(
        aws_smithy_protocol_test::validate_body(&body, "Action=ChangeMessageVisibility&Version=2012-11-05&QueueUrl=http%3A%2F%2Fsomequeue.amazon.com&ReceiptHandle=handlehandle&VisibilityTimeout=0", aws_smithy_protocol_test::MediaType::from("application/x-www-formurl-encoded"))
        );
    }
}

/// Operation shape for `ChangeMessageVisibilityBatch`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`change_message_visibility_batch`](crate::client::Client::change_message_visibility_batch).
///
/// See [`crate::client::fluent_builders::ChangeMessageVisibilityBatch`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ChangeMessageVisibilityBatch {
    _private: (),
}
impl ChangeMessageVisibilityBatch {
    /// Creates a new builder-style object to manufacture [`ChangeMessageVisibilityBatchInput`](crate::input::ChangeMessageVisibilityBatchInput).
    pub fn builder() -> crate::input::change_message_visibility_batch_input::Builder {
        crate::input::change_message_visibility_batch_input::Builder::default()
    }
    /// Creates a new `ChangeMessageVisibilityBatch` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ChangeMessageVisibilityBatch {
    type Output = std::result::Result<
        crate::output::ChangeMessageVisibilityBatchOutput,
        crate::error::ChangeMessageVisibilityBatchError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_change_message_visibility_batch_error(response)
        } else {
            crate::operation_deser::parse_change_message_visibility_batch_response(response)
        }
    }
}

/// Operation shape for `CreateQueue`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_queue`](crate::client::Client::create_queue).
///
/// See [`crate::client::fluent_builders::CreateQueue`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateQueue {
    _private: (),
}
impl CreateQueue {
    /// Creates a new builder-style object to manufacture [`CreateQueueInput`](crate::input::CreateQueueInput).
    pub fn builder() -> crate::input::create_queue_input::Builder {
        crate::input::create_queue_input::Builder::default()
    }
    /// Creates a new `CreateQueue` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateQueue {
    type Output =
        std::result::Result<crate::output::CreateQueueOutput, crate::error::CreateQueueError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_queue_error(response)
        } else {
            crate::operation_deser::parse_create_queue_response(response)
        }
    }
}

/// Operation shape for `DeleteMessage`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_message`](crate::client::Client::delete_message).
///
/// See [`crate::client::fluent_builders::DeleteMessage`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteMessage {
    _private: (),
}
impl DeleteMessage {
    /// Creates a new builder-style object to manufacture [`DeleteMessageInput`](crate::input::DeleteMessageInput).
    pub fn builder() -> crate::input::delete_message_input::Builder {
        crate::input::delete_message_input::Builder::default()
    }
    /// Creates a new `DeleteMessage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteMessage {
    type Output =
        std::result::Result<crate::output::DeleteMessageOutput, crate::error::DeleteMessageError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_message_error(response)
        } else {
            crate::operation_deser::parse_delete_message_response(response)
        }
    }
}

/// Operation shape for `DeleteMessageBatch`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_message_batch`](crate::client::Client::delete_message_batch).
///
/// See [`crate::client::fluent_builders::DeleteMessageBatch`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteMessageBatch {
    _private: (),
}
impl DeleteMessageBatch {
    /// Creates a new builder-style object to manufacture [`DeleteMessageBatchInput`](crate::input::DeleteMessageBatchInput).
    pub fn builder() -> crate::input::delete_message_batch_input::Builder {
        crate::input::delete_message_batch_input::Builder::default()
    }
    /// Creates a new `DeleteMessageBatch` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteMessageBatch {
    type Output = std::result::Result<
        crate::output::DeleteMessageBatchOutput,
        crate::error::DeleteMessageBatchError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_message_batch_error(response)
        } else {
            crate::operation_deser::parse_delete_message_batch_response(response)
        }
    }
}

/// Operation shape for `DeleteQueue`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_queue`](crate::client::Client::delete_queue).
///
/// See [`crate::client::fluent_builders::DeleteQueue`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteQueue {
    _private: (),
}
impl DeleteQueue {
    /// Creates a new builder-style object to manufacture [`DeleteQueueInput`](crate::input::DeleteQueueInput).
    pub fn builder() -> crate::input::delete_queue_input::Builder {
        crate::input::delete_queue_input::Builder::default()
    }
    /// Creates a new `DeleteQueue` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteQueue {
    type Output =
        std::result::Result<crate::output::DeleteQueueOutput, crate::error::DeleteQueueError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_queue_error(response)
        } else {
            crate::operation_deser::parse_delete_queue_response(response)
        }
    }
}

/// Operation shape for `GetQueueAttributes`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_queue_attributes`](crate::client::Client::get_queue_attributes).
///
/// See [`crate::client::fluent_builders::GetQueueAttributes`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetQueueAttributes {
    _private: (),
}
impl GetQueueAttributes {
    /// Creates a new builder-style object to manufacture [`GetQueueAttributesInput`](crate::input::GetQueueAttributesInput).
    pub fn builder() -> crate::input::get_queue_attributes_input::Builder {
        crate::input::get_queue_attributes_input::Builder::default()
    }
    /// Creates a new `GetQueueAttributes` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetQueueAttributes {
    type Output = std::result::Result<
        crate::output::GetQueueAttributesOutput,
        crate::error::GetQueueAttributesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_queue_attributes_error(response)
        } else {
            crate::operation_deser::parse_get_queue_attributes_response(response)
        }
    }
}

/// Operation shape for `GetQueueUrl`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_queue_url`](crate::client::Client::get_queue_url).
///
/// See [`crate::client::fluent_builders::GetQueueUrl`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetQueueUrl {
    _private: (),
}
impl GetQueueUrl {
    /// Creates a new builder-style object to manufacture [`GetQueueUrlInput`](crate::input::GetQueueUrlInput).
    pub fn builder() -> crate::input::get_queue_url_input::Builder {
        crate::input::get_queue_url_input::Builder::default()
    }
    /// Creates a new `GetQueueUrl` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetQueueUrl {
    type Output =
        std::result::Result<crate::output::GetQueueUrlOutput, crate::error::GetQueueUrlError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_queue_url_error(response)
        } else {
            crate::operation_deser::parse_get_queue_url_response(response)
        }
    }
}

/// Operation shape for `ListDeadLetterSourceQueues`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_dead_letter_source_queues`](crate::client::Client::list_dead_letter_source_queues).
///
/// See [`crate::client::fluent_builders::ListDeadLetterSourceQueues`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListDeadLetterSourceQueues {
    _private: (),
}
impl ListDeadLetterSourceQueues {
    /// Creates a new builder-style object to manufacture [`ListDeadLetterSourceQueuesInput`](crate::input::ListDeadLetterSourceQueuesInput).
    pub fn builder() -> crate::input::list_dead_letter_source_queues_input::Builder {
        crate::input::list_dead_letter_source_queues_input::Builder::default()
    }
    /// Creates a new `ListDeadLetterSourceQueues` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListDeadLetterSourceQueues {
    type Output = std::result::Result<
        crate::output::ListDeadLetterSourceQueuesOutput,
        crate::error::ListDeadLetterSourceQueuesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_dead_letter_source_queues_error(response)
        } else {
            crate::operation_deser::parse_list_dead_letter_source_queues_response(response)
        }
    }
}

/// Operation shape for `ListQueues`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_queues`](crate::client::Client::list_queues).
///
/// See [`crate::client::fluent_builders::ListQueues`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListQueues {
    _private: (),
}
impl ListQueues {
    /// Creates a new builder-style object to manufacture [`ListQueuesInput`](crate::input::ListQueuesInput).
    pub fn builder() -> crate::input::list_queues_input::Builder {
        crate::input::list_queues_input::Builder::default()
    }
    /// Creates a new `ListQueues` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListQueues {
    type Output =
        std::result::Result<crate::output::ListQueuesOutput, crate::error::ListQueuesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_queues_error(response)
        } else {
            crate::operation_deser::parse_list_queues_response(response)
        }
    }
}

/// Operation shape for `ListQueueTags`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_queue_tags`](crate::client::Client::list_queue_tags).
///
/// See [`crate::client::fluent_builders::ListQueueTags`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListQueueTags {
    _private: (),
}
impl ListQueueTags {
    /// Creates a new builder-style object to manufacture [`ListQueueTagsInput`](crate::input::ListQueueTagsInput).
    pub fn builder() -> crate::input::list_queue_tags_input::Builder {
        crate::input::list_queue_tags_input::Builder::default()
    }
    /// Creates a new `ListQueueTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListQueueTags {
    type Output =
        std::result::Result<crate::output::ListQueueTagsOutput, crate::error::ListQueueTagsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_queue_tags_error(response)
        } else {
            crate::operation_deser::parse_list_queue_tags_response(response)
        }
    }
}

/// Operation shape for `PurgeQueue`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`purge_queue`](crate::client::Client::purge_queue).
///
/// See [`crate::client::fluent_builders::PurgeQueue`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PurgeQueue {
    _private: (),
}
impl PurgeQueue {
    /// Creates a new builder-style object to manufacture [`PurgeQueueInput`](crate::input::PurgeQueueInput).
    pub fn builder() -> crate::input::purge_queue_input::Builder {
        crate::input::purge_queue_input::Builder::default()
    }
    /// Creates a new `PurgeQueue` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PurgeQueue {
    type Output =
        std::result::Result<crate::output::PurgeQueueOutput, crate::error::PurgeQueueError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_purge_queue_error(response)
        } else {
            crate::operation_deser::parse_purge_queue_response(response)
        }
    }
}

/// Operation shape for `ReceiveMessage`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`receive_message`](crate::client::Client::receive_message).
///
/// See [`crate::client::fluent_builders::ReceiveMessage`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ReceiveMessage {
    _private: (),
}
impl ReceiveMessage {
    /// Creates a new builder-style object to manufacture [`ReceiveMessageInput`](crate::input::ReceiveMessageInput).
    pub fn builder() -> crate::input::receive_message_input::Builder {
        crate::input::receive_message_input::Builder::default()
    }
    /// Creates a new `ReceiveMessage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ReceiveMessage {
    type Output =
        std::result::Result<crate::output::ReceiveMessageOutput, crate::error::ReceiveMessageError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_receive_message_error(response)
        } else {
            crate::operation_deser::parse_receive_message_response(response)
        }
    }
}

/// Operation shape for `RemovePermission`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`remove_permission`](crate::client::Client::remove_permission).
///
/// See [`crate::client::fluent_builders::RemovePermission`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RemovePermission {
    _private: (),
}
impl RemovePermission {
    /// Creates a new builder-style object to manufacture [`RemovePermissionInput`](crate::input::RemovePermissionInput).
    pub fn builder() -> crate::input::remove_permission_input::Builder {
        crate::input::remove_permission_input::Builder::default()
    }
    /// Creates a new `RemovePermission` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RemovePermission {
    type Output = std::result::Result<
        crate::output::RemovePermissionOutput,
        crate::error::RemovePermissionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_remove_permission_error(response)
        } else {
            crate::operation_deser::parse_remove_permission_response(response)
        }
    }
}

/// Operation shape for `SendMessage`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`send_message`](crate::client::Client::send_message).
///
/// See [`crate::client::fluent_builders::SendMessage`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SendMessage {
    _private: (),
}
impl SendMessage {
    /// Creates a new builder-style object to manufacture [`SendMessageInput`](crate::input::SendMessageInput).
    pub fn builder() -> crate::input::send_message_input::Builder {
        crate::input::send_message_input::Builder::default()
    }
    /// Creates a new `SendMessage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SendMessage {
    type Output =
        std::result::Result<crate::output::SendMessageOutput, crate::error::SendMessageError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_send_message_error(response)
        } else {
            crate::operation_deser::parse_send_message_response(response)
        }
    }
}

/// Operation shape for `SendMessageBatch`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`send_message_batch`](crate::client::Client::send_message_batch).
///
/// See [`crate::client::fluent_builders::SendMessageBatch`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SendMessageBatch {
    _private: (),
}
impl SendMessageBatch {
    /// Creates a new builder-style object to manufacture [`SendMessageBatchInput`](crate::input::SendMessageBatchInput).
    pub fn builder() -> crate::input::send_message_batch_input::Builder {
        crate::input::send_message_batch_input::Builder::default()
    }
    /// Creates a new `SendMessageBatch` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SendMessageBatch {
    type Output = std::result::Result<
        crate::output::SendMessageBatchOutput,
        crate::error::SendMessageBatchError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_send_message_batch_error(response)
        } else {
            crate::operation_deser::parse_send_message_batch_response(response)
        }
    }
}

/// Operation shape for `SetQueueAttributes`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`set_queue_attributes`](crate::client::Client::set_queue_attributes).
///
/// See [`crate::client::fluent_builders::SetQueueAttributes`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SetQueueAttributes {
    _private: (),
}
impl SetQueueAttributes {
    /// Creates a new builder-style object to manufacture [`SetQueueAttributesInput`](crate::input::SetQueueAttributesInput).
    pub fn builder() -> crate::input::set_queue_attributes_input::Builder {
        crate::input::set_queue_attributes_input::Builder::default()
    }
    /// Creates a new `SetQueueAttributes` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SetQueueAttributes {
    type Output = std::result::Result<
        crate::output::SetQueueAttributesOutput,
        crate::error::SetQueueAttributesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_set_queue_attributes_error(response)
        } else {
            crate::operation_deser::parse_set_queue_attributes_response(response)
        }
    }
}

/// Operation shape for `TagQueue`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_queue`](crate::client::Client::tag_queue).
///
/// See [`crate::client::fluent_builders::TagQueue`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagQueue {
    _private: (),
}
impl TagQueue {
    /// Creates a new builder-style object to manufacture [`TagQueueInput`](crate::input::TagQueueInput).
    pub fn builder() -> crate::input::tag_queue_input::Builder {
        crate::input::tag_queue_input::Builder::default()
    }
    /// Creates a new `TagQueue` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagQueue {
    type Output = std::result::Result<crate::output::TagQueueOutput, crate::error::TagQueueError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_queue_error(response)
        } else {
            crate::operation_deser::parse_tag_queue_response(response)
        }
    }
}

/// Operation shape for `UntagQueue`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_queue`](crate::client::Client::untag_queue).
///
/// See [`crate::client::fluent_builders::UntagQueue`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagQueue {
    _private: (),
}
impl UntagQueue {
    /// Creates a new builder-style object to manufacture [`UntagQueueInput`](crate::input::UntagQueueInput).
    pub fn builder() -> crate::input::untag_queue_input::Builder {
        crate::input::untag_queue_input::Builder::default()
    }
    /// Creates a new `UntagQueue` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagQueue {
    type Output =
        std::result::Result<crate::output::UntagQueueOutput, crate::error::UntagQueueError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_queue_error(response)
        } else {
            crate::operation_deser::parse_untag_queue_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
