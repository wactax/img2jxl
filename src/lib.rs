// use image::EncodableLayout;
use image::ImageFormat;
use jpegxl_rs::{
  encode::{EncoderFrame, EncoderResult, EncoderSpeed},
  encoder_builder,
};
use napi::{
  bindgen_prelude::{AsyncTask, Buffer},
  Env, Result, Task,
};
use napi_derive::napi;

struct Pkg {
  bin: Buffer,
  quality: f32,
  ext: Option<String>,
}

impl Task for Pkg {
  type Output = Buffer;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    Ok(_img_jxl(self)?)
  }

  fn resolve(&mut self, _: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output)
  }
}
fn _img_jxl(pkg: &Pkg) -> anyhow::Result<Buffer> {
  let bin = &pkg.bin;
  let guessed;
  let format;
  loop {
    if let Some(ext) = &pkg.ext {
      if let Some(f) = ImageFormat::from_mime_type(ext) {
        guessed = false;
        format = f;
        break;
      }
    }
    guessed = true;
    format = image::guess_format(bin)?;
    break;
  }

  let img = match image::load_from_memory_with_format(bin, format) {
    Ok(r) => r,
    Err(err) => {
      if guessed {
        Err(err)?;
      };
      image::load_from_memory_with_format(bin, image::guess_format(bin)?)?
    }
  };
  let width = img.width();
  let height = img.height();
  let img = img.into_rgba8();
  let img = EncoderFrame::new(img.as_raw()).num_channels(4);

  let lossless = pkg.quality == 0.0;
  let mut bind = encoder_builder();
  let mut encoder = bind
    .speed(EncoderSpeed::Kitten)
    .has_alpha(true)
    .lossless(lossless);
  if !lossless {
    // https://docs.rs/jpegxl-rs/latest/jpegxl_rs/encode/struct.JxlEncoderBuilder.html#method.quality
    encoder = encoder.quality(pkg.quality);
  };

  let buffer: EncoderResult<u8> = encoder.build()?.encode_frame(&img, width, height)?;
  Ok(buffer.data.into())
}

#[allow(dead_code)]
#[napi]
fn img_jxl(bin: Buffer, ext: Option<String>, quality: f64) -> AsyncTask<Pkg> {
  let quality = quality as f32;
  AsyncTask::new(Pkg { bin, quality, ext })
}
