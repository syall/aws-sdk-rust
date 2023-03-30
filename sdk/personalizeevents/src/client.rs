// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[doc(inline)]
                pub use aws_smithy_client::Builder;
#[derive(Debug)]
            pub(crate) struct Handle {
                pub(crate) client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>,
                pub(crate) conf: crate::Config,
            }

            /// Client for Amazon Personalize Events
                    ///
                    /// Client for invoking operations on Amazon Personalize Events. Each operation on Amazon Personalize Events is a method on this
                    /// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
                        /// # Examples
                        /// **Constructing a client and invoking an operation**
                        /// ```rust,no_run
                        /// # async fn docs() {
                        ///     // create a shared configuration. This can be used & shared between multiple service clients.
                        ///     let shared_config = aws_config::load_from_env().await;
                        ///     let client = aws_sdk_personalizeevents::Client::new(&shared_config);
                        ///     // invoke an operation
                        ///     /* let rsp = client
                        ///         .<operation_name>().
                        ///         .<param>("some value")
                        ///         .send().await; */
                        /// # }
                        /// ```
                        /// **Constructing a client with custom configuration**
                        /// ```rust,no_run
                        /// use aws_config::retry::RetryConfig;
                        /// # async fn docs() {
                        /// let shared_config = aws_config::load_from_env().await;
                        /// let config = aws_sdk_personalizeevents::config::Builder::from(&shared_config)
                        ///   .retry_config(RetryConfig::disabled())
                        ///   .build();
                        /// let client = aws_sdk_personalizeevents::Client::from_conf(config);
                        /// # }
            #[derive(std::fmt::Debug)]
            pub struct Client {
                handle: std::sync::Arc<Handle>
            }

            impl std::clone::Clone for Client {
                fn clone(&self) -> Self {
                    Self { handle: self.handle.clone() }
                }
            }

            impl From<aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>> for Client {
                fn from(client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>) -> Self {
                    Self::with_config(client, crate::Config::builder().build())
                }
            }

            impl Client {
                /// Creates a client with the given service configuration.
                pub fn with_config(client: aws_smithy_client::Client<aws_smithy_client::erase::DynConnector, aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>>, conf: crate::Config) -> Self {
                    Self {
                        handle: std::sync::Arc::new(Handle {
                            client,
                            conf,
                        })
                    }
                }

                /// Returns the client's configuration.
                pub fn conf(&self) -> &crate::Config {
                    &self.handle.conf
                }
            }
impl Client  {
    /// Constructs a fluent builder for the [`PutEvents`](crate::client::fluent_builders::PutEvents) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`tracking_id(impl Into<String>)`](crate::client::fluent_builders::PutEvents::tracking_id) / [`set_tracking_id(Option<String>)`](crate::client::fluent_builders::PutEvents::set_tracking_id): <p>The tracking ID for the event. The ID is generated by a call to the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_CreateEventTracker.html">CreateEventTracker</a> API.</p>
    ///   - [`user_id(impl Into<String>)`](crate::client::fluent_builders::PutEvents::user_id) / [`set_user_id(Option<String>)`](crate::client::fluent_builders::PutEvents::set_user_id): <p>The user associated with the event.</p>
    ///   - [`session_id(impl Into<String>)`](crate::client::fluent_builders::PutEvents::session_id) / [`set_session_id(Option<String>)`](crate::client::fluent_builders::PutEvents::set_session_id): <p>The session ID associated with the user's visit. Your application generates the sessionId when a user first visits your website or uses your application. Amazon Personalize uses the sessionId to associate events with the user before they log in. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/recording-events.html">Recording Events</a>.</p>
    ///   - [`event_list(Vec<Event>)`](crate::client::fluent_builders::PutEvents::event_list) / [`set_event_list(Option<Vec<Event>>)`](crate::client::fluent_builders::PutEvents::set_event_list): <p>A list of event data from the session.</p>
                        /// - On success, responds with [`PutEventsOutput`](crate::output::PutEventsOutput)
                        
