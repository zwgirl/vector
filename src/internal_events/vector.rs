use super::InternalEvent;
use metrics::counter;
use prost::DecodeError;

#[derive(Debug)]
pub struct VectorEventSent {
    pub byte_size: usize,
}

impl InternalEvent for VectorEventSent {
    fn emit_metrics(&self) {
        counter!("vector_events_processed_total", 1);
        counter!("vector_processed_bytes_total", self.byte_size as u64);
    }
}

#[derive(Debug)]
pub struct VectorEventReceived {
    pub byte_size: usize,
}

impl InternalEvent for VectorEventReceived {
    fn emit_logs(&self) {
        trace!(message = "Received one event.");
    }

    fn emit_metrics(&self) {
        counter!("vector_events_processed_total", 1);
        counter!("vector_processed_bytes_total", self.byte_size as u64);
    }
}

#[derive(Debug)]
pub struct VectorProtoDecodeError {
    pub error: DecodeError,
}

impl InternalEvent for VectorProtoDecodeError {
    fn emit_logs(&self) {
        error!(message = "Failed to decode protobuf message.", error = %self.error, rate_limit_secs = 10);
    }

    fn emit_metrics(&self) {
        counter!("vector_protobuf_decode_errors_total", 1);
    }
}
