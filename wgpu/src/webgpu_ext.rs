use crate::{Device, Texture};
use wasm_bindgen::JsValue;

pub trait ToWebGpu {
    fn to_webgpu(&self) -> JsValue;
}

impl ToWebGpu for Device {
    fn to_webgpu(&self) -> JsValue {
        self.inner.as_webgpu().inner.clone().into()
    }
}

impl ToWebGpu for Texture {
    fn to_webgpu(&self) -> JsValue {
        self.inner.as_webgpu().inner.clone().into()
    }
}
