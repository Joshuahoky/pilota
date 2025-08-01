//! Protocol Buffers well-known wrapper types.
//!
//! This module provides implementations of `Message` for Rust standard library
//! types which correspond to a Protobuf well-known wrapper type. The remaining
//! well-known types are defined in the `prost-types` crate in order to avoid a
//! cyclic dependency between `prost` and `prost-build`.
extern crate alloc;

use alloc::{string::String, vec::Vec};

use ::bytes::Bytes;
use linkedbytes::LinkedBytes;

use super::{
    DecodeError, Message,
    encoding::{
        DecodeContext, WireType, bool, bytes, double, float, int32, int64, skip_field, string,
        uint32, uint64,
    },
};

/// `google.protobuf.BoolValue`
impl Message for bool {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if *self {
            bool::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            bool::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self { 2 } else { 0 }
    }
}

/// `google.protobuf.UInt32Value`
impl Message for u32 {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if *self != 0 {
            uint32::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            uint32::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self != 0 {
            uint32::encoded_len(1, self)
        } else {
            0
        }
    }
}

/// `google.protobuf.UInt64Value`
impl Message for u64 {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if *self != 0 {
            uint64::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            uint64::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self != 0 {
            uint64::encoded_len(1, self)
        } else {
            0
        }
    }
}

/// `google.protobuf.Int32Value`
impl Message for i32 {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if *self != 0 {
            int32::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            int32::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self != 0 {
            int32::encoded_len(1, self)
        } else {
            0
        }
    }
}

/// `google.protobuf.Int64Value`
impl Message for i64 {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if *self != 0 {
            int64::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            int64::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self != 0 {
            int64::encoded_len(1, self)
        } else {
            0
        }
    }
}

/// `google.protobuf.FloatValue`
impl Message for f32 {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if *self != 0.0 {
            float::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            float::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self != 0.0 {
            float::encoded_len(1, self)
        } else {
            0
        }
    }
}

/// `google.protobuf.DoubleValue`
impl Message for f64 {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if *self != 0.0 {
            double::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            double::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if *self != 0.0 {
            double::encoded_len(1, self)
        } else {
            0
        }
    }
}

/// `google.protobuf.StringValue`
impl Message for String {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if !self.is_empty() {
            string::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            string::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if !self.is_empty() {
            string::encoded_len(1, self)
        } else {
            0
        }
    }
}

/// `google.protobuf.BytesValue`
impl Message for Vec<u8> {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if !self.is_empty() {
            bytes::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            bytes::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if !self.is_empty() {
            bytes::encoded_len(1, self)
        } else {
            0
        }
    }
}

/// `google.protobuf.BytesValue`
impl Message for Bytes {
    fn encode_raw(&self, buf: &mut LinkedBytes) {
        if !self.is_empty() {
            bytes::encode(1, self, buf)
        }
    }
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError> {
        if tag == 1 {
            bytes::merge(wire_type, self, buf, ctx)
        } else {
            skip_field(wire_type, tag, buf, ctx)
        }
    }
    fn encoded_len(&self) -> usize {
        if !self.is_empty() {
            bytes::encoded_len(1, self)
        } else {
            0
        }
    }
}

/// `google.protobuf.Empty`
impl Message for () {
    fn encode_raw(&self, _buf: &mut LinkedBytes) {}
    fn merge_field(
        &mut self,
        tag: u32,
        wire_type: WireType,
        buf: &mut Bytes,
        ctx: &mut DecodeContext,
    ) -> Result<(), DecodeError> {
        skip_field(wire_type, tag, buf, ctx)
    }
    fn encoded_len(&self) -> usize {
        0
    }
}