                        /// - On failure, responds with [`SdkError<PutEventsError>`](crate::error::PutEventsError)
    pub fn put_events(&self) -> fluent_builders::PutEvents {
                            fluent_builders::PutEvents::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`PutItems`](crate::client::fluent_builders::PutItems) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`dataset_arn(impl Into<String>)`](crate::client::fluent_builders::PutItems::dataset_arn) / [`set_dataset_arn(Option<String>)`](crate::client::fluent_builders::PutItems::set_dataset_arn): <p>The Amazon Resource Name (ARN) of the Items dataset you are adding the item or items to.</p>
    ///   - [`items(Vec<Item>)`](crate::client::fluent_builders::PutItems::items) / [`set_items(Option<Vec<Item>>)`](crate::client::fluent_builders::PutItems::set_items): <p>A list of item data.</p>
                        /// - On success, responds with [`PutItemsOutput`](crate::output::PutItemsOutput)
                        
                        /// - On failure, responds with [`SdkError<PutItemsError>`](crate::error::PutItemsError)
    pub fn put_items(&self) -> fluent_builders::PutItems {
                            fluent_builders::PutItems::new(self.handle.clone())
                        }
    /// Constructs a fluent builder for the [`PutUsers`](crate::client::fluent_builders::PutUsers) operation.
                        ///
                        /// - The fluent builder is configurable:
                        ///   - [`dataset_arn(impl Into<String>)`](crate::client::fluent_builders::PutUsers::dataset_arn) / [`set_dataset_arn(Option<String>)`](crate::client::fluent_builders::PutUsers::set_dataset_arn): <p>The Amazon Resource Name (ARN) of the Users dataset you are adding the user or users to.</p>
    ///   - [`users(Vec<User>)`](crate::client::fluent_builders::PutUsers::users) / [`set_users(Option<Vec<User>>)`](crate::client::fluent_builders::PutUsers::set_users): <p>A list of user data.</p>
                        /// - On success, responds with [`PutUsersOutput`](crate::output::PutUsersOutput)
                        
                        /// - On failure, responds with [`SdkError<PutUsersError>`](crate::error::PutUsersError)
    pub fn put_users(&self) -> fluent_builders::PutUsers {
                            fluent_builders::PutUsers::new(self.handle.clone())
                        }
}
pub mod fluent_builders {
    
