use serde::{Deserialize, Serialize};

macro_rules! generate_enum {
    ($first:ident, $($rest:ident),*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Program {
            $first = 0,
            $($rest),*
        }

        impl Program {
            pub fn name(&self) -> String {
                match self {
                    Self::$first => get_name!($first),
                    $(Self::$rest => get_name!($rest)),*
                }
            }
        }
    };
}

macro_rules! get_name {
    ($id:ident) => {
        stringify!($id)
            .chars()
            .enumerate()
            .map(|(i, c)| match c {
                'A'..='Z' => {
                    if i > 0 {
                        format!(" {}", c)
                    } else {
                        c.to_string()
                    }
                }
                _ => c.to_string(),
            })
            .collect::<String>()
    };
}

generate_enum!(
    AcousticGrandPiano,
    BrightAcousticPiano,
    ElectricGrandPiano,
    HonkyTonkPiano,
    ElectricPiano1,
    ElectricPiano2,
    Harpsichord,
    Clavi,
    Celesta,
    Glockenspiel,
    MusicBox,
    Vibraphone,
    Marimba,
    Xylophone,
    TubularBells,
    Dulcimer,
    DrawbarOrgan,
    PercussiveOrgan,
    RockOrgan,
    ChurchOrgan,
    ReedOrgan,
    Accordion,
    Harmonica,
    TangoAccordion,
    AcousticNylonGuitar,
    AcousticSteelGuitar,
    ElectricJazzGuitar,
    ElectricCleanGuitar,
    ElectricMutedGuitar,
    OverdrivenGuitar,
    DistortionGuitar,
    GuitarHarmonics,
    AcousticBass,
    ElectricFingerBass,
    ElectricPickBass,
    FretlessBass,
    SlapBass1,
    SlapBass2,
    SynthBass1,
    SynthBass2,
    Violin,
    Viola,
    Cello,
    Contrabass,
    TremoloStrings,
    PizzicatoStrings,
    OrchestralHarp,
    Timpani,
    StringEnsemble1,
    StringEnsemble2,
    SynthStrings1,
    SynthStrings2,
    ChoirAahs,
    VoiceOohs,
    SynthVoice,
    OrchestraHit,
    Trumpet,
    Trombone,
    Tuba,
    MutedTrumpet,
    FrenchHorn,
    BrassSection,
    SynthBrass1,
    SynthBrass2,
    SopranoSax,
    AltoSax,
    TenorSax,
    BaritoneSax,
    Oboe,
    EnglishHorn,
    Bassoon,
    Clarinet,
    Piccolo,
    Flute,
    Recorder,
    PanFlute,
    BlownBottle,
    Shakuhachi,
    Whistle,
    Ocarina,
    SquareLead,
    SawtoothLead,
    CalliopeLead,
    ChiffLead,
    CharangLead,
    VoiceLead,
    FifthsLead,
    BassAndLead,
    NewAgePad,
    WarmPad,
    PolysynthPad,
    ChoirPad,
    BowedPad,
    MetallicPad,
    HaloPad,
    SweepPad,
    RainFX,
    SoundtrackFX,
    CrystalFX,
    AtmosphereFX,
    BrightnessFX,
    GoblinsFX,
    EchoesFX,
    SciFiFX,
    Sitar,
    Banjo,
    Shamisen,
    Koto,
    Kalimba,
    Bagpipe,
    Fiddle,
    Shanai,
    TinkleBell,
    Agogo,
    SteelDrums,
    Woodblock,
    TaikoDrum,
    MelodicTom,
    SynthDrum,
    ReverseCymbal,
    GuitarFretNoise,
    BreathNoise,
    Seashore,
    BirdTweet,
    TelephoneRing,
    Helicopter,
    Applause,
    Gunshot
);
