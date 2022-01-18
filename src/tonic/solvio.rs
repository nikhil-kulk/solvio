#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionInfoRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCollectionsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionDescription {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<CollectionInfo>,
    #[prost(double, tag = "2")]
    pub time: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCollectionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub collections: ::prost::alloc::vec::Vec<CollectionDescription>,
    #[prost(double, tag = "2")]
    pub time: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HnswConfigDiff {
    #[prost(uint64, optional, tag = "1")]
    pub m: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub ef_construct: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "3")]
    pub full_scan_threshold: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalConfigDiff {
    #[prost(uint64, optional, tag = "1")]
    pub wal_capacity_mb: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "2")]
    pub wal_segments_ahead: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptimizersConfigDiff {
    #[prost(double, optional, tag = "1")]
    pub deleted_threshold: ::core::option::Option<f64>,
    #[prost(uint64, optional, tag = "2")]
    pub vacuum_min_vector_number: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "3")]
    pub default_segment_number: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "4")]
    pub max_segment_size: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "5")]
    pub memmap_threshold: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "6")]
    pub indexing_threshold: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "7")]
    pub payload_indexing_threshold: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "8")]
    pub flush_interval_sec: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "9")]
    pub max_optimization_threads: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCollection {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub vector_size: u64,
    #[prost(enumeration = "Distance", tag = "3")]
    pub distance: i32,
    #[prost(message, optional, tag = "4")]
    pub hnsw_config: ::core::option::Option<HnswConfigDiff>,
    #[prost(message, optional, tag = "5")]
    pub wal_config: ::core::option::Option<WalConfigDiff>,
    #[prost(message, optional, tag = "6")]
    pub optimizers_config: ::core::option::Option<OptimizersConfigDiff>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCollection {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub optimizers_config: ::core::option::Option<OptimizersConfigDiff>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCollection {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionOperationResponse {
    #[prost(bool, tag = "1")]
    pub result: bool,
    #[prost(double, tag = "2")]
    pub time: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionParams {
    #[prost(uint64, tag = "1")]
    pub vector_size: u64,
    #[prost(enumeration = "Distance", tag = "2")]
    pub distance: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionConfig {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<CollectionParams>,
    #[prost(message, optional, tag = "2")]
    pub hnsw_config: ::core::option::Option<HnswConfigDiff>,
    #[prost(message, optional, tag = "3")]
    pub optimizer_config: ::core::option::Option<OptimizersConfigDiff>,
    #[prost(message, optional, tag = "4")]
    pub wal_config: ::core::option::Option<WalConfigDiff>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayloadSchemaInfo {
    #[prost(enumeration = "PayloadSchemaType", tag = "1")]
    pub data_type: i32,
    #[prost(bool, tag = "2")]
    pub indexed: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionInfo {
    #[prost(enumeration = "CollectionStatus", tag = "1")]
    pub status: i32,
    #[prost(uint64, tag = "2")]
    pub vectors_count: u64,
    #[prost(uint64, tag = "3")]
    pub segments_count: u64,
    #[prost(uint64, tag = "4")]
    pub disk_data_size: u64,
    #[prost(uint64, tag = "5")]
    pub ram_data_size: u64,
    #[prost(message, optional, tag = "6")]
    pub config: ::core::option::Option<CollectionConfig>,
    #[prost(map = "string, message", tag = "7")]
    pub payload_schema:
        ::std::collections::HashMap<::prost::alloc::string::String, PayloadSchemaInfo>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Distance {
    Cosine = 0,
    Euclid = 1,
    Dot = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CollectionStatus {
    Green = 0,
    Yellow = 1,
    Red = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PayloadSchemaType {
    Keyword = 0,
    Integer = 1,
    Float = 2,
    Geo = 3,
}
#[doc = r" Generated client implementations."]
pub mod collections_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct CollectionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CollectionsClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CollectionsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CollectionsClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            CollectionsClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCollectionInfoRequest>,
        ) -> Result<tonic::Response<super::GetCollectionInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/solvio.Collections/Get");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCollectionsRequest>,
        ) -> Result<tonic::Response<super::ListCollectionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/solvio.Collections/List");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCollection>,
        ) -> Result<tonic::Response<super::CollectionOperationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/solvio.Collections/Create");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCollection>,
        ) -> Result<tonic::Response<super::CollectionOperationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/solvio.Collections/Update");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCollection>,
        ) -> Result<tonic::Response<super::CollectionOperationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/solvio.Collections/Delete");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod collections_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CollectionsServer."]
    #[async_trait]
    pub trait Collections: Send + Sync + 'static {
        async fn get(
            &self,
            request: tonic::Request<super::GetCollectionInfoRequest>,
        ) -> Result<tonic::Response<super::GetCollectionInfoResponse>, tonic::Status>;
        async fn list(
            &self,
            request: tonic::Request<super::ListCollectionsRequest>,
        ) -> Result<tonic::Response<super::ListCollectionsResponse>, tonic::Status>;
        async fn create(
            &self,
            request: tonic::Request<super::CreateCollection>,
        ) -> Result<tonic::Response<super::CollectionOperationResponse>, tonic::Status>;
        async fn update(
            &self,
            request: tonic::Request<super::UpdateCollection>,
        ) -> Result<tonic::Response<super::CollectionOperationResponse>, tonic::Status>;
        async fn delete(
            &self,
            request: tonic::Request<super::DeleteCollection>,
        ) -> Result<tonic::Response<super::CollectionOperationResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct CollectionsServer<T: Collections> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Collections> CollectionsServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CollectionsServer<T>
    where
        T: Collections,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/solvio.Collections/Get" => {
                    #[allow(non_camel_case_types)]
                    struct GetSvc<T: Collections>(pub Arc<T>);
                    impl<T: Collections>
                        tonic::server::UnaryService<super::GetCollectionInfoRequest> for GetSvc<T>
                    {
                        type Response = super::GetCollectionInfoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCollectionInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/solvio.Collections/List" => {
                    #[allow(non_camel_case_types)]
                    struct ListSvc<T: Collections>(pub Arc<T>);
                    impl<T: Collections> tonic::server::UnaryService<super::ListCollectionsRequest> for ListSvc<T> {
                        type Response = super::ListCollectionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCollectionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/solvio.Collections/Create" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSvc<T: Collections>(pub Arc<T>);
                    impl<T: Collections> tonic::server::UnaryService<super::CreateCollection> for CreateSvc<T> {
                        type Response = super::CollectionOperationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCollection>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/solvio.Collections/Update" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSvc<T: Collections>(pub Arc<T>);
                    impl<T: Collections> tonic::server::UnaryService<super::UpdateCollection> for UpdateSvc<T> {
                        type Response = super::CollectionOperationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCollection>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/solvio.Collections/Delete" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSvc<T: Collections>(pub Arc<T>);
                    impl<T: Collections> tonic::server::UnaryService<super::DeleteCollection> for DeleteSvc<T> {
                        type Response = super::CollectionOperationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteCollection>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Collections> Clone for CollectionsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Collections> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Collections> tonic::transport::NamedService for CollectionsServer<T> {
        const NAME: &'static str = "solvio.Collections";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpsertPoints {
    #[prost(string, tag = "1")]
    pub collection: ::prost::alloc::string::String,
    #[prost(bool, optional, tag = "2")]
    pub wait: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "3")]
    pub points: ::prost::alloc::vec::Vec<PointStruct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointStruct {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(float, repeated, tag = "2")]
    pub vector: ::prost::alloc::vec::Vec<f32>,
    #[prost(map = "string, message", tag = "3")]
    pub payload: ::std::collections::HashMap<::prost::alloc::string::String, Payload>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payload {
    #[prost(message, optional, tag = "1")]
    pub keyword: ::core::option::Option<KeywordPayload>,
    #[prost(message, optional, tag = "2")]
    pub integer: ::core::option::Option<IntegerPayload>,
    #[prost(message, optional, tag = "3")]
    pub float: ::core::option::Option<FloatPayload>,
    #[prost(message, optional, tag = "4")]
    pub geo: ::core::option::Option<GeoPayload>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPayload {
    #[prost(string, repeated, tag = "1")]
    pub value: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntegerPayload {
    #[prost(int64, repeated, tag = "1")]
    pub value: ::prost::alloc::vec::Vec<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatPayload {
    #[prost(double, repeated, tag = "1")]
    pub value: ::prost::alloc::vec::Vec<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoPayload {
    #[prost(message, repeated, tag = "1")]
    pub value: ::prost::alloc::vec::Vec<GeoPoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoPoint {
    #[prost(double, tag = "1")]
    pub lon: f64,
    #[prost(double, tag = "2")]
    pub lat: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointsOperationResponse {
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<UpdateResult>,
    #[prost(double, tag = "2")]
    pub time: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResult {
    #[prost(uint64, tag = "1")]
    pub operation_id: u64,
    #[prost(enumeration = "UpdateStatus", tag = "2")]
    pub status: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UpdateStatus {
    Acknowledged = 0,
    Completed = 1,
}
#[doc = r" Generated client implementations."]
pub mod points_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct PointsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PointsClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PointsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PointsClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            PointsClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn upsert(
            &mut self,
            request: impl tonic::IntoRequest<super::UpsertPoints>,
        ) -> Result<tonic::Response<super::PointsOperationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/solvio.Points/Upsert");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod points_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PointsServer."]
    #[async_trait]
    pub trait Points: Send + Sync + 'static {
        async fn upsert(
            &self,
            request: tonic::Request<super::UpsertPoints>,
        ) -> Result<tonic::Response<super::PointsOperationResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PointsServer<T: Points> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Points> PointsServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PointsServer<T>
    where
        T: Points,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/solvio.Points/Upsert" => {
                    #[allow(non_camel_case_types)]
                    struct UpsertSvc<T: Points>(pub Arc<T>);
                    impl<T: Points> tonic::server::UnaryService<super::UpsertPoints> for UpsertSvc<T> {
                        type Response = super::PointsOperationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpsertPoints>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).upsert(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpsertSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Points> Clone for PointsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Points> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Points> tonic::transport::NamedService for PointsServer<T> {
        const NAME: &'static str = "solvio.Points";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckReply {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod solvio_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct SolvioClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SolvioClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SolvioClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SolvioClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            SolvioClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn health_check(
            &mut self,
            request: impl tonic::IntoRequest<super::HealthCheckRequest>,
        ) -> Result<tonic::Response<super::HealthCheckReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/solvio.Solvio/HealthCheck");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod solvio_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SolvioServer."]
    #[async_trait]
    pub trait Solvio: Send + Sync + 'static {
        async fn health_check(
            &self,
            request: tonic::Request<super::HealthCheckRequest>,
        ) -> Result<tonic::Response<super::HealthCheckReply>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SolvioServer<T: Solvio> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Solvio> SolvioServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SolvioServer<T>
    where
        T: Solvio,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/solvio.Solvio/HealthCheck" => {
                    #[allow(non_camel_case_types)]
                    struct HealthCheckSvc<T: Solvio>(pub Arc<T>);
                    impl<T: Solvio> tonic::server::UnaryService<super::HealthCheckRequest> for HealthCheckSvc<T> {
                        type Response = super::HealthCheckReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HealthCheckRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).health_check(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HealthCheckSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Solvio> Clone for SolvioServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Solvio> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Solvio> tonic::transport::NamedService for SolvioServer<T> {
        const NAME: &'static str = "solvio.Solvio";
    }
}
