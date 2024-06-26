use intervals::Interval;
use nih_plug::prelude::*;
use nih_plug_iced::IcedState;
use std::sync::{Arc, Mutex};
use sysex::EartrainerSysExMessage;


mod editor;
mod intervals;
mod note;

struct EarTrainer {
    params: Arc<EarTrainerParams>,
    shared: Arc<SharedState>,
}

#[derive(Params)]
struct EarTrainerParams {
    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,

    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "tonic"]
    pub tonic: EnumParam<note::Note>,

    #[nested(group="Active intervals")]
    pub active_intervals: ActiveIntervalsParams,
}
#[derive(Params)]
struct ActiveIntervalsParams {
    #[id = "activate-1"]
    pub activate_1: BoolParam,
    #[id = "activate-b2"]
    activate_b2: BoolParam,
    #[id = "activate-2"]
    activate_2: BoolParam,
    #[id = "activate-b3"]
    activate_b3: BoolParam,
    #[id = "activate-3"]
    activate_3: BoolParam,
    #[id = "activate-4"]
    activate_4: BoolParam,
    #[id = "activate-b5"]
    activate_b5: BoolParam,
    #[id = "activate-5"]
    activate_5: BoolParam,
    #[id = "activate-b6"]
    activate_b6: BoolParam,
    #[id = "activate-6"]
    activate_6: BoolParam,
    #[id = "activate-b7"]
    activate_b7: BoolParam,
    #[id = "activate-7"]
    activate_7: BoolParam,
}

struct SharedState {
    active_notes: Mutex<[bool; u8::MAX as usize + 1]>,
}

impl EarTrainer {
    fn on_note_on(&mut self, note_event: NoteEvent<EartrainerSysExMessage>) {
        if let NoteEvent::NoteOn { note, .. } = note_event {
            let mut active_notes = self.shared.active_notes.lock().unwrap();
            active_notes[note as usize] = true;
        }
    }

    fn on_note_off(&mut self, note_event: NoteEvent<EartrainerSysExMessage>) {
        if let NoteEvent::NoteOff { note, .. } = note_event {
            let mut active_notes = self.shared.active_notes.lock().unwrap();
            active_notes[note as usize] = false;
        }
    }
}

impl Default for EarTrainer {
    fn default() -> Self {
        Self {
            params: Arc::new(EarTrainerParams::default()),
            shared: Arc::new(SharedState {
                active_notes: Mutex::new([false; u8::MAX as usize + 1]),
            }),
        }
    }
}

impl Default for EarTrainerParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            // See the main gain example for more details
            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            tonic: EnumParam::new("Tonic", note::Note::C),

            active_intervals: ActiveIntervalsParams::default(),
        }
    }
}

impl Default for ActiveIntervalsParams {
    fn default() -> Self {
        Self {
            activate_1: BoolParam::new("Activate 1", true),
            activate_b2: BoolParam::new("Activate b2", true),
            activate_2: BoolParam::new("Activate 2", true),
            activate_b3: BoolParam::new("Activate b3", true),
            activate_3: BoolParam::new("Activate 3", true),
            activate_4: BoolParam::new("Activate 4", true),
            activate_b5: BoolParam::new("Activate b5", true),
            activate_5: BoolParam::new("Activate 5", true),
            activate_b6: BoolParam::new("Activate b6", true),
            activate_6: BoolParam::new("Activate 6", true),
            activate_b7: BoolParam::new("Activate b7", true),
            activate_7: BoolParam::new("Activate 7", true),
        }
    }
}

impl Plugin for EarTrainer {
    const NAME: &'static str = "Eartraining";
    const VENDOR: &'static str = "Test";
    const URL: &'static str = "https://youtu.be/dQw4w9WgXcQ";
    const EMAIL: &'static str = "test@example.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];
    const MIDI_INPUT: MidiConfig = MidiConfig::MidiCCs;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::MidiCCs;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = EartrainerSysExMessage;
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.shared.clone(), self.params.editor_state.clone())
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        while let Some(event) = context.next_event() {
            match event {
                NoteEvent::NoteOn { .. } => self.on_note_on(event),
                NoteEvent::NoteOff { .. } => self.on_note_off(event),
                NoteEvent::MidiSysEx { message, .. } => {
                    match message {
                        EartrainerSysExMessage::SetTonic(tonic) => {
                            nih_log!("Received SetTonic message: {}", tonic);
                        }
                    }
                },
                _ => (),
            }
            context.send_event(event);
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for EarTrainer {
    const CLAP_ID: &'static str = "com.maxhollmann.eartrainer";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("An eartraining plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::NoteEffect, ClapFeature::Utility];
}

impl Vst3Plugin for EarTrainer {
    const VST3_CLASS_ID: [u8; 16] = *b"GainGuiIcedAaAAa";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!(EarTrainer);
nih_export_vst3!(EarTrainer);