    //! Utilities to ergonomically construct a request to the service.
    //! 
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    /// Fluent builder constructing a request to `PutEvents`.
                        ///
    /// <p>Records user interaction event data. For more information see <a href="https://docs.aws.amazon.com/personalize/latest/dg/recording-events.html">Recording Events</a>.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct PutEvents {
                            handle: std::sync::Arc<super::Handle>,
                            inner: crate::input::put_events_input::Builder
                        }
    impl PutEvents  {
        /// Creates a new `PutEvents`.
                                pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
                                    Self { handle, inner: Default::default() }
                                }
        
                                /// Consume this builder, creating a customizable operation that can be modified before being
                                /// sent. The operation's inner [http::Request] can be modified as well.
                                pub async fn customize(self) -> std::result::Result<
                                    crate::operation::customize::CustomizableOperation<crate::operation::PutEvents, aws_http::retry::AwsResponseRetryClassifier,>,
                                    aws_smithy_http::result::SdkError<crate::error::PutEventsError>
                                >  {
                                    let handle = self.handle.clone();
                                    let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                                        .make_operation(&handle.conf)
                                        .await
                                        .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                                    Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                                }
        
                                /// Sends the request and returns the response.
                                ///
                                /// If an error occurs, an `SdkError` will be returned with additional details that
                                /// can be matched against.
                                ///
                                /// By default, any retryable failures will be retried twice. Retry behavior
                                /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                                /// set when configuring the client.
                                pub async fn send(self) -> std::result::Result<crate::output::PutEventsOutput, aws_smithy_http::result::SdkError<crate::error::PutEventsError>>
                                 {
                                    let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                                        .make_operation(&self.handle.conf)
                                        .await
                                        .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                                    self.handle.client.call(op).await
                                }
        /// <p>The tracking ID for the event. The ID is generated by a call to the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_CreateEventTracker.html">CreateEventTracker</a> API.</p>
        pub fn tracking_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.tracking_id(input.into());
            self
        }
        /// <p>The tracking ID for the event. The ID is generated by a call to the <a href="https://docs.aws.amazon.com/personalize/latest/dg/API_CreateEventTracker.html">CreateEventTracker</a> API.</p>
        pub fn set_tracking_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_tracking_id(input);
            self
        }
        /// <p>The user associated with the event.</p>
        pub fn user_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.user_id(input.into());
            self
        }
        /// <p>The user associated with the event.</p>
        pub fn set_user_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_user_id(input);
            self
        }
        /// <p>The session ID associated with the user's visit. Your application generates the sessionId when a user first visits your website or uses your application. Amazon Personalize uses the sessionId to associate events with the user before they log in. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/recording-events.html">Recording Events</a>.</p>
        pub fn session_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.session_id(input.into());
            self
        }
        /// <p>The session ID associated with the user's visit. Your application generates the sessionId when a user first visits your website or uses your application. Amazon Personalize uses the sessionId to associate events with the user before they log in. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/recording-events.html">Recording Events</a>.</p>
        pub fn set_session_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_session_id(input);
            self
        }
        /// Appends an item to `eventList`.
        ///
        /// To override the contents of this collection use [`set_event_list`](Self::set_event_list).
        ///
        /// <p>A list of event data from the session.</p>
        pub fn event_list(mut self, input: crate::model::Event) -> Self {
            self.inner = self.inner.event_list(input);
            self
        }
        /// <p>A list of event data from the session.</p>
        pub fn set_event_list(mut self, input: std::option::Option<std::vec::Vec<crate::model::Event>>) -> Self {
            self.inner = self.inner.set_event_list(input);
            self
        }
    }
    /// Fluent builder constructing a request to `PutItems`.
                        ///
    /// <p>Adds one or more items to an Items dataset. For more information see <a href="https://docs.aws.amazon.com/personalize/latest/dg/importing-items.html">Importing Items Incrementally</a>. </p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct PutItems {
                            handle: std::sync::Arc<super::Handle>,
                            inner: crate::input::put_items_input::Builder
                        }
    impl PutItems  {
        /// Creates a new `PutItems`.
                                pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
                                    Self { handle, inner: Default::default() }
                                }
        
                                /// Consume this builder, creating a customizable operation that can be modified before being
                                /// sent. The operation's inner [http::Request] can be modified as well.
                                pub async fn customize(self) -> std::result::Result<
                                    crate::operation::customize::CustomizableOperation<crate::operation::PutItems, aws_http::retry::AwsResponseRetryClassifier,>,
                                    aws_smithy_http::result::SdkError<crate::error::PutItemsError>
                                >  {
                                    let handle = self.handle.clone();
                                    let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                                        .make_operation(&handle.conf)
                                        .await
                                        .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                                    Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                                }
        
                                /// Sends the request and returns the response.
                                ///
                                /// If an error occurs, an `SdkError` will be returned with additional details that
                                /// can be matched against.
                                ///
                                /// By default, any retryable failures will be retried twice. Retry behavior
                                /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                                /// set when configuring the client.
                                pub async fn send(self) -> std::result::Result<crate::output::PutItemsOutput, aws_smithy_http::result::SdkError<crate::error::PutItemsError>>
                                 {
                                    let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                                        .make_operation(&self.handle.conf)
                                        .await
                                        .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                                    self.handle.client.call(op).await
                                }
        /// <p>The Amazon Resource Name (ARN) of the Items dataset you are adding the item or items to.</p>
        pub fn dataset_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.dataset_arn(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the Items dataset you are adding the item or items to.</p>
        pub fn set_dataset_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_dataset_arn(input);
            self
        }
        /// Appends an item to `items`.
        ///
        /// To override the contents of this collection use [`set_items`](Self::set_items).
        ///
        /// <p>A list of item data.</p>
        pub fn items(mut self, input: crate::model::Item) -> Self {
            self.inner = self.inner.items(input);
            self
        }
        /// <p>A list of item data.</p>
        pub fn set_items(mut self, input: std::option::Option<std::vec::Vec<crate::model::Item>>) -> Self {
            self.inner = self.inner.set_items(input);
            self
        }
    }
    /// Fluent builder constructing a request to `PutUsers`.
                        ///
    /// <p>Adds one or more users to a Users dataset. For more information see <a href="https://docs.aws.amazon.com/personalize/latest/dg/importing-users.html">Importing Users Incrementally</a>.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct PutUsers {
                            handle: std::sync::Arc<super::Handle>,
                            inner: crate::input::put_users_input::Builder
                        }
    impl PutUsers  {
        /// Creates a new `PutUsers`.
                                pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
                                    Self { handle, inner: Default::default() }
                                }
        
                                /// Consume this builder, creating a customizable operation that can be modified before being
                                /// sent. The operation's inner [http::Request] can be modified as well.
                                pub async fn customize(self) -> std::result::Result<
                                    crate::operation::customize::CustomizableOperation<crate::operation::PutUsers, aws_http::retry::AwsResponseRetryClassifier,>,
                                    aws_smithy_http::result::SdkError<crate::error::PutUsersError>
                                >  {
                                    let handle = self.handle.clone();
                                    let operation = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                                        .make_operation(&handle.conf)
                                        .await
                                        .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                                    Ok(crate::operation::customize::CustomizableOperation { handle, operation })
                                }
        
                                /// Sends the request and returns the response.
                                ///
                                /// If an error occurs, an `SdkError` will be returned with additional details that
                                /// can be matched against.
                                ///
                                /// By default, any retryable failures will be retried twice. Retry behavior
                                /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                                /// set when configuring the client.
                                pub async fn send(self) -> std::result::Result<crate::output::PutUsersOutput, aws_smithy_http::result::SdkError<crate::error::PutUsersError>>
                                 {
                                    let op = self.inner.build().map_err(aws_smithy_http::result::SdkError::construction_failure)?
                                        .make_operation(&self.handle.conf)
                                        .await
                                        .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
                                    self.handle.client.call(op).await
                                }
        /// <p>The Amazon Resource Name (ARN) of the Users dataset you are adding the user or users to.</p>
        pub fn dataset_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.dataset_arn(input.into());
            self
        }
        /// <p>The Amazon Resource Name (ARN) of the Users dataset you are adding the user or users to.</p>
        pub fn set_dataset_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_dataset_arn(input);
            self
        }
        /// Appends an item to `users`.
        ///
        /// To override the contents of this collection use [`set_users`](Self::set_users).
        ///
        /// <p>A list of user data.</p>
        pub fn users(mut self, input: crate::model::User) -> Self {
            self.inner = self.inner.users(input);
            self
        }
        /// <p>A list of user data.</p>
        pub fn set_users(mut self, input: std::option::Option<std::vec::Vec<crate::model::User>>) -> Self {
            self.inner = self.inner.set_users(input);
            self
        }
    }
    
    
}

