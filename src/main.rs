mod google;
mod deepl;
mod opeanai;

use std::fs::File;
use std::io::BufWriter;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use cpal::{BufferSize, BuildStreamError, Device, Devices, FromSample, InputDevices, Sample, Stream, StreamConfig, SupportedBufferSize, SupportedStreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::WavWriter;
use serde::{Deserialize, Serialize};

type WavWriterHandle = Arc<Mutex<WavWriter<BufWriter<File>>>>;

#[tokio::main]
async fn main() -> hound::Result<()> {

    let host = cpal::default_host();
    let devices = host.input_devices().expect("no input device available");
    let device = select_device(devices).await;


    let mut supported_configs_range = device.supported_input_configs().expect("error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let file_path = "assedf";
    let (stream,handle) = write_waw(&device,&supported_config,file_path).await.expect("cannot create input stream from device");

    stream.play().expect("TODO: panic message");

    // Let recording go for roughly three seconds.
    std::thread::sleep(std::time::Duration::from_secs(3));
    drop(stream);

    handle.lock().as_deref_mut().map(
        |wr| wr.flush()
    ).expect("TODO: panic message")
        .expect("TODO: panic message");

    drop(handle); // Close the file

    Ok(())


}

fn convert_config(config: &SupportedStreamConfig) -> StreamConfig {

    let buf = match config.buffer_size() {
        SupportedBufferSize::Range { min,max } => BufferSize::Fixed(max.clone()),
        SupportedBufferSize::Unknown => BufferSize::Default
    };
    return StreamConfig{
        channels: config.channels(),
        sample_rate: config.sample_rate(),
        buffer_size: buf
    }
}

async fn select_device(device_list: InputDevices<Devices>) -> Device {
    todo!()
}



async fn write_waw(device:&Device, config: &SupportedStreamConfig, file_path:&str) -> Result<(Stream, WavWriterHandle),String> {
    let spec = wav_spec_from_config(&config);
    let str_config = convert_config(&config);
    let writer = hound::WavWriter::create(file_path, spec).map_err(|e| e.to_string())?;
    let writer = Arc::new(Mutex::new(writer));
    let writer2 = writer.clone();

    let err_fn = move |err| {
        eprintln!("an error occurred on stream: {}", err);
    };
   return match config.sample_format() {
        cpal::SampleFormat::I8 => device.build_input_stream(
            &str_config,
            move |data, _: &_| write_input_data::<i8, i8>(data, &writer),
            err_fn,
            None,
        ),
        cpal::SampleFormat::I16 => device.build_input_stream(
            &str_config,
            move |data, _: &_| write_input_data::<i16, i16>(data, &writer),
            err_fn,
            None,
        ),
        cpal::SampleFormat::I32 => device.build_input_stream(
            &str_config,
            move |data, _: &_| write_input_data::<i32, i32>(data, &writer),
            err_fn,
            None,
        ),
        cpal::SampleFormat::F32 => device.build_input_stream(
            &str_config,
            move |data, _: &_| write_input_data::<f32, f32>(data, &writer),
            err_fn,
            None,
        ),
        sample_format => Err(format!("Unsupported sample format '{sample_format}'"))?
    }.map(|stream| (stream,writer2)).map_err(|e|e.to_string());
}



    fn write_input_data<T, U>(input: &[T], writer: &Arc<Mutex<WavWriter<BufWriter<File>>>>)
        where
            T: Sample,
            U: Sample + hound::Sample + FromSample<T>,
    {
        if let Ok(mut guard) = writer.try_lock() {
            for &sample in input.iter() {
                let sample: U = U::from_sample(sample);
                guard.deref_mut().write_sample(sample).ok();
            }
        }
    }

    fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
        if format.is_float() {
            hound::SampleFormat::Float
        } else {
            hound::SampleFormat::Int
        }
    }

    fn wav_spec_from_config(config: &SupportedStreamConfig) -> hound::WavSpec {
        hound::WavSpec {
            channels: config.channels() as _,
            sample_rate: config.sample_rate().0 as _,
            bits_per_sample: (config.sample_format().sample_size() * 8) as _,
            sample_format: sample_format(config.sample_format()),
        }
    }
