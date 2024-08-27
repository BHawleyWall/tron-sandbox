#![allow(dead_code, unused_imports, unused_variables)]

use anyhow::Result;
use tracing::{debug, error, info, instrument, trace, warn};

mod tron_protos {
    tonic::include_proto!("protocol");
}
