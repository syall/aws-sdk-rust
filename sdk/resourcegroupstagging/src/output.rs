// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UntagResourcesOutput {
    /// <p>A map containing a key-value pair for each failed item that couldn't be untagged. The key is the ARN of the failed resource. The value is a <code>FailureInfo</code> object that contains an error code, a status code, and an error message. If there are no errors, the <code>FailedResourcesMap</code> is empty.</p>
    #[doc(hidden)]
    pub failed_resources_map: std::option::Option<
        std::collections::HashMap<std::string::String, crate::model::FailureInfo>,
    >,
}
impl UntagResourcesOutput {
    /// <p>A map containing a key-value pair for each failed item that couldn't be untagged. The key is the ARN of the failed resource. The value is a <code>FailureInfo</code> object that contains an error code, a status code, and an error message. If there are no errors, the <code>FailedResourcesMap</code> is empty.</p>
    pub fn failed_resources_map(
        &self,
    ) -> std::option::Option<
        &std::collections::HashMap<std::string::String, crate::model::FailureInfo>,
    > {
        self.failed_resources_map.as_ref()
    }
}
/// See [`UntagResourcesOutput`](crate::output::UntagResourcesOutput).
pub mod untag_resources_output {

    /// A builder for [`UntagResourcesOutput`](crate::output::UntagResourcesOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) failed_resources_map: std::option::Option<
            std::collections::HashMap<std::string::String, crate::model::FailureInfo>,
        >,
    }
    impl Builder {
        /// Adds a key-value pair to `failed_resources_map`.
        ///
        /// To override the contents of this collection use [`set_failed_resources_map`](Self::set_failed_resources_map).
        ///
        /// <p>A map containing a key-value pair for each failed item that couldn't be untagged. The key is the ARN of the failed resource. The value is a <code>FailureInfo</code> object that contains an error code, a status code, and an error message. If there are no errors, the <code>FailedResourcesMap</code> is empty.</p>
        pub fn failed_resources_map(
            mut self,
            k: impl Into<std::string::String>,
            v: crate::model::FailureInfo,
        ) -> Self {
            let mut hash_map = self.failed_resources_map.unwrap_or_default();
            hash_map.insert(k.into(), v);
            self.failed_resources_map = Some(hash_map);
            self
        }
        /// <p>A map containing a key-value pair for each failed item that couldn't be untagged. The key is the ARN of the failed resource. The value is a <code>FailureInfo</code> object that contains an error code, a status code, and an error message. If there are no errors, the <code>FailedResourcesMap</code> is empty.</p>
        pub fn set_failed_resources_map(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, crate::model::FailureInfo>,
            >,
        ) -> Self {
            self.failed_resources_map = input;
            self
        }
        /// Consumes the builder and constructs a [`UntagResourcesOutput`](crate::output::UntagResourcesOutput).
        pub fn build(self) -> crate::output::UntagResourcesOutput {
            crate::output::UntagResourcesOutput {
                failed_resources_map: self.failed_resources_map,
            }
        }
    }
}
impl UntagResourcesOutput {
    /// Creates a new builder-style object to manufacture [`UntagResourcesOutput`](crate::output::UntagResourcesOutput).
    pub fn builder() -> crate::output::untag_resources_output::Builder {
        crate::output::untag_resources_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct TagResourcesOutput {
    /// <p>A map containing a key-value pair for each failed item that couldn't be tagged. The key is the ARN of the failed resource. The value is a <code>FailureInfo</code> object that contains an error code, a status code, and an error message. If there are no errors, the <code>FailedResourcesMap</code> is empty.</p>
    #[doc(hidden)]
    pub failed_resources_map: std::option::Option<
        std::collections::HashMap<std::string::String, crate::model::FailureInfo>,
    >,
}
impl TagResourcesOutput {
    /// <p>A map containing a key-value pair for each failed item that couldn't be tagged. The key is the ARN of the failed resource. The value is a <code>FailureInfo</code> object that contains an error code, a status code, and an error message. If there are no errors, the <code>FailedResourcesMap</code> is empty.</p>
    pub fn failed_resources_map(
        &self,
    ) -> std::option::Option<
        &std::collections::HashMap<std::string::String, crate::model::FailureInfo>,
    > {
        self.failed_resources_map.as_ref()
    }
}
/// See [`TagResourcesOutput`](crate::output::TagResourcesOutput).
pub mod tag_resources_output {

    /// A builder for [`TagResourcesOutput`](crate::output::TagResourcesOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) failed_resources_map: std::option::Option<
            std::collections::HashMap<std::string::String, crate::model::FailureInfo>,
        >,
    }
    impl Builder {
        /// Adds a key-value pair to `failed_resources_map`.
        ///
        /// To override the contents of this collection use [`set_failed_resources_map`](Self::set_failed_resources_map).
        ///
        /// <p>A map containing a key-value pair for each failed item that couldn't be tagged. The key is the ARN of the failed resource. The value is a <code>FailureInfo</code> object that contains an error code, a status code, and an error message. If there are no errors, the <code>FailedResourcesMap</code> is empty.</p>
        pub fn failed_resources_map(
            mut self,
            k: impl Into<std::string::String>,
            v: crate::model::FailureInfo,
        ) -> Self {
            let mut hash_map = self.failed_resources_map.unwrap_or_default();
            hash_map.insert(k.into(), v);
            self.failed_resources_map = Some(hash_map);
            self
        }
        /// <p>A map containing a key-value pair for each failed item that couldn't be tagged. The key is the ARN of the failed resource. The value is a <code>FailureInfo</code> object that contains an error code, a status code, and an error message. If there are no errors, the <code>FailedResourcesMap</code> is empty.</p>
        pub fn set_failed_resources_map(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, crate::model::FailureInfo>,
            >,
        ) -> Self {
            self.failed_resources_map = input;
            self
        }
        /// Consumes the builder and constructs a [`TagResourcesOutput`](crate::output::TagResourcesOutput).
        pub fn build(self) -> crate::output::TagResourcesOutput {
            crate::output::TagResourcesOutput {
                failed_resources_map: self.failed_resources_map,
            }
        }
    }
}
impl TagResourcesOutput {
    /// Creates a new builder-style object to manufacture [`TagResourcesOutput`](crate::output::TagResourcesOutput).
    pub fn builder() -> crate::output::tag_resources_output::Builder {
        crate::output::tag_resources_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct StartReportCreationOutput {}
/// See [`StartReportCreationOutput`](crate::output::StartReportCreationOutput).
pub mod start_report_creation_output {

    /// A builder for [`StartReportCreationOutput`](crate::output::StartReportCreationOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`StartReportCreationOutput`](crate::output::StartReportCreationOutput).
        pub fn build(self) -> crate::output::StartReportCreationOutput {
            crate::output::StartReportCreationOutput {}
        }
    }
}
impl StartReportCreationOutput {
    /// Creates a new builder-style object to manufacture [`StartReportCreationOutput`](crate::output::StartReportCreationOutput).
    pub fn builder() -> crate::output::start_report_creation_output::Builder {
        crate::output::start_report_creation_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetTagValuesOutput {
    /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
    #[doc(hidden)]
    pub pagination_token: std::option::Option<std::string::String>,
    /// <p>A list of all tag values for the specified key currently used in the specified Amazon Web Services Region for the calling account.</p>
    #[doc(hidden)]
    pub tag_values: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl GetTagValuesOutput {
    /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
    pub fn pagination_token(&self) -> std::option::Option<&str> {
        self.pagination_token.as_deref()
    }
    /// <p>A list of all tag values for the specified key currently used in the specified Amazon Web Services Region for the calling account.</p>
    pub fn tag_values(&self) -> std::option::Option<&[std::string::String]> {
        self.tag_values.as_deref()
    }
}
/// See [`GetTagValuesOutput`](crate::output::GetTagValuesOutput).
pub mod get_tag_values_output {

    /// A builder for [`GetTagValuesOutput`](crate::output::GetTagValuesOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) pagination_token: std::option::Option<std::string::String>,
        pub(crate) tag_values: std::option::Option<std::vec::Vec<std::string::String>>,
    }
    impl Builder {
        /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
        pub fn pagination_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.pagination_token = Some(input.into());
            self
        }
        /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
        pub fn set_pagination_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.pagination_token = input;
            self
        }
        /// Appends an item to `tag_values`.
        ///
        /// To override the contents of this collection use [`set_tag_values`](Self::set_tag_values).
        ///
        /// <p>A list of all tag values for the specified key currently used in the specified Amazon Web Services Region for the calling account.</p>
        pub fn tag_values(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.tag_values.unwrap_or_default();
            v.push(input.into());
            self.tag_values = Some(v);
            self
        }
        /// <p>A list of all tag values for the specified key currently used in the specified Amazon Web Services Region for the calling account.</p>
        pub fn set_tag_values(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.tag_values = input;
            self
        }
        /// Consumes the builder and constructs a [`GetTagValuesOutput`](crate::output::GetTagValuesOutput).
        pub fn build(self) -> crate::output::GetTagValuesOutput {
            crate::output::GetTagValuesOutput {
                pagination_token: self.pagination_token,
                tag_values: self.tag_values,
            }
        }
    }
}
impl GetTagValuesOutput {
    /// Creates a new builder-style object to manufacture [`GetTagValuesOutput`](crate::output::GetTagValuesOutput).
    pub fn builder() -> crate::output::get_tag_values_output::Builder {
        crate::output::get_tag_values_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetTagKeysOutput {
    /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
    #[doc(hidden)]
    pub pagination_token: std::option::Option<std::string::String>,
    /// <p>A list of all tag keys in the Amazon Web Services account.</p>
    #[doc(hidden)]
    pub tag_keys: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl GetTagKeysOutput {
    /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
    pub fn pagination_token(&self) -> std::option::Option<&str> {
        self.pagination_token.as_deref()
    }
    /// <p>A list of all tag keys in the Amazon Web Services account.</p>
    pub fn tag_keys(&self) -> std::option::Option<&[std::string::String]> {
        self.tag_keys.as_deref()
    }
}
/// See [`GetTagKeysOutput`](crate::output::GetTagKeysOutput).
pub mod get_tag_keys_output {

    /// A builder for [`GetTagKeysOutput`](crate::output::GetTagKeysOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) pagination_token: std::option::Option<std::string::String>,
        pub(crate) tag_keys: std::option::Option<std::vec::Vec<std::string::String>>,
    }
    impl Builder {
        /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
        pub fn pagination_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.pagination_token = Some(input.into());
            self
        }
        /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
        pub fn set_pagination_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.pagination_token = input;
            self
        }
        /// Appends an item to `tag_keys`.
        ///
        /// To override the contents of this collection use [`set_tag_keys`](Self::set_tag_keys).
        ///
        /// <p>A list of all tag keys in the Amazon Web Services account.</p>
        pub fn tag_keys(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.tag_keys.unwrap_or_default();
            v.push(input.into());
            self.tag_keys = Some(v);
            self
        }
        /// <p>A list of all tag keys in the Amazon Web Services account.</p>
        pub fn set_tag_keys(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.tag_keys = input;
            self
        }
        /// Consumes the builder and constructs a [`GetTagKeysOutput`](crate::output::GetTagKeysOutput).
        pub fn build(self) -> crate::output::GetTagKeysOutput {
            crate::output::GetTagKeysOutput {
                pagination_token: self.pagination_token,
                tag_keys: self.tag_keys,
            }
        }
    }
}
impl GetTagKeysOutput {
    /// Creates a new builder-style object to manufacture [`GetTagKeysOutput`](crate::output::GetTagKeysOutput).
    pub fn builder() -> crate::output::get_tag_keys_output::Builder {
        crate::output::get_tag_keys_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetResourcesOutput {
    /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
    #[doc(hidden)]
    pub pagination_token: std::option::Option<std::string::String>,
    /// <p>A list of resource ARNs and the tags (keys and values) associated with each.</p>
    #[doc(hidden)]
    pub resource_tag_mapping_list:
        std::option::Option<std::vec::Vec<crate::model::ResourceTagMapping>>,
}
impl GetResourcesOutput {
    /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
    pub fn pagination_token(&self) -> std::option::Option<&str> {
        self.pagination_token.as_deref()
    }
    /// <p>A list of resource ARNs and the tags (keys and values) associated with each.</p>
    pub fn resource_tag_mapping_list(
        &self,
    ) -> std::option::Option<&[crate::model::ResourceTagMapping]> {
        self.resource_tag_mapping_list.as_deref()
    }
}
/// See [`GetResourcesOutput`](crate::output::GetResourcesOutput).
pub mod get_resources_output {

    /// A builder for [`GetResourcesOutput`](crate::output::GetResourcesOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) pagination_token: std::option::Option<std::string::String>,
        pub(crate) resource_tag_mapping_list:
            std::option::Option<std::vec::Vec<crate::model::ResourceTagMapping>>,
    }
    impl Builder {
        /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
        pub fn pagination_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.pagination_token = Some(input.into());
            self
        }
        /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
        pub fn set_pagination_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.pagination_token = input;
            self
        }
        /// Appends an item to `resource_tag_mapping_list`.
        ///
        /// To override the contents of this collection use [`set_resource_tag_mapping_list`](Self::set_resource_tag_mapping_list).
        ///
        /// <p>A list of resource ARNs and the tags (keys and values) associated with each.</p>
        pub fn resource_tag_mapping_list(
            mut self,
            input: crate::model::ResourceTagMapping,
        ) -> Self {
            let mut v = self.resource_tag_mapping_list.unwrap_or_default();
            v.push(input);
            self.resource_tag_mapping_list = Some(v);
            self
        }
        /// <p>A list of resource ARNs and the tags (keys and values) associated with each.</p>
        pub fn set_resource_tag_mapping_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ResourceTagMapping>>,
        ) -> Self {
            self.resource_tag_mapping_list = input;
            self
        }
        /// Consumes the builder and constructs a [`GetResourcesOutput`](crate::output::GetResourcesOutput).
        pub fn build(self) -> crate::output::GetResourcesOutput {
            crate::output::GetResourcesOutput {
                pagination_token: self.pagination_token,
                resource_tag_mapping_list: self.resource_tag_mapping_list,
            }
        }
    }
}
impl GetResourcesOutput {
    /// Creates a new builder-style object to manufacture [`GetResourcesOutput`](crate::output::GetResourcesOutput).
    pub fn builder() -> crate::output::get_resources_output::Builder {
        crate::output::get_resources_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetComplianceSummaryOutput {
    /// <p>A table that shows counts of noncompliant resources.</p>
    #[doc(hidden)]
    pub summary_list: std::option::Option<std::vec::Vec<crate::model::Summary>>,
    /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
    #[doc(hidden)]
    pub pagination_token: std::option::Option<std::string::String>,
}
impl GetComplianceSummaryOutput {
    /// <p>A table that shows counts of noncompliant resources.</p>
    pub fn summary_list(&self) -> std::option::Option<&[crate::model::Summary]> {
        self.summary_list.as_deref()
    }
    /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
    pub fn pagination_token(&self) -> std::option::Option<&str> {
        self.pagination_token.as_deref()
    }
}
/// See [`GetComplianceSummaryOutput`](crate::output::GetComplianceSummaryOutput).
pub mod get_compliance_summary_output {

    /// A builder for [`GetComplianceSummaryOutput`](crate::output::GetComplianceSummaryOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) summary_list: std::option::Option<std::vec::Vec<crate::model::Summary>>,
        pub(crate) pagination_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `summary_list`.
        ///
        /// To override the contents of this collection use [`set_summary_list`](Self::set_summary_list).
        ///
        /// <p>A table that shows counts of noncompliant resources.</p>
        pub fn summary_list(mut self, input: crate::model::Summary) -> Self {
            let mut v = self.summary_list.unwrap_or_default();
            v.push(input);
            self.summary_list = Some(v);
            self
        }
        /// <p>A table that shows counts of noncompliant resources.</p>
        pub fn set_summary_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Summary>>,
        ) -> Self {
            self.summary_list = input;
            self
        }
        /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
        pub fn pagination_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.pagination_token = Some(input.into());
            self
        }
        /// <p>A string that indicates that there is more data available than this response contains. To receive the next part of the response, specify this response value as the <code>PaginationToken</code> value in the request for the next page.</p>
        pub fn set_pagination_token(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.pagination_token = input;
            self
        }
        /// Consumes the builder and constructs a [`GetComplianceSummaryOutput`](crate::output::GetComplianceSummaryOutput).
        pub fn build(self) -> crate::output::GetComplianceSummaryOutput {
            crate::output::GetComplianceSummaryOutput {
                summary_list: self.summary_list,
                pagination_token: self.pagination_token,
            }
        }
    }
}
impl GetComplianceSummaryOutput {
    /// Creates a new builder-style object to manufacture [`GetComplianceSummaryOutput`](crate::output::GetComplianceSummaryOutput).
    pub fn builder() -> crate::output::get_compliance_summary_output::Builder {
        crate::output::get_compliance_summary_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeReportCreationOutput {
    /// <p>Reports the status of the operation.</p>
    /// <p>The operation status can be one of the following:</p>
    /// <ul>
    /// <li> <p> <code>RUNNING</code> - Report creation is in progress.</p> </li>
    /// <li> <p> <code>SUCCEEDED</code> - Report creation is complete. You can open the report from the Amazon S3 bucket that you specified when you ran <code>StartReportCreation</code>.</p> </li>
    /// <li> <p> <code>FAILED</code> - Report creation timed out or the Amazon S3 bucket is not accessible. </p> </li>
    /// <li> <p> <code>NO REPORT</code> - No report was generated in the last 90 days.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub status: std::option::Option<std::string::String>,
    /// <p>The path to the Amazon S3 bucket where the report was stored on creation.</p>
    #[doc(hidden)]
    pub s3_location: std::option::Option<std::string::String>,
    /// <p>The date and time that the report was started. </p>
    #[doc(hidden)]
    pub start_date: std::option::Option<std::string::String>,
    /// <p>Details of the common errors that all operations return.</p>
    #[doc(hidden)]
    pub error_message: std::option::Option<std::string::String>,
}
impl DescribeReportCreationOutput {
    /// <p>Reports the status of the operation.</p>
    /// <p>The operation status can be one of the following:</p>
    /// <ul>
    /// <li> <p> <code>RUNNING</code> - Report creation is in progress.</p> </li>
    /// <li> <p> <code>SUCCEEDED</code> - Report creation is complete. You can open the report from the Amazon S3 bucket that you specified when you ran <code>StartReportCreation</code>.</p> </li>
    /// <li> <p> <code>FAILED</code> - Report creation timed out or the Amazon S3 bucket is not accessible. </p> </li>
    /// <li> <p> <code>NO REPORT</code> - No report was generated in the last 90 days.</p> </li>
    /// </ul>
    pub fn status(&self) -> std::option::Option<&str> {
        self.status.as_deref()
    }
    /// <p>The path to the Amazon S3 bucket where the report was stored on creation.</p>
    pub fn s3_location(&self) -> std::option::Option<&str> {
        self.s3_location.as_deref()
    }
    /// <p>The date and time that the report was started. </p>
    pub fn start_date(&self) -> std::option::Option<&str> {
        self.start_date.as_deref()
    }
    /// <p>Details of the common errors that all operations return.</p>
    pub fn error_message(&self) -> std::option::Option<&str> {
        self.error_message.as_deref()
    }
}
/// See [`DescribeReportCreationOutput`](crate::output::DescribeReportCreationOutput).
pub mod describe_report_creation_output {

    /// A builder for [`DescribeReportCreationOutput`](crate::output::DescribeReportCreationOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) status: std::option::Option<std::string::String>,
        pub(crate) s3_location: std::option::Option<std::string::String>,
        pub(crate) start_date: std::option::Option<std::string::String>,
        pub(crate) error_message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Reports the status of the operation.</p>
        /// <p>The operation status can be one of the following:</p>
        /// <ul>
        /// <li> <p> <code>RUNNING</code> - Report creation is in progress.</p> </li>
        /// <li> <p> <code>SUCCEEDED</code> - Report creation is complete. You can open the report from the Amazon S3 bucket that you specified when you ran <code>StartReportCreation</code>.</p> </li>
        /// <li> <p> <code>FAILED</code> - Report creation timed out or the Amazon S3 bucket is not accessible. </p> </li>
        /// <li> <p> <code>NO REPORT</code> - No report was generated in the last 90 days.</p> </li>
        /// </ul>
        pub fn status(mut self, input: impl Into<std::string::String>) -> Self {
            self.status = Some(input.into());
            self
        }
        /// <p>Reports the status of the operation.</p>
        /// <p>The operation status can be one of the following:</p>
        /// <ul>
        /// <li> <p> <code>RUNNING</code> - Report creation is in progress.</p> </li>
        /// <li> <p> <code>SUCCEEDED</code> - Report creation is complete. You can open the report from the Amazon S3 bucket that you specified when you ran <code>StartReportCreation</code>.</p> </li>
        /// <li> <p> <code>FAILED</code> - Report creation timed out or the Amazon S3 bucket is not accessible. </p> </li>
        /// <li> <p> <code>NO REPORT</code> - No report was generated in the last 90 days.</p> </li>
        /// </ul>
        pub fn set_status(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.status = input;
            self
        }
        /// <p>The path to the Amazon S3 bucket where the report was stored on creation.</p>
        pub fn s3_location(mut self, input: impl Into<std::string::String>) -> Self {
            self.s3_location = Some(input.into());
            self
        }
        /// <p>The path to the Amazon S3 bucket where the report was stored on creation.</p>
        pub fn set_s3_location(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.s3_location = input;
            self
        }
        /// <p>The date and time that the report was started. </p>
        pub fn start_date(mut self, input: impl Into<std::string::String>) -> Self {
            self.start_date = Some(input.into());
            self
        }
        /// <p>The date and time that the report was started. </p>
        pub fn set_start_date(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.start_date = input;
            self
        }
        /// <p>Details of the common errors that all operations return.</p>
        pub fn error_message(mut self, input: impl Into<std::string::String>) -> Self {
            self.error_message = Some(input.into());
            self
        }
        /// <p>Details of the common errors that all operations return.</p>
        pub fn set_error_message(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.error_message = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeReportCreationOutput`](crate::output::DescribeReportCreationOutput).
        pub fn build(self) -> crate::output::DescribeReportCreationOutput {
            crate::output::DescribeReportCreationOutput {
                status: self.status,
                s3_location: self.s3_location,
                start_date: self.start_date,
                error_message: self.error_message,
            }
        }
    }
}
impl DescribeReportCreationOutput {
    /// Creates a new builder-style object to manufacture [`DescribeReportCreationOutput`](crate::output::DescribeReportCreationOutput).
    pub fn builder() -> crate::output::describe_report_creation_output::Builder {
        crate::output::describe_report_creation_output::Builder::default()
    }
}
