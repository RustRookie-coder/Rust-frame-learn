use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, ::prost::Message, Clone)]
pub struct Msg {
    /// must have
    #[prost(string, tag = "1")]
    pub send_id: String,
    /// must have
    #[prost(string, tag = "2")]
    pub receiver_id: String,
    /// must have
    #[prost(string, tag = "3")]
    pub local_id: String,
    #[prost(string, tag = "4")]
    pub server_id: String,
    /// timestamp
    #[prost(int64, tag = "5")]
    pub create_time: i64,
    #[prost(int64, tag = "6")]
    pub send_time: i64,
    /// receiver sequence
    #[prost(int64, tag = "7")]
    pub seq: i64,
    /// is there necessary to cary the user's avatar and nickname?
    #[prost(enumeration = "MsgType", tag = "8")]
    pub msg_type: i32,
    #[prost(enumeration = "ContentType", tag = "9")]
    pub content_type: i32,
    #[prost(bytes = "vec", tag = "10")]
    pub content: Vec<u8>,
    /// it is unnecessary to put those out of content
    /// optional string sdp = 12;
    /// optional string sdp_mid = 13;
    /// optional int32 sdp_m_index = 14;
    #[prost(bool, tag = "11")]
    pub is_read: bool,
    #[prost(string, tag = "15")]
    pub group_id: String,
    /// platform of the sender
    #[prost(enumeration = "PlatformType", tag = "16")]
    pub platform: i32,

}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MsgType {
    SingleMsg = 0,
    GroupMsg = 1,
    /// / group operation
    GroupInvitation = 2,
    GroupInviteNew = 3,
    GroupMemberExit = 4,
    GroupRemoveMember = 5,
    GroupDismiss = 6,
    GroupDismissOrExitReceived = 7,
    GroupInvitationReceived = 8,
    GroupUpdate = 9,
    /// / friend operation
    FriendApplyReq = 10,
    FriendApplyResp = 11,
    FriendBlack = 12,
    FriendDelete = 13,
    /// / single call operation
    SingleCallInvite = 14,
    RejectSingleCall = 15,
    AgreeSingleCall = 16,
    SingleCallInviteNotAnswer = 17,
    SingleCallInviteCancel = 18,
    SingleCallOffer = 19,
    Hangup = 20,
    ConnectSingleCall = 21,
    Candidate = 22,
    Read = 23,
    MsgRecResp = 24,
    Notification = 25,
    Service = 26,
    FriendshipReceived = 27,
}

impl MsgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MsgType::SingleMsg => "MsgTypeSingleMsg",
            MsgType::GroupMsg => "MsgTypeGroupMsg",
            MsgType::GroupInvitation => "MsgTypeGroupInvitation",
            MsgType::GroupInviteNew => "MsgTypeGroupInviteNew",
            MsgType::GroupMemberExit => "MsgTypeGroupMemberExit",
            MsgType::GroupRemoveMember => "MsgTypeGroupRemoveMember",
            MsgType::GroupDismiss => "MsgTypeGroupDismiss",
            MsgType::GroupDismissOrExitReceived => "MsgTypeGroupDismissOrExitReceived",
            MsgType::GroupInvitationReceived => "MsgTypeGroupInvitationReceived",
            MsgType::GroupUpdate => "MsgTypeGroupUpdate",
            MsgType::FriendApplyReq => "MsgTypeFriendApplyReq",
            MsgType::FriendApplyResp => "MsgTypeFriendApplyResp",
            MsgType::FriendBlack => "MsgTypeFriendBlack",
            MsgType::FriendDelete => "MsgTypeFriendDelete",
            MsgType::SingleCallInvite => "MsgTypeSingleCallInvite",
            MsgType::RejectSingleCall => "MsgTypeRejectSingleCall",
            MsgType::AgreeSingleCall => "MsgTypeAgreeSingleCall",
            MsgType::SingleCallInviteNotAnswer => "MsgTypeSingleCallInviteNotAnswer",
            MsgType::SingleCallInviteCancel => "MsgTypeSingleCallInviteCancel",
            MsgType::SingleCallOffer => "MsgTypeSingleCallOffer",
            MsgType::Hangup => "MsgTypeHangup",
            MsgType::ConnectSingleCall => "MsgTypeConnectSingleCall",
            MsgType::Candidate => "MsgTypeCandidate",
            MsgType::Read => "MsgTypeRead",
            MsgType::MsgRecResp => "MsgTypeMsgRecResp",
            MsgType::Notification => "MsgTypeNotification",
            MsgType::Service => "MsgTypeService",
            MsgType::FriendshipReceived => "MsgTypeFriendshipReceived",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "MsgTypeSingleMsg" => Some(Self::SingleMsg),
            "MsgTypeGroupMsg" => Some(Self::GroupMsg),
            "MsgTypeGroupInvitation" => Some(Self::GroupInvitation),
            "MsgTypeGroupInviteNew" => Some(Self::GroupInviteNew),
            "MsgTypeGroupMemberExit" => Some(Self::GroupMemberExit),
            "MsgTypeGroupRemoveMember" => Some(Self::GroupRemoveMember),
            "MsgTypeGroupDismiss" => Some(Self::GroupDismiss),
            "MsgTypeGroupDismissOrExitReceived" => Some(Self::GroupDismissOrExitReceived),
            "MsgTypeGroupInvitationReceived" => Some(Self::GroupInvitationReceived),
            "MsgTypeGroupUpdate" => Some(Self::GroupUpdate),
            "MsgTypeFriendApplyReq" => Some(Self::FriendApplyReq),
            "MsgTypeFriendApplyResp" => Some(Self::FriendApplyResp),
            "MsgTypeFriendBlack" => Some(Self::FriendBlack),
            "MsgTypeFriendDelete" => Some(Self::FriendDelete),
            "MsgTypeSingleCallInvite" => Some(Self::SingleCallInvite),
            "MsgTypeRejectSingleCall" => Some(Self::RejectSingleCall),
            "MsgTypeAgreeSingleCall" => Some(Self::AgreeSingleCall),
            "MsgTypeSingleCallInviteNotAnswer" => Some(Self::SingleCallInviteNotAnswer),
            "MsgTypeSingleCallInviteCancel" => Some(Self::SingleCallInviteCancel),
            "MsgTypeSingleCallOffer" => Some(Self::SingleCallOffer),
            "MsgTypeHangup" => Some(Self::Hangup),
            "MsgTypeConnectSingleCall" => Some(Self::ConnectSingleCall),
            "MsgTypeCandidate" => Some(Self::Candidate),
            "MsgTypeRead" => Some(Self::Read),
            "MsgTypeMsgRecResp" => Some(Self::MsgRecResp),
            "MsgTypeNotification" => Some(Self::Notification),
            "MsgTypeService" => Some(Self::Service),
            "MsgTypeFriendshipReceived" => Some(Self::FriendshipReceived),
            _ => None,
        }
    }
}

