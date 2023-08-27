#[allow(dead_code)]

#[derive(PartialEq, Debug)]
pub enum SampleRate {
    R32khz,
    R44_1khz,
    R48khz
}

#[derive(PartialEq, Debug)]
pub enum SampleDepth {
    D16,
    D24,
    D32
}

#[derive(PartialEq, Debug)]
pub struct SampleMETA {
    sample_rate: SampleRate,
    sample_bitdepth: SampleDepth
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
            SampleDepth::D16 => write!(f, "16 bits"),
            SampleDepth::D24 => write!(f, "24 bits"),
            SampleDepth::D32 => write!(f, "32 bits"),
        }
    }
}

impl std::fmt::Display for SampleMETA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sample rate: {}, Sample depth: {}.", self.sample_rate, self.sample_bitdepth)
    }
}
