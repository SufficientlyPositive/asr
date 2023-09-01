#[allow(dead_code)]

#[derive(PartialEq, Debug)]
pub enum SampleRate {
    R32khz,
    R44_1khz,
    R48khz
}

#[derive(PartialEq, Debug)]
pub enum SampleDepth {
    I16,
    I24,
    I32,
    F32
}

#[derive(PartialEq, Debug)]
pub struct SampleMETA {
    pub sample_rate: SampleRate,
    pub sample_bitdepth: SampleDepth
}

#[derive(Debug)]
struct AudioSample {
    sample: Vec<f64>,
    sample_metadata: SampleMETA
}

impl std::fmt::Display for SampleRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SampleRate::R32khz => write!(f, "32kHz"),
            SampleRate::R44_1khz => write!(f, "44.1kHz"),
            SampleRate::R48khz => write!(f, "48kHz"),
        }
    }
}

impl std::fmt::Display for SampleDepth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SampleDepth::I16 => write!(f, "16 bit integer"),
            SampleDepth::I24 => write!(f, "24 bit integer"),
            SampleDepth::I32 => write!(f, "32 bit integer"),
            SampleDepth::F32 => write!(f, "32 bit float")
        }
    }
}

impl std::fmt::Display for SampleMETA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sample rate: {}, Sample depth: {}.", self.sample_rate, self.sample_bitdepth)
    }
}
