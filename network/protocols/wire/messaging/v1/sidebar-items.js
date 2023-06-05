window.SIDEBAR_ITEMS = {"enum":[["ErrorCode",""],["NetworkMessage","Most primitive message type set on the network."],["NotSupportedType","Flags an unsupported network message.  This is a message that a peer can parse its header information but does not have a handler."],["ReadError","Errors from reading and deserializing network messages off the wire."],["WriteError","Errors from serializing and sending network messages on the wire."]],"fn":[["network_message_frame_codec","Returns a fully configured length-delimited codec for writing/reading serialized [`NetworkMessage`] frames to/from a socket."]],"struct":[["DirectSendMsg",""],["NetworkMessageSink","A `Sink` of outbound `NetworkMessage`s that will be serialized and sent over an underlying socket."],["NetworkMessageStream","A `Stream` of inbound `NetworkMessage`s read and deserialized from an underlying socket."],["ParsingErrorType","Flags an invalid network message with as much header information as possible. This is a message that this peer cannot even parse its header information."],["RpcRequest",""],["RpcResponse",""]],"type":[["Priority","Create alias Priority for u8."],["RequestId","Create alias RequestId for u32."]]};