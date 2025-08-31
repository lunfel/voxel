use noise::{NoiseFn, Perlin, Seedable};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone)]
pub struct Noise {
    noise: Perlin,
    pub octaves: i32,
    pub frequency: f64,
    pub amplitude: f64,
    pub lacunarity: f64,
    pub gain: f64,
}

impl Serialize for Noise {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Noise", 6)?;
        state.serialize_field("seed", &self.seed())?;
        state.serialize_field("octaves", &self.octaves)?;
        state.serialize_field("frequency", &self.frequency)?;
        state.serialize_field("amplitude", &self.amplitude)?;
        state.serialize_field("lacunarity", &self.lacunarity)?;
        state.serialize_field("gain", &self.gain)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Noise {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct NoiseHelper {
            seed: u32,
            octaves: i32,
            frequency: f64,
            amplitude: f64,
            lacunarity: f64,
            gain: f64,
        }

        let helper = NoiseHelper::deserialize(deserializer)?;

        Ok(Noise {
            noise: Perlin::new(helper.seed),
            octaves: helper.octaves,
            frequency: helper.frequency,
            amplitude: helper.amplitude,
            lacunarity: helper.lacunarity,
            gain: helper.gain,
        })
    }
}

impl Default for Noise {
    fn default() -> Self {
        Noise {
            noise: Perlin::new(0),
            octaves: 1,
            frequency: 120.0,
            amplitude: 40.0,
            lacunarity: 2.0,
            gain: 0.5,
        }
    }
}

impl NoiseFn<f64, 3> for Noise {
    /// values are remapped in the range [0, 1]
    fn get(&self, point: [f64; 3]) -> f64 {
        let offset = 0.1153;

        let offset_point: [f64; 3] = point.map(|x| x + offset);

        let mut total_noise_value = 0.0;

        for i in 0..self.octaves {
            // Lacunarity is the frequency multiplier for each octave. But since we note the
            // frequency as 1/frequency, we need to divide by the frequency of the previous octave instead
            let octave_frequency = self.frequency / self.lacunarity.powi(i + 1);

            let noise_value = self.noise.get(offset_point.map(|x| x / (octave_frequency)));

            total_noise_value +=
                ((noise_value + 1.0) / 2.0) * self.amplitude * self.gain.powi(i + 1);
        }

        total_noise_value
    }
}

impl Noise {
    fn seed(&self) -> u32 {
        self.noise.seed()
    }
}
