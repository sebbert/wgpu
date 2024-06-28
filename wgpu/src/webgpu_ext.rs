use crate::{backend::ContextWebGpu, context::Context, Device, Texture};
use crate::backend::webgpu::webgpu_sys::{GpuDevice, GpuTexture};
use wasm_bindgen::JsValue;

pub trait ToWebGpu {
    fn to_webgpu(&self) -> JsValue;
}

impl ToWebGpu for Device {
    fn to_webgpu(&self) -> JsValue {
        self.data.downcast_ref::<<ContextWebGpu as Context>::DeviceData>().unwrap().0.clone().into()
    }
}

impl ToWebGpu for Texture {
    fn to_webgpu(&self) -> JsValue {
        self.data.downcast_ref::<<ContextWebGpu as Context>::TextureData>().unwrap().0.clone().into()
    }
}