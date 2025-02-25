// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetSchemaByDefinition`](crate::operation::get_schema_by_definition::builders::GetSchemaByDefinitionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`schema_id(SchemaId)`](crate::operation::get_schema_by_definition::builders::GetSchemaByDefinitionFluentBuilder::schema_id) / [`set_schema_id(Option<SchemaId>)`](crate::operation::get_schema_by_definition::builders::GetSchemaByDefinitionFluentBuilder::set_schema_id):<br>required: **true**<br><p>This is a wrapper structure to contain schema identity fields. The structure contains:</p>  <ul>   <li> <p>SchemaId$SchemaArn: The Amazon Resource Name (ARN) of the schema. One of <code>SchemaArn</code> or <code>SchemaName</code> has to be provided.</p> </li>   <li> <p>SchemaId$SchemaName: The name of the schema. One of <code>SchemaArn</code> or <code>SchemaName</code> has to be provided.</p> </li>  </ul><br>
    ///   - [`schema_definition(impl Into<String>)`](crate::operation::get_schema_by_definition::builders::GetSchemaByDefinitionFluentBuilder::schema_definition) / [`set_schema_definition(Option<String>)`](crate::operation::get_schema_by_definition::builders::GetSchemaByDefinitionFluentBuilder::set_schema_definition):<br>required: **true**<br><p>The definition of the schema for which schema details are required.</p><br>
    /// - On success, responds with [`GetSchemaByDefinitionOutput`](crate::operation::get_schema_by_definition::GetSchemaByDefinitionOutput) with field(s):
    ///   - [`schema_version_id(Option<String>)`](crate::operation::get_schema_by_definition::GetSchemaByDefinitionOutput::schema_version_id): <p>The schema ID of the schema version.</p>
    ///   - [`schema_arn(Option<String>)`](crate::operation::get_schema_by_definition::GetSchemaByDefinitionOutput::schema_arn): <p>The Amazon Resource Name (ARN) of the schema.</p>
    ///   - [`data_format(Option<DataFormat>)`](crate::operation::get_schema_by_definition::GetSchemaByDefinitionOutput::data_format): <p>The data format of the schema definition. Currently <code>AVRO</code>, <code>JSON</code> and <code>PROTOBUF</code> are supported.</p>
    ///   - [`status(Option<SchemaVersionStatus>)`](crate::operation::get_schema_by_definition::GetSchemaByDefinitionOutput::status): <p>The status of the schema version.</p>
    ///   - [`created_time(Option<String>)`](crate::operation::get_schema_by_definition::GetSchemaByDefinitionOutput::created_time): <p>The date and time the schema was created.</p>
    /// - On failure, responds with [`SdkError<GetSchemaByDefinitionError>`](crate::operation::get_schema_by_definition::GetSchemaByDefinitionError)
    pub fn get_schema_by_definition(&self) -> crate::operation::get_schema_by_definition::builders::GetSchemaByDefinitionFluentBuilder {
        crate::operation::get_schema_by_definition::builders::GetSchemaByDefinitionFluentBuilder::new(self.handle.clone())
    }
}
