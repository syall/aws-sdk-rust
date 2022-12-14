// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdateReportDefinitionOutput {
    /// <p>ID of the report.</p>
    #[doc(hidden)]
    pub report_id: std::option::Option<std::string::String>,
}
impl UpdateReportDefinitionOutput {
    /// <p>ID of the report.</p>
    pub fn report_id(&self) -> std::option::Option<&str> {
        self.report_id.as_deref()
    }
}
/// See [`UpdateReportDefinitionOutput`](crate::output::UpdateReportDefinitionOutput).
pub mod update_report_definition_output {

    /// A builder for [`UpdateReportDefinitionOutput`](crate::output::UpdateReportDefinitionOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) report_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>ID of the report.</p>
        pub fn report_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.report_id = Some(input.into());
            self
        }
        /// <p>ID of the report.</p>
        pub fn set_report_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.report_id = input;
            self
        }
        /// Consumes the builder and constructs a [`UpdateReportDefinitionOutput`](crate::output::UpdateReportDefinitionOutput).
        pub fn build(self) -> crate::output::UpdateReportDefinitionOutput {
            crate::output::UpdateReportDefinitionOutput {
                report_id: self.report_id,
            }
        }
    }
}
impl UpdateReportDefinitionOutput {
    /// Creates a new builder-style object to manufacture [`UpdateReportDefinitionOutput`](crate::output::UpdateReportDefinitionOutput).
    pub fn builder() -> crate::output::update_report_definition_output::Builder {
        crate::output::update_report_definition_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PutReportDefinitionOutput {
    /// <p>ID of the report.</p>
    #[doc(hidden)]
    pub report_id: std::option::Option<std::string::String>,
}
impl PutReportDefinitionOutput {
    /// <p>ID of the report.</p>
    pub fn report_id(&self) -> std::option::Option<&str> {
        self.report_id.as_deref()
    }
}
/// See [`PutReportDefinitionOutput`](crate::output::PutReportDefinitionOutput).
pub mod put_report_definition_output {

    /// A builder for [`PutReportDefinitionOutput`](crate::output::PutReportDefinitionOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) report_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>ID of the report.</p>
        pub fn report_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.report_id = Some(input.into());
            self
        }
        /// <p>ID of the report.</p>
        pub fn set_report_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.report_id = input;
            self
        }
        /// Consumes the builder and constructs a [`PutReportDefinitionOutput`](crate::output::PutReportDefinitionOutput).
        pub fn build(self) -> crate::output::PutReportDefinitionOutput {
            crate::output::PutReportDefinitionOutput {
                report_id: self.report_id,
            }
        }
    }
}
impl PutReportDefinitionOutput {
    /// Creates a new builder-style object to manufacture [`PutReportDefinitionOutput`](crate::output::PutReportDefinitionOutput).
    pub fn builder() -> crate::output::put_report_definition_output::Builder {
        crate::output::put_report_definition_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListReportDefinitionsOutput {
    /// <p>The retrieved reports.</p>
    #[doc(hidden)]
    pub report_definitions: std::option::Option<std::vec::Vec<crate::model::ReportDefinition>>,
    /// <p>The value of the next token, if it exists. Null if there are no more results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListReportDefinitionsOutput {
    /// <p>The retrieved reports.</p>
    pub fn report_definitions(&self) -> std::option::Option<&[crate::model::ReportDefinition]> {
        self.report_definitions.as_deref()
    }
    /// <p>The value of the next token, if it exists. Null if there are no more results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
/// See [`ListReportDefinitionsOutput`](crate::output::ListReportDefinitionsOutput).
pub mod list_report_definitions_output {

    /// A builder for [`ListReportDefinitionsOutput`](crate::output::ListReportDefinitionsOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) report_definitions:
            std::option::Option<std::vec::Vec<crate::model::ReportDefinition>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `report_definitions`.
        ///
        /// To override the contents of this collection use [`set_report_definitions`](Self::set_report_definitions).
        ///
        /// <p>The retrieved reports.</p>
        pub fn report_definitions(mut self, input: crate::model::ReportDefinition) -> Self {
            let mut v = self.report_definitions.unwrap_or_default();
            v.push(input);
            self.report_definitions = Some(v);
            self
        }
        /// <p>The retrieved reports.</p>
        pub fn set_report_definitions(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ReportDefinition>>,
        ) -> Self {
            self.report_definitions = input;
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
        /// Consumes the builder and constructs a [`ListReportDefinitionsOutput`](crate::output::ListReportDefinitionsOutput).
        pub fn build(self) -> crate::output::ListReportDefinitionsOutput {
            crate::output::ListReportDefinitionsOutput {
                report_definitions: self.report_definitions,
                next_token: self.next_token,
            }
        }
    }
}
impl ListReportDefinitionsOutput {
    /// Creates a new builder-style object to manufacture [`ListReportDefinitionsOutput`](crate::output::ListReportDefinitionsOutput).
    pub fn builder() -> crate::output::list_report_definitions_output::Builder {
        crate::output::list_report_definitions_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ImportApplicationUsageOutput {
    /// <p>ID of the import request.</p>
    #[doc(hidden)]
    pub import_id: std::option::Option<std::string::String>,
}
impl ImportApplicationUsageOutput {
    /// <p>ID of the import request.</p>
    pub fn import_id(&self) -> std::option::Option<&str> {
        self.import_id.as_deref()
    }
}
/// See [`ImportApplicationUsageOutput`](crate::output::ImportApplicationUsageOutput).
pub mod import_application_usage_output {

    /// A builder for [`ImportApplicationUsageOutput`](crate::output::ImportApplicationUsageOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) import_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>ID of the import request.</p>
        pub fn import_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.import_id = Some(input.into());
            self
        }
        /// <p>ID of the import request.</p>
        pub fn set_import_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.import_id = input;
            self
        }
        /// Consumes the builder and constructs a [`ImportApplicationUsageOutput`](crate::output::ImportApplicationUsageOutput).
        pub fn build(self) -> crate::output::ImportApplicationUsageOutput {
            crate::output::ImportApplicationUsageOutput {
                import_id: self.import_id,
            }
        }
    }
}
impl ImportApplicationUsageOutput {
    /// Creates a new builder-style object to manufacture [`ImportApplicationUsageOutput`](crate::output::ImportApplicationUsageOutput).
    pub fn builder() -> crate::output::import_application_usage_output::Builder {
        crate::output::import_application_usage_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetReportDefinitionOutput {
    /// <p>ID of the report retrieved.</p>
    #[doc(hidden)]
    pub report_id: std::option::Option<std::string::String>,
    /// <p>Description of the report.</p>
    #[doc(hidden)]
    pub report_description: std::option::Option<std::string::String>,
    /// <p>Cadence used to generate the report.</p>
    #[doc(hidden)]
    pub report_frequency: std::option::Option<crate::model::ReportFrequency>,
    /// <p>Format of the generated report.</p>
    #[doc(hidden)]
    pub format: std::option::Option<crate::model::Format>,
    /// <p>Amazon Simple Storage Service (Amazon S3) location where the report is uploaded.</p>
    #[doc(hidden)]
    pub destination_s3_location: std::option::Option<crate::model::S3Location>,
    /// <p>Timestamp (milliseconds) when this report definition was created.</p>
    #[doc(hidden)]
    pub created_at: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>Timestamp (milliseconds) when this report definition was last updated.</p>
    #[doc(hidden)]
    pub last_updated: std::option::Option<aws_smithy_types::DateTime>,
}
impl GetReportDefinitionOutput {
    /// <p>ID of the report retrieved.</p>
    pub fn report_id(&self) -> std::option::Option<&str> {
        self.report_id.as_deref()
    }
    /// <p>Description of the report.</p>
    pub fn report_description(&self) -> std::option::Option<&str> {
        self.report_description.as_deref()
    }
    /// <p>Cadence used to generate the report.</p>
    pub fn report_frequency(&self) -> std::option::Option<&crate::model::ReportFrequency> {
        self.report_frequency.as_ref()
    }
    /// <p>Format of the generated report.</p>
    pub fn format(&self) -> std::option::Option<&crate::model::Format> {
        self.format.as_ref()
    }
    /// <p>Amazon Simple Storage Service (Amazon S3) location where the report is uploaded.</p>
    pub fn destination_s3_location(&self) -> std::option::Option<&crate::model::S3Location> {
        self.destination_s3_location.as_ref()
    }
    /// <p>Timestamp (milliseconds) when this report definition was created.</p>
    pub fn created_at(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
    /// <p>Timestamp (milliseconds) when this report definition was last updated.</p>
    pub fn last_updated(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.last_updated.as_ref()
    }
}
/// See [`GetReportDefinitionOutput`](crate::output::GetReportDefinitionOutput).
pub mod get_report_definition_output {

    /// A builder for [`GetReportDefinitionOutput`](crate::output::GetReportDefinitionOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) report_id: std::option::Option<std::string::String>,
        pub(crate) report_description: std::option::Option<std::string::String>,
        pub(crate) report_frequency: std::option::Option<crate::model::ReportFrequency>,
        pub(crate) format: std::option::Option<crate::model::Format>,
        pub(crate) destination_s3_location: std::option::Option<crate::model::S3Location>,
        pub(crate) created_at: std::option::Option<aws_smithy_types::DateTime>,
        pub(crate) last_updated: std::option::Option<aws_smithy_types::DateTime>,
    }
    impl Builder {
        /// <p>ID of the report retrieved.</p>
        pub fn report_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.report_id = Some(input.into());
            self
        }
        /// <p>ID of the report retrieved.</p>
        pub fn set_report_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.report_id = input;
            self
        }
        /// <p>Description of the report.</p>
        pub fn report_description(mut self, input: impl Into<std::string::String>) -> Self {
            self.report_description = Some(input.into());
            self
        }
        /// <p>Description of the report.</p>
        pub fn set_report_description(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.report_description = input;
            self
        }
        /// <p>Cadence used to generate the report.</p>
        pub fn report_frequency(mut self, input: crate::model::ReportFrequency) -> Self {
            self.report_frequency = Some(input);
            self
        }
        /// <p>Cadence used to generate the report.</p>
        pub fn set_report_frequency(
            mut self,
            input: std::option::Option<crate::model::ReportFrequency>,
        ) -> Self {
            self.report_frequency = input;
            self
        }
        /// <p>Format of the generated report.</p>
        pub fn format(mut self, input: crate::model::Format) -> Self {
            self.format = Some(input);
            self
        }
        /// <p>Format of the generated report.</p>
        pub fn set_format(mut self, input: std::option::Option<crate::model::Format>) -> Self {
            self.format = input;
            self
        }
        /// <p>Amazon Simple Storage Service (Amazon S3) location where the report is uploaded.</p>
        pub fn destination_s3_location(mut self, input: crate::model::S3Location) -> Self {
            self.destination_s3_location = Some(input);
            self
        }
        /// <p>Amazon Simple Storage Service (Amazon S3) location where the report is uploaded.</p>
        pub fn set_destination_s3_location(
            mut self,
            input: std::option::Option<crate::model::S3Location>,
        ) -> Self {
            self.destination_s3_location = input;
            self
        }
        /// <p>Timestamp (milliseconds) when this report definition was created.</p>
        pub fn created_at(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.created_at = Some(input);
            self
        }
        /// <p>Timestamp (milliseconds) when this report definition was created.</p>
        pub fn set_created_at(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.created_at = input;
            self
        }
        /// <p>Timestamp (milliseconds) when this report definition was last updated.</p>
        pub fn last_updated(mut self, input: aws_smithy_types::DateTime) -> Self {
            self.last_updated = Some(input);
            self
        }
        /// <p>Timestamp (milliseconds) when this report definition was last updated.</p>
        pub fn set_last_updated(
            mut self,
            input: std::option::Option<aws_smithy_types::DateTime>,
        ) -> Self {
            self.last_updated = input;
            self
        }
        /// Consumes the builder and constructs a [`GetReportDefinitionOutput`](crate::output::GetReportDefinitionOutput).
        pub fn build(self) -> crate::output::GetReportDefinitionOutput {
            crate::output::GetReportDefinitionOutput {
                report_id: self.report_id,
                report_description: self.report_description,
                report_frequency: self.report_frequency,
                format: self.format,
                destination_s3_location: self.destination_s3_location,
                created_at: self.created_at,
                last_updated: self.last_updated,
            }
        }
    }
}
impl GetReportDefinitionOutput {
    /// Creates a new builder-style object to manufacture [`GetReportDefinitionOutput`](crate::output::GetReportDefinitionOutput).
    pub fn builder() -> crate::output::get_report_definition_output::Builder {
        crate::output::get_report_definition_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteReportDefinitionOutput {
    /// <p>ID of the report that was deleted.</p>
    #[doc(hidden)]
    pub report_id: std::option::Option<std::string::String>,
}
impl DeleteReportDefinitionOutput {
    /// <p>ID of the report that was deleted.</p>
    pub fn report_id(&self) -> std::option::Option<&str> {
        self.report_id.as_deref()
    }
}
/// See [`DeleteReportDefinitionOutput`](crate::output::DeleteReportDefinitionOutput).
pub mod delete_report_definition_output {

    /// A builder for [`DeleteReportDefinitionOutput`](crate::output::DeleteReportDefinitionOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) report_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>ID of the report that was deleted.</p>
        pub fn report_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.report_id = Some(input.into());
            self
        }
        /// <p>ID of the report that was deleted.</p>
        pub fn set_report_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.report_id = input;
            self
        }
        /// Consumes the builder and constructs a [`DeleteReportDefinitionOutput`](crate::output::DeleteReportDefinitionOutput).
        pub fn build(self) -> crate::output::DeleteReportDefinitionOutput {
            crate::output::DeleteReportDefinitionOutput {
                report_id: self.report_id,
            }
        }
    }
}
impl DeleteReportDefinitionOutput {
    /// Creates a new builder-style object to manufacture [`DeleteReportDefinitionOutput`](crate::output::DeleteReportDefinitionOutput).
    pub fn builder() -> crate::output::delete_report_definition_output::Builder {
        crate::output::delete_report_definition_output::Builder::default()
    }
}
