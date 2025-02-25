// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A .csv file.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Csv {
    /// <p>The column names specified in the .csv file.</p>
    pub column_names: ::std::option::Option<::std::vec::Vec<crate::types::ColumnName>>,
}
impl Csv {
    /// <p>The column names specified in the .csv file.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.column_names.is_none()`.
    pub fn column_names(&self) -> &[crate::types::ColumnName] {
        self.column_names.as_deref().unwrap_or_default()
    }
}
impl Csv {
    /// Creates a new builder-style object to manufacture [`Csv`](crate::types::Csv).
    pub fn builder() -> crate::types::builders::CsvBuilder {
        crate::types::builders::CsvBuilder::default()
    }
}

/// A builder for [`Csv`](crate::types::Csv).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CsvBuilder {
    pub(crate) column_names: ::std::option::Option<::std::vec::Vec<crate::types::ColumnName>>,
}
impl CsvBuilder {
    /// Appends an item to `column_names`.
    ///
    /// To override the contents of this collection use [`set_column_names`](Self::set_column_names).
    ///
    /// <p>The column names specified in the .csv file.</p>
    pub fn column_names(mut self, input: crate::types::ColumnName) -> Self {
        let mut v = self.column_names.unwrap_or_default();
        v.push(input);
        self.column_names = ::std::option::Option::Some(v);
        self
    }
    /// <p>The column names specified in the .csv file.</p>
    pub fn set_column_names(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ColumnName>>) -> Self {
        self.column_names = input;
        self
    }
    /// <p>The column names specified in the .csv file.</p>
    pub fn get_column_names(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ColumnName>> {
        &self.column_names
    }
    /// Consumes the builder and constructs a [`Csv`](crate::types::Csv).
    pub fn build(self) -> crate::types::Csv {
        crate::types::Csv {
            column_names: self.column_names,
        }
    }
}
