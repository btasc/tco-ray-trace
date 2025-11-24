use crate::{
    error::{ LatrError, WindowError },
    latr_core::LatrEngine,
    config::LatrConfig,
    engine::engine_core::Engine,
    gpu_utils::gpu_core::GpuCore,
};

use std::sync::Arc;

pub fn run_event_loop(
    config: LatrConfig,
    engine_core: Engine,
    gpu_core: GpuCore,
    window: Arc<winit::window::Window>,
    event_loop: winit::event_loop::EventLoop<()>,
) -> Result<(), LatrError> {

    Err(LatrError::Window(WindowError::EventLoopExited))
}