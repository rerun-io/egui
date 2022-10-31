//! This crates provides bindings between [`egui`](https://github.com/emilk/egui) and [wgpu](https://crates.io/crates/wgpu).
//!
//! ## Feature flags
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
//!

#![allow(unsafe_code)]

pub use wgpu;

/// Low-level painting of [`egui`] on [`wgpu`].
pub mod renderer;
pub use renderer::CallbackFn;
pub use renderer::Renderer;

/// Module for painting [`egui`] with [`wgpu`] on [`winit`].
#[cfg(feature = "winit")]
pub mod winit;

use egui::mutex::RwLock;
use std::sync::Arc;

/// Access to the render state for egui.
#[derive(Clone)]
pub struct RenderState {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub target_format: wgpu::TextureFormat,
    pub renderer: Arc<RwLock<Renderer>>,
}

/// Configuration for using wgpu with eframe or the egui-wgpu winit feature.
#[derive(Clone)]
pub struct WgpuConfiguration {
    /// Configuration passed on device request.
    pub device_descriptor: wgpu::DeviceDescriptor<'static>,

    /// Backends that should be supported (wgpu will pick one of these)
    pub backends: wgpu::Backends,

    /// Present mode used for the primary surface.
    pub present_mode: wgpu::PresentMode,

    /// Power preference for the adapter.
    pub power_preference: wgpu::PowerPreference,

    /// Callback for surface errors.
    pub on_surface_error: Arc<dyn Fn(wgpu::SurfaceError)>,
}

impl Default for WgpuConfiguration {
    fn default() -> Self {
        Self {
            device_descriptor: wgpu::DeviceDescriptor {
                label: Some("egui wgpu device"),
                features: wgpu::Features::default(),
                limits: wgpu::Limits::default(),
            },
            backends: wgpu::Backends::PRIMARY | wgpu::Backends::GL,
            present_mode: wgpu::PresentMode::AutoVsync,
            power_preference: wgpu::PowerPreference::HighPerformance,

            on_surface_error: Arc::new(|err| match err {
                wgpu::SurfaceError::Outdated => {
                    // This error occurs when the app is minimized on Windows.
                    // Silently return here to prevent spamming the console with:
                    // "The underlying surface has changed, and therefore the swap chain must be updated"
                }
                _ => {
                    tracing::warn!("Dropped frame with error: {err}");
                }
            }),
        }
    }
}

/// Find the framebuffer format that egui prefers
pub fn preferred_framebuffer_format(formats: &[wgpu::TextureFormat]) -> wgpu::TextureFormat {
    for &format in formats {
        if matches!(
            format,
            wgpu::TextureFormat::Rgba8Unorm | wgpu::TextureFormat::Bgra8Unorm
        ) {
            return format;
        }
    }
    formats[0] // take the first
}
