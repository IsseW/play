mod functions;
use functions::*;

pub enum Phonetics {
    A,
    E,
    I,
    O,
    K,
    L,
    M,
    S,
}

impl Phonetics {
    pub fn hard_stop(&self) -> bool {
        match self {
            Phonetics::A => false,
            Phonetics::E => false,
            Phonetics::I => false,
            Phonetics::O => false,
            Phonetics::K => true,
            Phonetics::L => true,
            Phonetics::M => false,
            Phonetics::S => true,
        }
    }
    pub fn sample(&self, t: f64) -> f64 {
        match self {
            Phonetics::A => sin
                .mix([1.0, 3.0, 4.0, 6.0, 7.0, 9.0, 11.0, 12.0], ())
                .sample(t),
            Phonetics::E => todo!(),
            Phonetics::I => todo!(),
            Phonetics::O => sin
                .blend(tri, 0.025 + sin(t / 8.0) * 0.03)
                .mix([1.2, 3.0, 9.0, 27.0], pow_sampler::<1.1>)
                .add(sin.mix([2.5, 4.5, 9.0, 27.0], pow_sampler::<3.0>).mul(0.5))
                .add(
                    0.05.mul(tri(t * 3.0)).mul(
                        sin.mix::<2, _, _>(pow_sampler::<2.0>, pow_sampler::<3.0>)
                            .chain(lin::<0.5>),
                    ),
                )
                .add(
                    sin.mix([2.5, 4.5, 9.0, 27.0], pow_sampler::<3.0>)
                        .chain(1.0 / 1.5)
                        .mul(0.3),
                )
                .sample(t),
            Phonetics::K => todo!(),
            Phonetics::L => sin
                .chain(
                    sin.chain(pow::<3.0>.chain(tri.chain(lin::<0.001>)).mul(10.0))
                        .add(lin::<0.5>),
                )
                .sample(t),
            Phonetics::M => todo!(),
            Phonetics::S => todo!(),
        }
    }
}

// Where the sound is formed
enum Place {
    Bilabial,
    Labiodental,
    Dental,
    Alveolar,
    Postalveolar,
    Retroflex,
    Palatal,
    Velar,
    Uvular,
    Pharyngeal,
    Glottal,
}

enum Former {
    Plosive,
    Nasal,
    Fricative,
    Trill,
    Tap,
    Flap,
    LateralFricative,
    Approximant,
    LateralApproximant,
    Ejective,
    Click,
    Implosive,
}

enum Phonetic {
    Consonant {
        voiced: bool,
        place: Place,
        former: Former,
    },
    Vowel {
        open: f64,
        place: f64,
    },
    Break,
}
