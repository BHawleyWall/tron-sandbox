mod adapters;
mod entities;
mod use_cases;

use anyhow::Result;
use tracing::{instrument, trace};

use crate::adapters::gateways::telemetry::interface::init_tracing;

#[instrument]
pub fn dummy(verbosity: u8) -> Result<()> {
    init_tracing(verbosity)?;

    trace!("Dummy function called");

    Ok(())
}