/// / message content type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContentType {
    Default = 0,
    Text = 1,
    Image = 2,
    Video = 3,
    Audio = 4,
    File = 5,
    Emoji = 6,
    VideoCall = 7,
    AudioCall = 8,
    Error = 9,
}

impl ContentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContentType::Default => "Default",
            ContentType::Text => "Text",
            ContentType::Image => "Image",
            ContentType::Video => "Video",
            ContentType::Audio => "Audio",
            ContentType::File => "File",
            ContentType::Emoji => "Emoji",
            ContentType::VideoCall => "VideoCall",
            ContentType::AudioCall => "AudioCall",
            ContentType::Error => "Error",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Default" => Some(Self::Default),
            "Text" => Some(Self::Text),
            "Image" => Some(Self::Image),
            "Video" => Some(Self::Video),
            "Audio" => Some(Self::Audio),
            "File" => Some(Self::File),
            "Emoji" => Some(Self::Emoji),
            "VideoCall" => Some(Self::VideoCall),
            "AudioCall" => Some(Self::AudioCall),
            "Error" => Some(Self::Error),
            _ => None,
        }
    }
}

/// / user platform which login the system
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum PlatformType {
    Desktop = 0,
    Mobile = 1,
}

impl PlatformType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlatformType::Desktop => "Desktop",
            PlatformType::Mobile => "Mobile",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Desktop" => Some(Self::Desktop),
            "Mobile" => Some(Self::Mobile),
            _ => None,
        }
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq)]
pub struct SendMsgRequest {
    pub message: Option<Msg>,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMsgResponse {}

#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgResponse {
    #[prost(string, tag = "1")]
    pub local_id: String,
    #[prost(string, tag = "2")]
    pub server_id: String,
    #[prost(int64, tag = "3")]
    pub send_time: i64,
    #[prost(string, tag = "4")]
    pub err: String,
}


pub mod chat_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]

    use std::sync::Arc;
    use std::task::{Context, Poll};
    use http::Request;
    use tonic::async_trait;
    use tonic::codec::{CompressionEncoding, EnabledCompressionEncodings};
    use tonic::codegen::{Body, BoxFuture, InterceptedService, StdError};

    #[async_trait]
    pub trait ChatService: Send + Sync + 'static {
        async fn send_msg(&self, request: tonic::Request<super::SendMsgRequest>) -> Result<tonic::Response<super::MsgResponse>, tonic::Status>;
    }


    /// chat service, receive message then generate message id and send message to
    /// mq: response operation result
    #[derive(Debug)]
    pub struct ChatServiceServer<T: ChatService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    #[derive(Debug)]
    struct _Inner<T>(Arc<T>);

    impl<T: ChatService> ChatServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }

        pub fn from_arc(_inner: Arc<T>) -> Self {
            let inner = _Inner(_inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None
                ,
            }
        }

        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
            where
                F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }

    impl<T, B> tonic::codegen::Service<http::Request<B>> for ChatServiceServer<T>
        where T: ChatService,
              B: Body + Send + 'static,
              B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;

        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            todo!()
        }

        fn call(&mut self, req: Request<B>) -> Self::Future {
            todo!()
        }
    }
}
