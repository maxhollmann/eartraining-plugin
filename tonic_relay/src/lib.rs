use nih_plug::prelude::*;
use std::sync::Arc;
use sysex::EartrainerSysExMessage;

struct TonicRelay {
    params: Arc<TonicRelayParams>,
}

#[derive(Params)]
struct TonicRelayParams {
}


impl Default for TonicRelay {
    fn default() -> Self {
        Self {
            params: Arc::new(TonicRelayParams::default()),
        }
    }
}

impl Default for TonicRelayParams {
    fn default() -> Self {
        Self {
        }
    }
}

impl Plugin for TonicRelay {
    const NAME: &'static str = "Eartraining Tonic Relay";
    const VENDOR: &'static str = "Test";
    const URL: &'static str = "https://youtu.be/dQw4w9WgXcQ";
    const EMAIL: &'static str = "test@example.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[];
    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::Basic;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = EartrainerSysExMessage;
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        None
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
                NoteEvent::NoteOn { note, timing, .. } => {
                    // context.send_event(NoteEvent::MidiSysEx { timing, message: EartrainerSysExMessage::SetTonic(note) });
                    context.send_event(NoteEvent::NoteOn { note: note % 12, timing, channel: 0, velocity: 100.0, voice_id: None  });
                },
                NoteEvent::NoteOff { note, timing, .. } => {
                    context.send_event(NoteEvent::NoteOff { note: note % 12, timing, channel: 0, velocity: 100.0, voice_id: None  });
                },
                _ => (),
            }
            context.send_event(event);
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for TonicRelay {
    const CLAP_ID: &'static str = "com.maxhollmann.eartrainer-tonicrelay";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Eartraining tonic relay");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::NoteEffect, ClapFeature::Utility];
}

nih_export_clap!(TonicRelay);
