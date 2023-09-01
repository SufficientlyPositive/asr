#[allow(dead_code)]
pub enum BoundFloatError {
    ValueOutOfBounds(String)
}


/**
 * A wrapper struct for f64 that guarantees values are within the range [0, 1]
 * This allows for 
 */
#[derive(Clone, Debug)]
pub struct Probability(f64);

impl Probability {
    const LOW: f64 = 0.0;
    const HIGH: f64 = 1.0;

    pub fn new(n: f64) -> Self {
        Probability(n.min(Self::HIGH).max(Self::LOW))
    }

    pub fn fallible_new(n: f64) -> Result<Self, BoundFloatError> {
        match n {
            n if n < Self::LOW => Err(BoundFloatError::ValueOutOfBounds(format!("Value ({}) too low. Minimum value is {}", n, Self::LOW))),
            n if n > Self::HIGH => Err(BoundFloatError::ValueOutOfBounds(format!("Value ({}) too high. Maximum value is {}", n, Self::HIGH))),
            n => Ok(Probability(n)),
        }
    }

    pub fn set(&mut self, n: f64) {
        *self = Probability(n.min(Self::HIGH).max(Self::LOW))
    }

    pub fn as_f64(&self) -> f64 {
        self.0
    }

    /**
     * This function chops any value outside its bounds down to size.
     * [-2.0, 0.5, 1.0, 4.0] would become [-1.0, 0.5, 1.0, 1.0]
     */
    pub fn to_bounded_vec(vec: Vec<f64>) -> Vec<Self> {
        vec.into_iter()
            .map(|f| Probability::new(f))
            .collect()
    }

    /**
     * This function scales any value by the maximum absolute value found.
     * [-2.0, 0.5, 1.0, 4.0] would become [-0.5, 0.125, 0.25, 1.0]
     */
    pub fn to_scaled_vec(vec: Vec<f64>) -> Vec<Self> {
        let max = vec.iter()
            .map(|f| f.abs())
            .fold(f64::NEG_INFINITY, f64::max);
        
        vec.into_iter()
            .map(|f| Probability::new(f / max))
            .collect()
    }
}

impl std::ops::Deref for Probability {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Add for Probability {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Probability::new(self.0 + rhs.0)
    }
}

impl std::ops::Mul for Probability {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Probability::new(self.0 * rhs.0)
    }
}

impl std::fmt::Display for Probability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// wrapper struct for f32 ensuring value remains between -1 and 1.
// a bit dumb, but looks like importing anything with format f32 will just straight up be guaranteed this...
// fml though, don't do any maths with this shit lmao either allow maths operations of itself (like +=) or easy wrapping/unwrapping
// refactor this properly instead of doing what is currently here which is dirty, and unless the compiler is a gigawizard, slow af
// actually, see what these ops compile to before making judgements... but ye is probably garbage.
//
// might be a good idea to remove the type altogether, have it as an alias for f32 and have a conversion function for &[f32] that spits out
// a scaled &[f32] that is in the desired range. This is much weaker ofc but is probably actually good performance compared to this shit.
// ofc need to test, but still...
#[derive(Clone, Debug)]
pub struct Amplitude32(f32);

impl Amplitude32 {
    const LOW: f32 = 0.0;
    const HIGH: f32 = 1.0;

    pub fn new(n: f32) -> Self {
        Amplitude32(n.min(Self::HIGH).max(Self::LOW))
    }

    pub fn fallible_new(n: f32) -> Result<Self, BoundFloatError> {
        match n {
            n if n < Self::LOW => Err(BoundFloatError::ValueOutOfBounds(format!("Value ({}) too low. Minimum value is {}", n, Self::LOW))),
            n if n > Self::HIGH => Err(BoundFloatError::ValueOutOfBounds(format!("Value ({}) too high. Maximum value is {}", n, Self::HIGH))),
            n => Ok(Amplitude32(n)),
        }
    }

    pub fn set(&mut self, n: f32) {
        *self = Amplitude32(n.min(Self::HIGH).max(Self::LOW))
    }

    pub fn as_f32(&self) -> f32 {
        self.0
    }

    pub fn max(self, other: Amplitude32) -> Amplitude32 {
        Amplitude32::new(f32::max(self.0, other.0))
    }

    /**
     * This function chops any value outside its bounds down to size.
     * [-2.0, 0.5, 1.0, 4.0] would become [-1.0, 0.5, 1.0, 1.0]
     */
    pub fn to_bounded_vec(vec: Vec<f32>) -> Vec<Self> {
        vec.into_iter()
            .map(|f| Amplitude32::new(f))
            .collect()
    }

    /**
     * This function scales any value by the maximum absolute value found.
     * [-2.0, 0.5, 1.0, 4.0] would become [-0.5, 0.125, 0.25, 1.0]
     */
    pub fn to_scaled_vec(vec: Vec<f32>) -> Vec<Self> {
        let max = vec.iter()
            .map(|f| f.abs())
            .fold(f32::NEG_INFINITY, f32::max);
        
        vec.into_iter()
            .map(|f| Amplitude32::new(f / max))
            .collect()
    }
}

impl std::ops::Deref for Amplitude32 {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Add for Amplitude32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Amplitude32::new(self.0 + rhs.0)
    }
}

impl std::ops::Mul for Amplitude32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Amplitude32::new(self.0 * rhs.0)
    }
}

impl std::fmt::Display for Amplitude32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// wrapper struct for f64 ensuring value remains between -1 and 1.
#[derive(Clone, Debug)]
pub struct Amplitude64(f64);

impl Amplitude64 {
    const LOW: f64 = 0.0;
    const HIGH: f64 = 1.0;

    pub fn new(n: f64) -> Self {
        Amplitude64(n.min(Self::HIGH).max(Self::LOW))
    }

    pub fn fallible_new(n: f64) -> Result<Self, BoundFloatError> {
        match n {
            n if n < Self::LOW => Err(BoundFloatError::ValueOutOfBounds(format!("Value ({}) too low. Minimum value is {}", n, Self::LOW))),
            n if n > Self::HIGH => Err(BoundFloatError::ValueOutOfBounds(format!("Value ({}) too high. Maximum value is {}", n, Self::HIGH))),
            n => Ok(Amplitude64(n)),
        }
    }

    pub fn set(&mut self, n: f64) {
        *self = Amplitude64(n.min(Self::HIGH).max(Self::LOW))
    }

    pub fn as_f64(&self) -> f64 {
        self.0
    }

    /**
     * This function chops any value outside its bounds down to size.
     * [-2.0, 0.5, 1.0, 4.0] would become [-1.0, 0.5, 1.0, 1.0]
     */
    pub fn to_bounded_vec(vec: Vec<f64>) -> Vec<Self> {
        vec.into_iter()
            .map(|f| Amplitude64::new(f))
            .collect()
    }

    /**
     * This function scales any value by the maximum absolute value found.
     * [-2.0, 0.5, 1.0, 4.0] would become [-0.5, 0.125, 0.25, 1.0]
     */
    pub fn to_scaled_vec(vec: Vec<f64>) -> Vec<Self> {
        let max = vec.iter()
            .map(|f| f.abs())
            .fold(f64::NEG_INFINITY, f64::max);
        
        vec.into_iter()
            .map(|f| Amplitude64::new(f / max))
            .collect()
    }
}

impl std::ops::Deref for Amplitude64 {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Add for Amplitude64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Amplitude64::new(self.0 + rhs.0)
    }
}

impl std::ops::Mul for Amplitude64 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Amplitude64::new(self.0 * rhs.0)
    }
}

impl std::fmt::Display for Amplitude64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
