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

  #[allow(clippy::never_loop)]
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

  let lossless = pkg.quality == 0.0;
  let mut bind = encoder_builder();
  let has_alpha = img.color().has_alpha();
  let mut encoder = bind
    .speed(EncoderSpeed::Kitten)
    .has_alpha(has_alpha)
    .lossless(lossless);
  if !lossless {
    // https://docs.rs/jpegxl-rs/latest/jpegxl_rs/encode/struct.JxlEncoderBuilder.html#method.quality
    encoder = encoder.quality(pkg.quality);
  };
  let width = img.width();
  let height = img.height();

  macro_rules! encode {
    ($rgb:ident,$n:expr) => {{
      let img = img.$rgb();
      let img = EncoderFrame::new(img.as_raw()).num_channels($n);
      let buffer: EncoderResult<u8> = encoder.build()?.encode_frame(&img, width, height)?;
      buffer.data.into()
    }};
  }

  Ok(if has_alpha {
    encode!(into_rgba8, 4)
  } else {
    encode!(into_rgb8, 3)
  })
}

#[allow(dead_code)]
#[napi]
fn img_jxl(bin: Buffer, ext: Option<String>, quality: f64) -> AsyncTask<Pkg> {
  let quality = quality as f32;
  AsyncTask::new(Pkg { bin, quality, ext })
}
