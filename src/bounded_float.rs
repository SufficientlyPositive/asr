// https://stackoverflow.com/questions/16755181/type-casting-arrays-vectors-in-rust
// note for better ways of doing transmute than dipping into unsafe, look at the documentation:
// https://doc.rust-lang.org/std/mem/fn.transmute.html

// would be cool to have a better way of doing this
// maybe instead of all this "on trust" stuff, look at Duration in std::time?
// right now this means all Probability interaction should be wrapped in unsafe{} lol

// Type alias' used to indicate range is `[-1.0, 1.0]`.
// This is **not** enforced outside of the functions in this module.
pub type Amplitude32 = f32;
pub type Amplitude64 = f64;

// Type alias' used to indicate range is `[0.0, 1.0]`.
// This is **not** enforced outside of the functions in this module.
pub type Probability32 = f32;
pub type Probability64 = f64;

// Converts a vec of `f32` to a new vec of `Amplitude32` with range `[-1.0, 1.0]`
// 
// May cause a loss of precision if the vector contains values whose absolute value is greater than 1.0.
// Does not do anything other than copy the vec if all values are already within the range.
pub fn cast_to_amplitude32(vec: Vec<f32>) -> Vec<Amplitude32> {
    let max = vec.iter().map(|f| f.abs()).fold(f32::NEG_INFINITY, f32::max);

    if max > 1.0 {
        vec.iter().map(|&e| e / max as Amplitude32).collect()
    } else {
        vec.iter().map(|&e| e as Amplitude32).collect()
    }
}

pub fn cast_to_scaled_amplitude32(vec: Vec<f32>) -> Vec<Amplitude32> {
    let max = vec.iter().map(|f| f.abs()).fold(f32::NEG_INFINITY, f32::max);
    trace!("(cast_to_scaled_amplitude32) Max calculated as: {}", max);
    vec.iter().map(|e| {
        if max == 0.0 {
            0.0
        } else {
            e / max as Amplitude32
        }
    }).collect()
}

// Converts a vec of `Amplitude32` to new a vec of `f32`
pub fn cast_from_amplitude32(vec: Vec<Amplitude32>) -> Vec<f32> {
    vec.iter().map(|&v| v as f32).collect()
}

// Converts a vec of `f64` to a new vec of `Amplitude64` with range `[-1.0, 1.0]`
// 
// May cause a loss of precision if the vector contains values whose absolute value is greater than 1.0.
// Does not do anything other than copy the vec if all values are already within the range.
pub fn cast_to_amplitude64(vec: Vec<f64>) -> Vec<Amplitude64> {
    let max = vec.iter().map(|f| f.abs()).fold(f64::NEG_INFINITY, f64::max);

    if max > 1.0 {
        vec.iter().map(|&e| e / max as Amplitude64).collect()
    } else {
        vec.iter().map(|&e| e as Amplitude64).collect()
    }
}

// Converts a vec of `Amplitude64` to new a vec of `f64`
pub fn cast_from_amplitude64(vec: Vec<Amplitude64>) -> Vec<f64> {
    vec.iter().map(|&v| v as f64).collect()
}

// Converts a vec of `f32` to a new vec of `Probability32` with range `[0.0, 1.0]`
// 
// Any values outside the range are sent to `0.0` and `1.0` depending on which side of the range they fall.
// Does not do anything other than copy the vec if all values are already within the range.
pub fn cast_to_probability32(vec: Vec<f32>) -> Vec<Probability32> {
    vec.iter().map(|&e| {
        if e > 1.0 {
            1.0
        } else if e < 0.0 {
            0.0
        } else {
            e as Probability32
        }
    }).collect()
}

// Converts a vec of `Probability32` to new a vec of `f32`
pub fn cast_from_probability32(vec: Vec<Probability32>) -> Vec<f32> {
    vec.iter().map(|&v| v as f32).collect()
}

// Converts a vec of `f64` to a new vec of `Probability64` with range `[0.0, 1.0]`
// 
// Any values outside the range are sent to `0.0` and `1.0` depending on which side of the range they fall.
// Does not do anything other than copy the vec if all values are already within the range.
pub fn cast_to_probability64(vec: Vec<f64>) -> Vec<Probability64> {
    vec.iter().map(|&e| {
        if e > 1.0 {
            1.0
        } else if e < 0.0 {
            0.0
        } else {
            e as Probability64
        }
    }).collect()
} 

// Converts a vec of `Amplitude64` to new a vec of `f64`
pub fn cast_from_probability64(vec: Vec<Probability64>) -> Vec<f64> {
    vec.iter().map(|&v| v as f64).collect()
}