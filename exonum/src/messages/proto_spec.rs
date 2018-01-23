use encoding;
use storage;
use crypto;
use messages::{self, Message, RawMessage};

/// TODO
pub trait ServiceMessage: Message {
    /// TODO
    const SERVICE_ID: u16;
    /// TODO
    const ID: u16;

    /// TODO
    fn from_raw(raw: RawMessage) -> Result<Self, encoding::Error> where Self: Sized;
}

#[macro_export]
macro_rules! exonum_protocol {
    {
        const SERVICE_ID = $service_id:expr;

        $(
            $(#[$tx_attr:meta])*
            struct $tx_name:ident {
            $(
                $(#[$field_attr:meta])*
                $field_name:ident : $field_type:ty
            ),*
            $(,)* // optional trailing comma
            }
        )*
    }

    =>

    {
        __ex_proto_messages!(
            $service_id,
            0,
            $(
                $(#[$tx_attr])*
                struct $tx_name {
                $(
                    $(#[$field_attr])*
                    $field_name : $field_type
                ),*
                }
            )*
        );
    };
}

macro_rules! __ex_proto_messages {
    {
        $service_id:expr,
        $id:expr,

        $(#[$tx_attr:meta])*
        struct $tx_name:ident {
        $(
            $(#[$field_attr:meta])*
            $field_name:ident : $field_type:ty
        ),*
        $(,)*
        }

        $($tt:tt)*
    } => {

        #[derive(Clone, PartialEq, Debug)] //FIXME: no debug
        $(#[$tx_attr])*
        pub struct $tx_name {
            raw: $crate::messages::RawMessage
        }

        impl $crate::messages::Message for $tx_name {
            fn raw(&self) -> &$crate::messages::RawMessage {
                &self.raw
            }
        }

        // Can't move out of macro due to coherence
        impl AsRef<$crate::messages::RawMessage> for $tx_name {
            fn as_ref(&self) -> &$crate::messages::RawMessage {
                $crate::messages::Message::raw(self)
            }
        }


        // Can't move out of macro due to coherence
        impl $crate::encoding::serialize::FromHex for $tx_name {
            type Error = $crate::encoding::Error;

            fn from_hex<B: AsRef<[u8]>>(hex: B) -> Result<Self, Self::Error> {
                let vec = Vec::<u8>::from_hex(hex)
                    .map_err(|e| $crate::encoding::Error::Other(Box::new(e)))?;
                if vec.len() < $crate::messages::HEADER_LENGTH {
                    return Err($crate::encoding::Error::Basic("Hex is too short.".into()));
                }
                let buf = $crate::messages::MessageBuffer::from_vec(vec);
                let raw = $crate::messages::RawMessage::new(buf);
                Self::from_raw(raw)
            }
        }


        impl $tx_name {
            /// TODO
            pub fn from_raw(raw: $crate::messages::RawMessage) -> Result<$tx_name, $crate::encoding::Error> {
                $crate::messages::ServiceMessage::from_raw(raw)
            }

            #[cfg_attr(feature="cargo-clippy", allow(too_many_arguments))]
            /// Creates message and sign it.
            #[allow(unused_mut)]
            #[allow(unused)]
            pub fn new($($field_name: $field_type,)*
                       secret_key: &$crate::crypto::SecretKey) -> Self {
                use $crate::messages::{RawMessage, MessageWriter, ServiceMessage};
                let mut writer = MessageWriter::new(
                    $crate::messages::PROTOCOL_MAJOR_VERSION,
                    $crate::messages::TEST_NETWORK_ID,
                    Self::SERVICE_ID,
                    $id, $tx_name::__ex_header_size() as usize,
                );
                __ex_for_each_field!(
                    __ex_message_write_field, (writer),
                    $( ($(#[$field_attr])*, $field_name, $field_type) )*
                );
                $tx_name { raw: RawMessage::new(writer.sign(secret_key)) }
            }

            /// Creates message and appends existing signature.
            #[cfg_attr(feature="cargo-clippy", allow(too_many_arguments))]
            #[allow(dead_code, unused_mut)]
            pub fn new_with_signature($($field_name: $field_type,)*
                                      signature: &$crate::crypto::Signature) -> Self {
                use $crate::messages::{RawMessage, MessageWriter, ServiceMessage};
                let mut writer = MessageWriter::new(
                    $crate::messages::PROTOCOL_MAJOR_VERSION,
                    $crate::messages::TEST_NETWORK_ID,
                    Self::SERVICE_ID,
                    $id, $tx_name::__ex_header_size() as usize,
                );
                __ex_for_each_field!(
                    __ex_message_write_field, (writer),
                    $( ($(#[$field_attr])*, $field_name, $field_type) )*
                );
                $tx_name { raw: RawMessage::new(writer.append_signature(signature)) }
            }

            __ex_for_each_field!(
                __ex_message_mk_field, (),
                $( ($(#[$field_attr])*, $field_name, $field_type) )*
            );

            #[allow(unused_variables)]
            #[doc(hidden)]
            fn __ex_check_fields(
                raw_message: &$crate::messages::RawMessage,
            ) -> $crate::encoding::Result {
                let header_length =
                    $crate::messages::HEADER_LENGTH as $crate::encoding::Offset;
                let latest_segment = (Self::__ex_header_size() + header_length).into();
                __ex_for_each_field!(
                    __ex_message_check_field, (latest_segment, raw_message),
                    $( ($(#[$field_attr])*, $field_name, $field_type) )*
                );
                Ok(latest_segment)
            }

            #[doc(hidden)]
            fn __ex_header_size() -> $crate::encoding::Offset {
                __ex_header_size!($($field_type),*)
            }
        }

        impl $crate::messages::ServiceMessage for $tx_name {
            const SERVICE_ID: u16 = $service_id;
            const ID: u16 = $id;

            /// Converts the raw message into the specific one.
            fn from_raw(raw: $crate::messages::RawMessage)
                -> Result<$tx_name, $crate::encoding::Error> {
                use $crate::messages::{ServiceMessage};

                let min_message_size = $tx_name::__ex_header_size() as usize
                            + $crate::messages::HEADER_LENGTH as usize
                            + $crate::crypto::SIGNATURE_LENGTH as usize;
                if raw.len() < min_message_size {
                    return Err($crate::encoding::Error::UnexpectedlyShortPayload {
                        actual_size: raw.len() as $crate::encoding::Offset,
                        minimum_size: min_message_size as $crate::encoding::Offset,
                    });
                }

                // Check identifiers
                if raw.version() != $crate::messages::PROTOCOL_MAJOR_VERSION {
                    return Err($crate::encoding::Error::UnsupportedProtocolVersion {
                        version: $crate::messages::PROTOCOL_MAJOR_VERSION
                    });
                }
                if raw.network_id() != $crate::messages::TEST_NETWORK_ID {
                    return Err($crate::encoding::Error::IncorrectNetworkId {
                        network_id: $crate::messages::TEST_NETWORK_ID
                    });
                }
                if raw.message_type() != $id {
                    return Err($crate::encoding::Error::IncorrectMessageType {
                        message_type: $id
                    });
                }
                if raw.service_id() != Self::SERVICE_ID {
                    return Err($crate::encoding::Error::IncorrectServiceId {
                        service_id: Self::SERVICE_ID
                    });
                }

                // Check body
                let body_len = Self::__ex_check_fields(&raw)?;
                if body_len.unchecked_offset() as usize +
                    $crate::crypto::SIGNATURE_LENGTH as usize != raw.len()  {
                   return Err("Incorrect raw message length.".into())
                }

                Ok($tx_name { raw: raw })
            }
        }

        impl $crate::encoding::serialize::json::ExonumJson for $tx_name {
            fn deserialize_field<B> (value: &$crate::encoding::serialize::json::reexport::Value,
                                        buffer: & mut B,
                                        from: $crate::encoding::Offset,
                                        to: $crate::encoding::Offset )
                -> Result<(), Box<::std::error::Error>>
            where B: $crate::encoding::serialize::WriteBufferWrapper
            {
                use $crate::encoding::serialize::json::ExonumJsonDeserialize;
                // deserialize full field
                let structure = <Self as ExonumJsonDeserialize>::deserialize(value)?;
                // then write it
                buffer.write(from, to, structure);
                Ok(())
            }


            #[allow(unused_mut)]
            fn serialize_field(&self)
                -> Result<$crate::encoding::serialize::json::reexport::Value,
                            Box<::std::error::Error + Send + Sync>>
            {
                use $crate::encoding::serialize::json::reexport::Value;
                use $crate::encoding::serialize::json::reexport::Map;
                let mut body = Map::new();
                $(
                    body.insert(stringify!($field_name).to_string(),
                        self.$field_name().serialize_field()?);
                )*
                let mut structure = Map::new();
                structure.insert("body".to_string(), Value::Object(body));
                structure.insert("signature".to_string(),
                                    self.raw.signature().serialize_field()?);
                structure.insert("message_id".to_string(),
                                    self.raw.message_type().serialize_field()?);
                structure.insert("service_id".to_string(),
                                    self.raw.service_id().serialize_field()?);
                structure.insert("network_id".to_string(),
                                    self.raw.network_id().serialize_field()?);
                structure.insert("protocol_version".to_string(),
                                    self.raw.version().serialize_field()?);
                Ok(Value::Object(structure))
            }
        }

        impl $crate::encoding::serialize::json::ExonumJsonDeserialize for $tx_name {
            #[allow(unused_imports, unused_variables, unused_mut)]
            fn deserialize(value: &$crate::encoding::serialize::json::reexport::Value)
                -> Result<Self, Box<::std::error::Error>>
            {
                eprintln!("deserialize {}", stringify!($tx_name));

                use $crate::encoding::serialize::json::ExonumJson;
                use $crate::encoding::serialize::json::reexport::from_value;
                use $crate::messages::{RawMessage, MessageWriter, ServiceMessage};

                // if we could deserialize values, try append signature
                let obj = value.as_object().ok_or("Can't cast json as object.")?;

                let body = obj.get("body").ok_or("Can't get body from json.")?;

                let signature = from_value(obj.get("signature")
                                    .ok_or("Can't get signature from json")?.clone())?;
                let message_type = from_value(obj.get("message_id")
                                    .ok_or("Can't get message_type from json")?.clone())?;
                let service_id = from_value(obj.get("service_id")
                                    .ok_or("Can't get service_id from json")?.clone())?;

                let network_id = from_value(obj.get("network_id")
                                    .ok_or("Can't get network_id from json")?.clone())?;
                let protocol_version = from_value(obj.get("protocol_version")
                                        .ok_or("Can't get protocol_version from json")?.clone())?;

                if service_id != Self::SERVICE_ID {
                    return Err("service_id didn't equal real service_id.".into())
                }

                if message_type != $id {
                    return Err("message_id didn't equal real message_id.".into())
                }

                let mut writer = MessageWriter::new(
                    protocol_version,
                    network_id,
                    service_id,
                    message_type,
                    $tx_name::__ex_header_size() as usize,
                );
                let obj = body.as_object().ok_or("Can't cast body as object.")?;
                __ex_for_each_field!(
                    __ex_deserialize_field, (obj, writer),
                    $( ($(#[$field_attr])*, $field_name, $field_type) )*
                );
                Ok($tx_name { raw: RawMessage::new(writer.append_signature(&signature)) })
            }
        }

        // TODO: Rewrite Deserialize and Serialize implementation (ECR-156)
        impl<'de> $crate::encoding::serialize::reexport::Deserialize<'de> for $tx_name {
            #[allow(unused_mut)]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: $crate::encoding::serialize::reexport::Deserializer<'de>
            {
                eprintln!("JUST DESER");
                use $crate::encoding::serialize::json::reexport::Value;
                use $crate::encoding::serialize::reexport::{DeError, Deserialize};
                let value = <Value as Deserialize>::deserialize(deserializer)?;
                <Self as $crate::encoding::serialize::json::ExonumJsonDeserialize>::deserialize(
                    &value).map_err(|e| D::Error::custom(
                            format!("Can't deserialize a value: {}", e.description())))
            }
        }

        impl $crate::encoding::serialize::reexport::Serialize for $tx_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: $crate::encoding::serialize::reexport::Serializer
            {
                use $crate::encoding::serialize::reexport::SerError;
                use $crate::encoding::serialize::json::ExonumJson;
                self.serialize_field()
                    .map_err(|_| S::Error::custom(
                                concat!("Can not serialize structure: ", stringify!($tx_name))))?
                    .serialize(serializer)
            }
        }

        __ex_proto_messages!(
            $service_id,
            $id + 1,
            $($tt)*
        );
    };

    { $service_id:expr, $id:expr, } => {};
}

impl <'a, T: ServiceMessage> encoding::SegmentField<'a> for T {
    fn item_size() -> encoding::Offset {
        1
    }

    fn count(&self) -> encoding::Offset {
        self.raw().len() as encoding::Offset
    }

    unsafe fn from_buffer(buffer: &'a [u8], from: encoding::Offset, count: encoding::Offset) -> Self {
        let raw_message: RawMessage = encoding::SegmentField::from_buffer(buffer, from, count);
        Self::from_raw(raw_message).unwrap()
    }

    fn extend_buffer(&self, buffer: &mut Vec<u8>) {
        buffer.extend_from_slice(self.raw().as_ref().as_ref())
    }

    fn check_data(
        buffer: &'a [u8],
        from: encoding::CheckedOffset,
        count: encoding::CheckedOffset,
        latest_segment: encoding::CheckedOffset
    ) -> encoding::Result {
        let latest_segment_origin = RawMessage::check_data(buffer, from, count, latest_segment)?;
        // TODO: remove this allocation,
        // by allowing creating message from borrowed data (ECR-156)
        let raw_message: RawMessage = unsafe {
            encoding::SegmentField::from_buffer(
                buffer, from.unchecked_offset(), count.unchecked_offset()
            )
        };
        let _ = Self::from_raw(raw_message)?;
        Ok(latest_segment_origin)
    }
}

impl<T: ServiceMessage> storage::StorageValue for T {
    fn hash(&self) -> crypto::Hash {
        Message::hash(self)
    }

    fn into_bytes(self) -> Vec<u8> {
        self.raw().as_ref().as_ref().to_vec()
    }

    fn from_bytes(value: ::std::borrow::Cow<[u8]>) -> Self {
        let raw = messages::RawMessage::new(
            messages::MessageBuffer::from_vec(value.into_owned())
        );
        // FIXME: check perf?
        Self::from_raw(raw).unwrap()
    }
}


use std::net::SocketAddr;
use std::time::SystemTime;

use crypto::{Hash, PublicKey};
use helpers::{Height, Round, ValidatorId};

exonum_protocol! {
    const SERVICE_ID = 0;

    /// Connect to a node.
    ///
    /// ### Validation
    /// The message is ignored if its time is earlier than in the previous `Connect` message received
    /// from the same peer.
    ///
    /// ### Processing
    /// Connect to the peer.
    ///
    /// ### Generation
    /// A node sends `Connect` message to all known addresses during initialization. Additionally,
    /// the node responds by its own `Connect` message after receiving `node::Event::Connected`.
    struct Connect {
        /// The sender's public key.
        pub_key: &PublicKey,
        /// The node's address.
        addr: SocketAddr,
        /// Time when the message was created.
        time: SystemTime,
    }

    /// Proposal for a new block.
    ///
    /// ### Validation
    /// The message is ignored if it
    ///     * contains incorrect `prev_hash`
    ///     * is sent by non-leader
    ///     * contains already committed transactions
    ///     * is already known
    ///
    /// ### Processing
    /// If the message contains unknown transactions, then `TransactionsRequest` is sent in reply.
    /// Otherwise `Prevote` is broadcast.
    ///
    /// ### Generation
    /// A node broadcasts `Propose` if it is a leader and is not locked for a different proposal. Also
    /// `Propose` can be sent as response to `ProposeRequest`.
    struct Propose {
        /// The validator id.
        validator: ValidatorId,
        /// The height to which the message is related.
        height: Height,
        /// The round to which the message is related.
        round: Round,
        /// Hash of the previous block.
        prev_hash: &Hash,
        /// The list of transactions to include in the next block.
        transactions: &[Hash],
    }

    /// Pre-vote for a new block.
    ///
    /// ### Validation
    /// A node panics if it has already sent a different `Prevote` for the same round.
    ///
    /// ### Processing
    /// Pre-vote is added to the list of known votes for the same proposal.
    /// If `locked_round` number from the message is bigger than in a node state, then a node replies
    /// with `PrevotesRequest`.
    /// If there are unknown transactions in the propose specified by `propose_hash`,
    /// `TransactionsRequest` is sent in reply.
    /// Otherwise if all transactions are known and there are +2/3 pre-votes, then a node is locked
    /// to that proposal and `Precommit` is broadcast.
    ///
    /// ### Generation
    /// A node broadcasts `Prevote` in response to `Propose` when it has received all the transactions.
    struct Prevote {
        /// The validator id.
        validator: ValidatorId,
        /// The height to which the message is related.
        height: Height,
        /// The round to which the message is related.
        round: Round,
        /// Hash of the corresponding `Propose`.
        propose_hash: &Hash,
        /// Locked round.
        locked_round: Round,
    }
}
