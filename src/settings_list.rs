use crate::style;
use crate::view::listing_label;
use crate::view::Elem;
use iced::{button, Column, Container, Length, Space};

pub(crate) struct SettingsList {
    setting_labels: Vec<SettingLabel>,
}

impl SettingsList {
    pub(crate) fn new() -> Self {
        Self {
            setting_labels: vec![SettingLabel::new("Sources")],
        }
    }

    pub(crate) fn view(&mut self, selected_setting: usize) -> Elem<'_> {
        let mut column_content = Column::new();
        for (idx, setting) in self.setting_labels.iter_mut().enumerate() {
            let mut label = listing_label(&mut setting.state, &setting.label);
            if selected_setting == idx {
                label = label.style(style::SelectedLabel);
            }
            column_content = column_content
                .push(label)
                .push(Space::with_height(Length::Fill));
        }

        Container::new(column_content)
            .height(Length::Fill)
            .width(Length::from(150))
            .style(style::QuestionsColumn)
            .center_y()
            .into()
    }
}

#[derive(Default)]
struct SettingLabel {
    state: button::State,
    label: String,
}

impl SettingLabel {
    fn new<S: Into<String>>(label: S) -> Self {
        Self {
            label: label.into(),
            ..Self::default()
        }
    }
}