impl Client {
    /// Creates a new client from an [SDK Config](aws_types::sdk_config::SdkConfig).
                    ///
                    /// # Panics
                    ///
                    /// - This method will panic if the `sdk_config` is missing an async sleep implementation. If you experience this panic, set
                    ///     the `sleep_impl` on the Config passed into this function to fix it.
                    /// - This method will panic if the `sdk_config` is missing an HTTP connector. If you experience this panic, set the
                    ///     `http_connector` on the Config passed into this function to fix it.
                    pub fn new(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
                        Self::from_conf(sdk_config.into())
                    }
    
                    /// Creates a new client from the service [`Config`](crate::Config).
                    ///
                    /// # Panics
                    ///
                    /// - This method will panic if the `conf` is missing an async sleep implementation. If you experience this panic, set
                    ///     the `sleep_impl` on the Config passed into this function to fix it.
                    /// - This method will panic if the `conf` is missing an HTTP connector. If you experience this panic, set the
                    ///     `http_connector` on the Config passed into this function to fix it.
                    pub fn from_conf(conf: crate::Config) -> Self {
                        let retry_config = conf.retry_config().cloned().unwrap_or_else(aws_smithy_types::retry::RetryConfig::disabled);
                        let timeout_config = conf.timeout_config().cloned().unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
                        let sleep_impl = conf.sleep_impl();
                        if (retry_config.has_retry() || timeout_config.has_timeouts()) && sleep_impl.is_none() {
                            panic!("An async sleep implementation is required for retries or timeouts to work. \
                                    Set the `sleep_impl` on the Config passed into this function to fix this panic.");
                        }
    
                        let connector = conf.http_connector().and_then(|c| {
                            let timeout_config = conf
                                .timeout_config()
                                .cloned()
                                .unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
                            let connector_settings = aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(
                                &timeout_config,
                            );
                            c.connector(&connector_settings, conf.sleep_impl())
                        });
    
                        let builder = aws_smithy_client::Builder::new();
    
                        let builder = match connector {
                            // Use provided connector
                            Some(c) => builder.connector(c),
                            None =>{
                                #[cfg(any(feature = "rustls", feature = "native-tls"))]
                                {
                                    // Use default connector based on enabled features
                                    builder.dyn_https_connector(aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(&timeout_config))
                                }
                                #[cfg(not(any(feature = "rustls", feature = "native-tls")))]
                                {
                                    panic!("No HTTP connector was available. Enable the `rustls` or `native-tls` crate feature or set a connector to fix this.");
                                }
                            }
                        };
                        let mut builder = builder
                            .middleware(aws_smithy_client::erase::DynMiddleware::new(crate::middleware::DefaultMiddleware::new()))
                            .retry_config(retry_config.into())
                            .operation_timeout_config(timeout_config.into());
                        builder.set_sleep_impl(sleep_impl);
                        let client = builder.build();
    
                        Self { handle: std::sync::Arc::new(Handle { client, conf }) }
                    }
}

