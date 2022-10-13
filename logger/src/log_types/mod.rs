use std::fmt::Display;

use fake::{Dummy, Fake};
use serde::Serialize;

#[derive(Serialize, Dummy)]
pub struct TransportLog {
    _transport: String,
    syslog_facility: u8,
    _boot_id: String,
    message: String,
    syslog_identifier: String,
    __realtime_timestamp: u16,
    _hostname: String,
    _source_monotonic_timestamp: u8,
    _machine_id: String,
    __monotonic_timestamp: u8,
    __cursor: String,
    priority: u8,
}

#[derive(Serialize, Dummy)]
pub struct ExtendedTransportLog {
    _audit_field_scontext: String,
    _audit_field_permissive: u8,
    _source_realtime_timestamp: u16,
    _audit_field_tclass: String,
    _audit_type_name: String,
    _audit_field_tcontext: String,
    _audit_type: u8,
    _audit_id: u8,
    _pid: u8,
    _transport: String,
    syslog_facility: u8,
    _boot_id: String,
    message: String,
    syslog_identifier: String,
    __realtime_timestamp: u16,
    _hostname: String,
    _source_monotonic_timestamp: u8,
    _machine_id: String,
    __monotonic_timestamp: u8,
    __cursor: String,
    priority: u8,
    _comm: String,
    _audit_field_dev: String,
    _audit_field_ino: String,
    _audit_field_path: String,
}
