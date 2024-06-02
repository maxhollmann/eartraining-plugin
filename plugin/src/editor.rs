use nih_plug::prelude::{Editor, GuiContext};
use nih_plug_iced::*;
use std::sync::Arc;

use crate::{intervals::Interval, EarTrainerParams, SharedState};

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(200, 150)
}

pub(crate) fn create(
    params: Arc<EarTrainerParams>,
    shared: Arc<SharedState>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<EarTrainerEditor>(editor_state, (params, shared))
}

struct EarTrainerEditor {
    params: Arc<EarTrainerParams>,
    shared: Arc<SharedState>,
    context: Arc<dyn GuiContext>,
    last_interval: Option<Interval>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
}

fn get_interval(note: u32, tonic: u32) -> u32 {
    (note + 12 - tonic) % 12
}

impl IcedEditor for EarTrainerEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = (Arc<EarTrainerParams>, Arc<SharedState>);

    fn new(
        (params, shared): Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = EarTrainerEditor {
            params,
            shared,
            context,
            last_interval: None,
        };

        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        _message: Self::Message,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let tonic: u32 = self.params.tonic.value().midi_index() as u32;

        let mut active_intervals: [bool; 12] = [false; 12];
        let active_notes = self.shared.active_notes.lock().unwrap();
        for (note, active) in active_notes.iter().enumerate() {
            if *active {
                active_intervals[get_interval(note as u32, tonic as u32) as usize] = true;
            }
        }

        let mut row = Row::new().spacing(10).width(Length::Fill).align_items(Alignment::Center);
        let mut any_active = false;
        for (i, active) in active_intervals.iter().enumerate() {
            if *active {
                let interval = Interval::ALL[i];
                self.last_interval = Some(interval);
                row = row.push(interval_text(interval));
                any_active = true;
            }
        };

        if !any_active && self.last_interval.is_some() {
            row = row.push(interval_text(self.last_interval.unwrap()));
        }

        Column::new()
            .align_items(Alignment::Center)
            .push(row)
            .height(Length::Fill)
            .into()
    }

    fn background_color(&self) -> nih_plug_iced::Color {
        nih_plug_iced::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

fn interval_text(interval: Interval) -> Text {
    Text::new(format!("{}", interval))
    .font(assets::NOTO_SANS_BOLD)
    .size(100)
    .height(100.into())
    .width(100.into())
    .horizontal_alignment(alignment::Horizontal::Center)
    .vertical_alignment(alignment::Vertical::Bottom)
    .color(interval.color())
}
