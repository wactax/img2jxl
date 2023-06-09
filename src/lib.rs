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

pub struct Pkg {
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
      if let Some(f) = ImageFormat::from_extension(ext) {
        guessed = false;
        format = f;
        break;
      }
    }
    guessed = true;
    format = image::guess_format(bin)?;
    break;
  }

  let mut bind = encoder_builder();
  // https://docs.rs/jpegxl-rs/latest/jpegxl_rs/encode/struct.JxlEncoderBuilder.html#method.quality
  let mut quality = pkg.quality;
  let mut lossless = quality == 0.0;

  if let ImageFormat::Jpeg = format {
    let mut encoder = bind
      .quality(quality)
      .lossless(lossless)
      .speed(EncoderSpeed::Tortoise)
      .build()?;
    if let Ok(r) = encoder.encode_jpeg(bin) {
      return Ok(r.data.into());
    }
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

  let has_alpha = img.color().has_alpha();
  let encoder = bind.has_alpha(has_alpha);
  let width = img.width();
  let height = img.height();

  let speed = if width > 2800 || height > 2800 {
    // 大图用无损压缩太慢了
    if lossless {
      lossless = false;
      quality = 1.0;
    };
    if width > 7680 || height > 4320 {
      EncoderSpeed::Hare
    } else {
      EncoderSpeed::Squirrel
    }
  } else {
    EncoderSpeed::Tortoise
  };

  let encoder = encoder.quality(quality).speed(speed).lossless(lossless);

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

#[napi]
pub fn img_jxl(bin: Buffer, ext: Option<String>, quality: f64) -> AsyncTask<Pkg> {
  let quality = quality as f32;
  AsyncTask::new(Pkg { bin, quality, ext })
}

// #[cfg(test)]
// mod test {
//   use std::{fmt::format, fs::File, io, io::prelude::*};
//
//   use super::*;
//
//   #[test]
//   fn test() -> anyhow::Result<()> {
//     let dir = std::env::current_dir()?;
//     let img = dir.join("img/1.jpg");
//
//     let mut f = File::open(img)?;
//     let mut bin = Vec::new();
//     f.read_to_end(&mut bin)?;
//
//     let mut bind = encoder_builder();
//     let mut quality = 1.0;
//     let mut lossless = false;
//     let mut encoder = bind
//       .quality(quality)
//       .lossless(lossless)
//       .speed(EncoderSpeed::Tortoise)
//       .build()?;
//
//     let img = image::load_from_memory(&bin)?;
//     let img = img.into_rgb8();
//     // let out = encoder.encode_jpeg(&bin)?.data;
//     let width = img.width();
//     let height = img.height();
//     let img = EncoderFrame::new(img.as_raw()).num_channels(3);
//     let buffer: EncoderResult<u8> = encoder.encode_frame(&img, width, height)?;
//
//     Ok(())
//   }
// }
