#![allow(non_snake_case)]

use std::io::Write;
use jni_simple::{*};
use load_image::ImageData;
use load_image::export::rgb::ComponentMap;
use std::os::raw::{c_void};
use std::ptr::{null, null_mut};
use std::thread;
use std::time::Duration;
use std::io::stdout;

#[unsafe(no_mangle)]
pub unsafe extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: *mut c_void) -> jint {
    //All error codes are jint, never JNI_OK. See JNI documentation for their meaning when you handle them.
    //This is a Result<JNIEnv, jint>.
    let env : JNIEnv = vm.GetEnv(JNI_VERSION_10).unwrap();

    println!("RUST: JNI_OnLoad");

    JNI_VERSION_10
}


#[unsafe(no_mangle)]
pub unsafe extern "system" fn Java_dev_lasm_compressedtexture_Native_loadImage(env: JNIEnv, _class: jclass, byteBuf: *const c_void, len: usize,
    width: *mut jlong, height: *mut jlong, ch: *mut jlong, malloc: Option<extern "C" fn(usize) -> *mut u8>) -> *mut u8 {
    let data = std::ptr::slice_from_raw_parts(byteBuf as *const u8, len);
    let image = load_image::load_data(&*data).unwrap();
    /*
    let mut channels = 1;

    let mut src: *mut u8 = null_mut();

    let dest = match &image.bitmap {
        ImageData::RGB8(bitmap) => {
            channels = 4;
            let src: Vec<_> = bitmap.iter().map(|px| px.with_alpha(255)).collect();
            let src = src.as_ptr() as *mut u8;
            let dest = malloc.unwrap()(image.width * image.height * channels);
            std::ptr::copy_nonoverlapping(src, dest, image.width * image.height * channels);
            dest
        },
        ImageData::RGBA8(bitmap) => {
            channels = 4;
            let src = bitmap.as_ptr() as *mut u8;
            let dest = malloc.unwrap()(image.width * image.height * channels);
            std::ptr::copy_nonoverlapping(src, dest, image.width * image.height * channels);
            dest
        },
        ImageData::RGB16(bitmap) => {
            channels = 4;
            let src: Vec<_> = bitmap.iter().map(|px| px.map(|c| (c>>8) as u8).with_alpha(255)).collect();
            let src = src.as_ptr() as *mut u8;
            let dest = malloc.unwrap()(image.width * image.height * channels);
            std::ptr::copy_nonoverlapping(src, dest, image.width * image.height * channels);
            dest
        },
        ImageData::RGBA16(bitmap) => {
            channels = 4;
            let src: Vec<_> = bitmap.iter().map(|px| px.map(|c| (c>>8) as u8)).collect();
            let src = src.as_ptr() as *mut u8;
            let dest = malloc.unwrap()(image.width * image.height * channels);
            std::ptr::copy_nonoverlapping(src, dest, image.width * image.height * channels);
            dest
        },
        ImageData::GRAY8(bitmap) => {
            channels = 1;
            let src = bitmap.as_ptr() as *mut u8;
            let dest = malloc.unwrap()(image.width * image.height * channels);
            std::ptr::copy_nonoverlapping(src, dest, image.width * image.height * channels);
            dest
        },
        ImageData::GRAY16(bitmap) => {
            channels = 1;
            let src: Vec<_> = bitmap.iter().map(|px| px.map(|c| (c>>8) as u8)).collect();
            let src = src.as_ptr() as *mut u8;
            let dest = malloc.unwrap()(image.width * image.height * channels);
            std::ptr::copy_nonoverlapping(src, dest, image.width * image.height * channels);
            dest
        },
        ImageData::GRAYA8(bitmap) => {
            channels = 2;
            src = bitmap.as_ptr() as *mut u8;
            let src = bitmap.as_ptr() as *mut u8;
            let dest = malloc.unwrap()(image.width * image.height * channels);
            std::ptr::copy_nonoverlapping(src, dest, image.width * image.height * channels);
            dest
        },
        ImageData::GRAYA16(bitmap) => {
            channels = 2;
            let src: Vec<_> = bitmap.iter().map(|px| px.map(|c| (c>>8) as u8)).collect();
            let src = src.as_ptr() as *mut u8;
            let dest = malloc.unwrap()(image.width * image.height * channels);
            std::ptr::copy_nonoverlapping(src, dest, image.width * image.height * channels);
            dest
        },
    };*/
    let mut binding = image.into_rgba();
    let (src, w, h) = binding.0.as_contiguous_buf();
    let dest = malloc.unwrap()(w*h*4);
    std::ptr::copy_nonoverlapping(src.as_ptr() as *const u8, dest, w*h*4);
    *width = w as jlong;
    *height = h as jlong;
    *ch = 4 as jlong;
    dest
}
