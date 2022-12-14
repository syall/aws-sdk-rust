// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct StartChangeSetOutput {
    /// <p>Unique identifier generated for the request.</p>
    #[doc(hidden)]
    pub change_set_id: std::option::Option<std::string::String>,
    /// <p>The ARN associated to the unique identifier generated for the request.</p>
    #[doc(hidden)]
    pub change_set_arn: std::option::Option<std::string::String>,
}
impl StartChangeSetOutput {
    /// <p>Unique identifier generated for the request.</p>
    pub fn change_set_id(&self) -> std::option::Option<&str> {
        self.change_set_id.as_deref()
    }
    /// <p>The ARN associated to the unique identifier generated for the request.</p>
    pub fn change_set_arn(&self) -> std::option::Option<&str> {
        self.change_set_arn.as_deref()
    }
}
/// See [`StartChangeSetOutput`](crate::output::StartChangeSetOutput).
pub mod start_change_set_output {

    /// A builder for [`StartChangeSetOutput`](crate::output::StartChangeSetOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) change_set_id: std::option::Option<std::string::String>,
        pub(crate) change_set_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Unique identifier generated for the request.</p>
        pub fn change_set_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.change_set_id = Some(input.into());
            self
        }
        /// <p>Unique identifier generated for the request.</p>
        pub fn set_change_set_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.change_set_id = input;
            self
        }
        /// <p>The ARN associated to the unique identifier generated for the request.</p>
        pub fn change_set_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.change_set_arn = Some(input.into());
            self
        }
        /// <p>The ARN associated to the unique identifier generated for the request.</p>
        pub fn set_change_set_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.change_set_arn = input;
            self
        }
        /// Consumes the builder and constructs a [`StartChangeSetOutput`](crate::output::StartChangeSetOutput).
        pub fn build(self) -> crate::output::StartChangeSetOutput {
            crate::output::StartChangeSetOutput {
                change_set_id: self.change_set_id,
                change_set_arn: self.change_set_arn,
            }
        }
    }
}
impl StartChangeSetOutput {
    /// Creates a new builder-style object to manufacture [`StartChangeSetOutput`](crate::output::StartChangeSetOutput).
    pub fn builder() -> crate::output::start_change_set_output::Builder {
        crate::output::start_change_set_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListEntitiesOutput {
    /// <p> Array of <code>EntitySummary</code> object.</p>
    #[doc(hidden)]
    pub entity_summary_list: std::option::Option<std::vec::Vec<crate::model::EntitySummary>>,
    /// <p>The value of the next token if it exists. Null if there is no more result.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListEntitiesOutput {
    /// <p> Array of <code>EntitySummary</code> object.</p>
    pub fn entity_summary_list(&self) -> std::option::Option<&[crate::model::EntitySummary]> {
        self.entity_summary_list.as_deref()
    }
    /// <p>The value of the next token if it exists. Null if there is no more result.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
/// See [`ListEntitiesOutput`](crate::output::ListEntitiesOutput).
pub mod list_entities_output {

    /// A builder for [`ListEntitiesOutput`](crate::output::ListEntitiesOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) entity_summary_list:
            std::option::Option<std::vec::Vec<crate::model::EntitySummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `entity_summary_list`.
        ///
        /// To override the contents of this collection use [`set_entity_summary_list`](Self::set_entity_summary_list).
        ///
        /// <p> Array of <code>EntitySummary</code> object.</p>
        pub fn entity_summary_list(mut self, input: crate::model::EntitySummary) -> Self {
            let mut v = self.entity_summary_list.unwrap_or_default();
            v.push(input);
            self.entity_summary_list = Some(v);
            self
        }
        /// <p> Array of <code>EntitySummary</code> object.</p>
        pub fn set_entity_summary_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::EntitySummary>>,
        ) -> Self {
            self.entity_summary_list = input;
            self
        }
        /// <p>The value of the next token if it exists. Null if there is no more result.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The value of the next token if it exists. Null if there is no more result.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListEntitiesOutput`](crate::output::ListEntitiesOutput).
        pub fn build(self) -> crate::output::ListEntitiesOutput {
            crate::output::ListEntitiesOutput {
                entity_summary_list: self.entity_summary_list,
                next_token: self.next_token,
            }
        }
    }
}
impl ListEntitiesOutput {
    /// Creates a new builder-style object to manufacture [`ListEntitiesOutput`](crate::output::ListEntitiesOutput).
    pub fn builder() -> crate::output::list_entities_output::Builder {
        crate::output::list_entities_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListChangeSetsOutput {
    /// <p> Array of <code>ChangeSetSummaryListItem</code> objects.</p>
    #[doc(hidden)]
    pub change_set_summary_list:
        std::option::Option<std::vec::Vec<crate::model::ChangeSetSummaryListItem>>,
    /// <p>The value of the next token, if it exists. Null if there are no more results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListChangeSetsOutput {
    /// <p> Array of <code>ChangeSetSummaryListItem</code> objects.</p>
    pub fn change_set_summary_list(
        &self,
    ) -> std::option::Option<&[crate::model::ChangeSetSummaryListItem]> {
        self.change_set_summary_list.as_deref()
    }
    /// <p>The value of the next token, if it exists. Null if there are no more results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
/// See [`ListChangeSetsOutput`](crate::output::ListChangeSetsOutput).
pub mod list_change_sets_output {

    /// A builder for [`ListChangeSetsOutput`](crate::output::ListChangeSetsOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) change_set_summary_list:
            std::option::Option<std::vec::Vec<crate::model::ChangeSetSummaryListItem>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `change_set_summary_list`.
        ///
        /// To override the contents of this collection use [`set_change_set_summary_list`](Self::set_change_set_summary_list).
        ///
        /// <p> Array of <code>ChangeSetSummaryListItem</code> objects.</p>
        pub fn change_set_summary_list(
            mut self,
            input: crate::model::ChangeSetSummaryListItem,
        ) -> Self {
            let mut v = self.change_set_summary_list.unwrap_or_default();
            v.push(input);
            self.change_set_summary_list = Some(v);
            self
        }
        /// <p> Array of <code>ChangeSetSummaryListItem</code> objects.</p>
        pub fn set_change_set_summary_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ChangeSetSummaryListItem>>,
        ) -> Self {
            self.change_set_summary_list = input;
            self
        }
        /// <p>The value of the next token, if it exists. Null if there are no more results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The value of the next token, if it exists. Null if there are no more results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListChangeSetsOutput`](crate::output::ListChangeSetsOutput).
        pub fn build(self) -> crate::output::ListChangeSetsOutput {
            crate::output::ListChangeSetsOutput {
                change_set_summary_list: self.change_set_summary_list,
                next_token: self.next_token,
            }
        }
    }
}
impl ListChangeSetsOutput {
    /// Creates a new builder-style object to manufacture [`ListChangeSetsOutput`](crate::output::ListChangeSetsOutput).
    pub fn builder() -> crate::output::list_change_sets_output::Builder {
        crate::output::list_change_sets_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeEntityOutput {
    /// <p>The named type of the entity, in the format of <code>EntityType@Version</code>.</p>
    #[doc(hidden)]
    pub entity_type: std::option::Option<std::string::String>,
    /// <p>The identifier of the entity, in the format of <code>EntityId@RevisionId</code>.</p>
    #[doc(hidden)]
    pub entity_identifier: std::option::Option<std::string::String>,
    /// <p>The ARN associated to the unique identifier for the entity referenced in this request.</p>
    #[doc(hidden)]
    pub entity_arn: std::option::Option<std::string::String>,
    /// <p>The last modified date of the entity, in ISO 8601 format (2018-02-27T13:45:22Z).</p>
    #[doc(hidden)]
    pub last_modified_date: std::option::Option<std::string::String>,
    /// <p>This stringified JSON object includes the details of the entity.</p>
    #[doc(hidden)]
    pub details: std::option::Option<std::string::String>,
}
impl DescribeEntityOutput {
    /// <p>The named type of the entity, in the format of <code>EntityType@Version</code>.</p>
    pub fn entity_type(&self) -> std::option::Option<&str> {
        self.entity_type.as_deref()
    }
    /// <p>The identifier of the entity, in the format of <code>EntityId@RevisionId</code>.</p>
    pub fn entity_identifier(&self) -> std::option::Option<&str> {
        self.entity_identifier.as_deref()
    }
    /// <p>The ARN associated to the unique identifier for the entity referenced in this request.</p>
    pub fn entity_arn(&self) -> std::option::Option<&str> {
        self.entity_arn.as_deref()
    }
    /// <p>The last modified date of the entity, in ISO 8601 format (2018-02-27T13:45:22Z).</p>
    pub fn last_modified_date(&self) -> std::option::Option<&str> {
        self.last_modified_date.as_deref()
    }
    /// <p>This stringified JSON object includes the details of the entity.</p>
    pub fn details(&self) -> std::option::Option<&str> {
        self.details.as_deref()
    }
}
/// See [`DescribeEntityOutput`](crate::output::DescribeEntityOutput).
pub mod describe_entity_output {

    /// A builder for [`DescribeEntityOutput`](crate::output::DescribeEntityOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) entity_type: std::option::Option<std::string::String>,
        pub(crate) entity_identifier: std::option::Option<std::string::String>,
        pub(crate) entity_arn: std::option::Option<std::string::String>,
        pub(crate) last_modified_date: std::option::Option<std::string::String>,
        pub(crate) details: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The named type of the entity, in the format of <code>EntityType@Version</code>.</p>
        pub fn entity_type(mut self, input: impl Into<std::string::String>) -> Self {
            self.entity_type = Some(input.into());
            self
        }
        /// <p>The named type of the entity, in the format of <code>EntityType@Version</code>.</p>
        pub fn set_entity_type(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.entity_type = input;
            self
        }
        /// <p>The identifier of the entity, in the format of <code>EntityId@RevisionId</code>.</p>
        pub fn entity_identifier(mut self, input: impl Into<std::string::String>) -> Self {
            self.entity_identifier = Some(input.into());
            self
        }
        /// <p>The identifier of the entity, in the format of <code>EntityId@RevisionId</code>.</p>
        pub fn set_entity_identifier(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.entity_identifier = input;
            self
        }
        /// <p>The ARN associated to the unique identifier for the entity referenced in this request.</p>
        pub fn entity_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.entity_arn = Some(input.into());
            self
        }
        /// <p>The ARN associated to the unique identifier for the entity referenced in this request.</p>
        pub fn set_entity_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.entity_arn = input;
            self
        }
        /// <p>The last modified date of the entity, in ISO 8601 format (2018-02-27T13:45:22Z).</p>
        pub fn last_modified_date(mut self, input: impl Into<std::string::String>) -> Self {
            self.last_modified_date = Some(input.into());
            self
        }
        /// <p>The last modified date of the entity, in ISO 8601 format (2018-02-27T13:45:22Z).</p>
        pub fn set_last_modified_date(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.last_modified_date = input;
            self
        }
        /// <p>This stringified JSON object includes the details of the entity.</p>
        pub fn details(mut self, input: impl Into<std::string::String>) -> Self {
            self.details = Some(input.into());
            self
        }
        /// <p>This stringified JSON object includes the details of the entity.</p>
        pub fn set_details(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.details = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeEntityOutput`](crate::output::DescribeEntityOutput).
        pub fn build(self) -> crate::output::DescribeEntityOutput {
            crate::output::DescribeEntityOutput {
                entity_type: self.entity_type,
                entity_identifier: self.entity_identifier,
                entity_arn: self.entity_arn,
                last_modified_date: self.last_modified_date,
                details: self.details,
            }
        }
    }
}
impl DescribeEntityOutput {
    /// Creates a new builder-style object to manufacture [`DescribeEntityOutput`](crate::output::DescribeEntityOutput).
    pub fn builder() -> crate::output::describe_entity_output::Builder {
        crate::output::describe_entity_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeChangeSetOutput {
    /// <p>Required. The unique identifier for the change set referenced in this request.</p>
    #[doc(hidden)]
    pub change_set_id: std::option::Option<std::string::String>,
    /// <p>The ARN associated with the unique identifier for the change set referenced in this request.</p>
    #[doc(hidden)]
    pub change_set_arn: std::option::Option<std::string::String>,
    /// <p>The optional name provided in the <code>StartChangeSet</code> request. If you do not provide a name, one is set by default.</p>
    #[doc(hidden)]
    pub change_set_name: std::option::Option<std::string::String>,
    /// <p>The date and time, in ISO 8601 format (2018-02-27T13:45:22Z), the request started. </p>
    #[doc(hidden)]
    pub start_time: std::option::Option<std::string::String>,
    /// <p>The date and time, in ISO 8601 format (2018-02-27T13:45:22Z), the request transitioned to a terminal state. The change cannot transition to a different state. Null if the request is not in a terminal state. </p>
    #[doc(hidden)]
    pub end_time: std::option::Option<std::string::String>,
    /// <p>The status of the change request.</p>
    #[doc(hidden)]
    pub status: std::option::Option<crate::model::ChangeStatus>,
    /// <p>Returned if the change set is in <code>FAILED</code> status. Can be either <code>CLIENT_ERROR</code>, which means that there are issues with the request (see the <code>ErrorDetailList</code>), or <code>SERVER_FAULT</code>, which means that there is a problem in the system, and you should retry your request.</p>
    #[doc(hidden)]
    pub failure_code: std::option::Option<crate::model::FailureCode>,
    /// <p>Returned if there is a failure on the change set, but that failure is not related to any of the changes in the request.</p>
    #[doc(hidden)]
    pub failure_description: std::option::Option<std::string::String>,
    /// <p>An array of <code>ChangeSummary</code> objects.</p>
    #[doc(hidden)]
    pub change_set: std::option::Option<std::vec::Vec<crate::model::ChangeSummary>>,
}
impl DescribeChangeSetOutput {
    /// <p>Required. The unique identifier for the change set referenced in this request.</p>
    pub fn change_set_id(&self) -> std::option::Option<&str> {
        self.change_set_id.as_deref()
    }
    /// <p>The ARN associated with the unique identifier for the change set referenced in this request.</p>
    pub fn change_set_arn(&self) -> std::option::Option<&str> {
        self.change_set_arn.as_deref()
    }
    /// <p>The optional name provided in the <code>StartChangeSet</code> request. If you do not provide a name, one is set by default.</p>
    pub fn change_set_name(&self) -> std::option::Option<&str> {
        self.change_set_name.as_deref()
    }
    /// <p>The date and time, in ISO 8601 format (2018-02-27T13:45:22Z), the request started. </p>
    pub fn start_time(&self) -> std::option::Option<&str> {
        self.start_time.as_deref()
    }
    /// <p>The date and time, in ISO 8601 format (2018-02-27T13:45:22Z), the request transitioned to a terminal state. The change cannot transition to a different state. Null if the request is not in a terminal state. </p>
    pub fn end_time(&self) -> std::option::Option<&str> {
        self.end_time.as_deref()
    }
    /// <p>The status of the change request.</p>
    pub fn status(&self) -> std::option::Option<&crate::model::ChangeStatus> {
        self.status.as_ref()
    }
    /// <p>Returned if the change set is in <code>FAILED</code> status. Can be either <code>CLIENT_ERROR</code>, which means that there are issues with the request (see the <code>ErrorDetailList</code>), or <code>SERVER_FAULT</code>, which means that there is a problem in the system, and you should retry your request.</p>
    pub fn failure_code(&self) -> std::option::Option<&crate::model::FailureCode> {
        self.failure_code.as_ref()
    }
    /// <p>Returned if there is a failure on the change set, but that failure is not related to any of the changes in the request.</p>
    pub fn failure_description(&self) -> std::option::Option<&str> {
        self.failure_description.as_deref()
    }
    /// <p>An array of <code>ChangeSummary</code> objects.</p>
    pub fn change_set(&self) -> std::option::Option<&[crate::model::ChangeSummary]> {
        self.change_set.as_deref()
    }
}
/// See [`DescribeChangeSetOutput`](crate::output::DescribeChangeSetOutput).
pub mod describe_change_set_output {

    /// A builder for [`DescribeChangeSetOutput`](crate::output::DescribeChangeSetOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) change_set_id: std::option::Option<std::string::String>,
        pub(crate) change_set_arn: std::option::Option<std::string::String>,
        pub(crate) change_set_name: std::option::Option<std::string::String>,
        pub(crate) start_time: std::option::Option<std::string::String>,
        pub(crate) end_time: std::option::Option<std::string::String>,
        pub(crate) status: std::option::Option<crate::model::ChangeStatus>,
        pub(crate) failure_code: std::option::Option<crate::model::FailureCode>,
        pub(crate) failure_description: std::option::Option<std::string::String>,
        pub(crate) change_set: std::option::Option<std::vec::Vec<crate::model::ChangeSummary>>,
    }
    impl Builder {
        /// <p>Required. The unique identifier for the change set referenced in this request.</p>
        pub fn change_set_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.change_set_id = Some(input.into());
            self
        }
        /// <p>Required. The unique identifier for the change set referenced in this request.</p>
        pub fn set_change_set_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.change_set_id = input;
            self
        }
        /// <p>The ARN associated with the unique identifier for the change set referenced in this request.</p>
        pub fn change_set_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.change_set_arn = Some(input.into());
            self
        }
        /// <p>The ARN associated with the unique identifier for the change set referenced in this request.</p>
        pub fn set_change_set_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.change_set_arn = input;
            self
        }
        /// <p>The optional name provided in the <code>StartChangeSet</code> request. If you do not provide a name, one is set by default.</p>
        pub fn change_set_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.change_set_name = Some(input.into());
            self
        }
        /// <p>The optional name provided in the <code>StartChangeSet</code> request. If you do not provide a name, one is set by default.</p>
        pub fn set_change_set_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.change_set_name = input;
            self
        }
        /// <p>The date and time, in ISO 8601 format (2018-02-27T13:45:22Z), the request started. </p>
        pub fn start_time(mut self, input: impl Into<std::string::String>) -> Self {
            self.start_time = Some(input.into());
            self
        }
        /// <p>The date and time, in ISO 8601 format (2018-02-27T13:45:22Z), the request started. </p>
        pub fn set_start_time(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.start_time = input;
            self
        }
        /// <p>The date and time, in ISO 8601 format (2018-02-27T13:45:22Z), the request transitioned to a terminal state. The change cannot transition to a different state. Null if the request is not in a terminal state. </p>
        pub fn end_time(mut self, input: impl Into<std::string::String>) -> Self {
            self.end_time = Some(input.into());
            self
        }
        /// <p>The date and time, in ISO 8601 format (2018-02-27T13:45:22Z), the request transitioned to a terminal state. The change cannot transition to a different state. Null if the request is not in a terminal state. </p>
        pub fn set_end_time(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.end_time = input;
            self
        }
        /// <p>The status of the change request.</p>
        pub fn status(mut self, input: crate::model::ChangeStatus) -> Self {
            self.status = Some(input);
            self
        }
        /// <p>The status of the change request.</p>
        pub fn set_status(
            mut self,
            input: std::option::Option<crate::model::ChangeStatus>,
        ) -> Self {
            self.status = input;
            self
        }
        /// <p>Returned if the change set is in <code>FAILED</code> status. Can be either <code>CLIENT_ERROR</code>, which means that there are issues with the request (see the <code>ErrorDetailList</code>), or <code>SERVER_FAULT</code>, which means that there is a problem in the system, and you should retry your request.</p>
        pub fn failure_code(mut self, input: crate::model::FailureCode) -> Self {
            self.failure_code = Some(input);
            self
        }
        /// <p>Returned if the change set is in <code>FAILED</code> status. Can be either <code>CLIENT_ERROR</code>, which means that there are issues with the request (see the <code>ErrorDetailList</code>), or <code>SERVER_FAULT</code>, which means that there is a problem in the system, and you should retry your request.</p>
        pub fn set_failure_code(
            mut self,
            input: std::option::Option<crate::model::FailureCode>,
        ) -> Self {
            self.failure_code = input;
            self
        }
        /// <p>Returned if there is a failure on the change set, but that failure is not related to any of the changes in the request.</p>
        pub fn failure_description(mut self, input: impl Into<std::string::String>) -> Self {
            self.failure_description = Some(input.into());
            self
        }
        /// <p>Returned if there is a failure on the change set, but that failure is not related to any of the changes in the request.</p>
        pub fn set_failure_description(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.failure_description = input;
            self
        }
        /// Appends an item to `change_set`.
        ///
        /// To override the contents of this collection use [`set_change_set`](Self::set_change_set).
        ///
        /// <p>An array of <code>ChangeSummary</code> objects.</p>
        pub fn change_set(mut self, input: crate::model::ChangeSummary) -> Self {
            let mut v = self.change_set.unwrap_or_default();
            v.push(input);
            self.change_set = Some(v);
            self
        }
        /// <p>An array of <code>ChangeSummary</code> objects.</p>
        pub fn set_change_set(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ChangeSummary>>,
        ) -> Self {
            self.change_set = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeChangeSetOutput`](crate::output::DescribeChangeSetOutput).
        pub fn build(self) -> crate::output::DescribeChangeSetOutput {
            crate::output::DescribeChangeSetOutput {
                change_set_id: self.change_set_id,
                change_set_arn: self.change_set_arn,
                change_set_name: self.change_set_name,
                start_time: self.start_time,
                end_time: self.end_time,
                status: self.status,
                failure_code: self.failure_code,
                failure_description: self.failure_description,
                change_set: self.change_set,
            }
        }
    }
}
impl DescribeChangeSetOutput {
    /// Creates a new builder-style object to manufacture [`DescribeChangeSetOutput`](crate::output::DescribeChangeSetOutput).
    pub fn builder() -> crate::output::describe_change_set_output::Builder {
        crate::output::describe_change_set_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CancelChangeSetOutput {
    /// <p>The unique identifier for the change set referenced in this request.</p>
    #[doc(hidden)]
    pub change_set_id: std::option::Option<std::string::String>,
    /// <p>The ARN associated with the change set referenced in this request.</p>
    #[doc(hidden)]
    pub change_set_arn: std::option::Option<std::string::String>,
}
impl CancelChangeSetOutput {
    /// <p>The unique identifier for the change set referenced in this request.</p>
    pub fn change_set_id(&self) -> std::option::Option<&str> {
        self.change_set_id.as_deref()
    }
    /// <p>The ARN associated with the change set referenced in this request.</p>
    pub fn change_set_arn(&self) -> std::option::Option<&str> {
        self.change_set_arn.as_deref()
    }
}
/// See [`CancelChangeSetOutput`](crate::output::CancelChangeSetOutput).
pub mod cancel_change_set_output {

    /// A builder for [`CancelChangeSetOutput`](crate::output::CancelChangeSetOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) change_set_id: std::option::Option<std::string::String>,
        pub(crate) change_set_arn: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The unique identifier for the change set referenced in this request.</p>
        pub fn change_set_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.change_set_id = Some(input.into());
            self
        }
        /// <p>The unique identifier for the change set referenced in this request.</p>
        pub fn set_change_set_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.change_set_id = input;
            self
        }
        /// <p>The ARN associated with the change set referenced in this request.</p>
        pub fn change_set_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.change_set_arn = Some(input.into());
            self
        }
        /// <p>The ARN associated with the change set referenced in this request.</p>
        pub fn set_change_set_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.change_set_arn = input;
            self
        }
        /// Consumes the builder and constructs a [`CancelChangeSetOutput`](crate::output::CancelChangeSetOutput).
        pub fn build(self) -> crate::output::CancelChangeSetOutput {
            crate::output::CancelChangeSetOutput {
                change_set_id: self.change_set_id,
                change_set_arn: self.change_set_arn,
            }
        }
    }
}
impl CancelChangeSetOutput {
    /// Creates a new builder-style object to manufacture [`CancelChangeSetOutput`](crate::output::CancelChangeSetOutput).
    pub fn builder() -> crate::output::cancel_change_set_output::Builder {
        crate::output::cancel_change_set_output::Builder::default()
    }
}
