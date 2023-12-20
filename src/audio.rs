use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Stream;
use cpal::{SampleFormat, SampleRate, SupportedStreamConfig};
use std::sync::{Arc, Mutex};
use usfx::{Mixer, Sample};

const SAMPLE_RATE: u32 = 44_100;

pub struct Audio {
    mixer: Arc<Mutex<Mixer>>,
    stream: Stream,
}

impl Audio {
    pub fn new() -> Self {
        let mixer = Arc::new(Mutex::new(Mixer::new(SAMPLE_RATE as usize)));
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("no output device available");

        let config = device
            .supported_output_configs()
            .expect("no output configs available")
            .find(|config| config.sample_format() == SampleFormat::F32);

        if config.is_none() {
            panic!("no F32 config available");
        }

        let config = config.unwrap();

        if config.min_sample_rate() > SampleRate(SAMPLE_RATE)
            || config.max_sample_rate() < SampleRate(SAMPLE_RATE)
        {
            panic!("44100 Hz not supported");
        }

        let format = SupportedStreamConfig::new(
            config.channels(),
            SampleRate(SAMPLE_RATE),
            config.buffer_size().clone(),
            SampleFormat::F32,
        );

        let stream_mixer = mixer.clone();

        let stream = device
            .build_output_stream::<f32, _, _>(
                &format.config(),
                move |data, _| stream_mixer.lock().unwrap().generate(data),
                |err| eprintln!("cpal error: {:?}", err),
                None
            )
            .expect("could not build output stream");

        let struct_mixer = mixer;
        Self {
            mixer: struct_mixer,
            stream,
        }
    }

    pub fn play(&mut self, sample: Sample) {
        self.mixer.lock().unwrap().play(sample);
    }

    pub fn run(&mut self) {
        self.stream.play().expect("unable to start stream");
    }
}
