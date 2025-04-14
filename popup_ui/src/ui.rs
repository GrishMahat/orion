use iced::{
    widget::{column, container, scrollable, Row, Text, TextInput},
    Length, Element, Alignment, Color, Theme,
};
use shared::models::{SearchResult, SearchQuery};

// Custom style for selected items
struct SelectedItemStyle;

impl container::StyleSheet for SelectedItemStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Color::from_rgb(0.2, 0.4, 0.8).into()),
            text_color: Some(Color::WHITE),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchInputChanged(String),
    ResultSelected(usize),
    CloseRequested,
    ExecuteCommand,
}

pub struct SearchUI {
    input_value: String,
    results: Vec<SearchResult>,
    selected_idx: Option<usize>,
}

impl Default for SearchUI {
    fn default() -> Self {
        Self {
            input_value: String::new(),
            results: Vec::new(),
            selected_idx: None,
        }
    }
}

impl SearchUI {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update(&mut self, message: Message) -> bool {
        match message {
            Message::SearchInputChanged(value) => {
                self.input_value = value;
                true // Trigger search
            }
            Message::ResultSelected(idx) => {
                if idx < self.results.len() {
                    self.selected_idx = Some(idx);
                }
                false
            }
            Message::CloseRequested => false,
            Message::ExecuteCommand => false,
        }
    }

    pub fn view(&self) -> Element<Message, Theme> {
        let search_input = TextInput::new(
            "Type to search...",
            &self.input_value,
        )
        .on_input(Message::SearchInputChanged)
        .padding(10)
        .size(20);

        let results_list: Element<Message, Theme> = if self.results.is_empty() {
            if !self.input_value.is_empty() {
                column![Text::<Theme>::new("No results found").size(16)]
                    .spacing(10)
                    .into()
            } else {
                column![Text::<Theme>::new("Start typing to search").size(16)]
                    .spacing(10)
                    .into()
            }
        } else {
            let results_widgets: Vec<Element<Message, Theme>> = self.results
                .iter()
                .enumerate()
                .map(|(idx, result)| {
                    let is_selected = self.selected_idx == Some(idx);
                    let result_row = Row::new()
                        .spacing(10)
                        .align_items(Alignment::Center)
                        .push(Text::<Theme>::new(&result.title).size(16))
                        .push(if let Some(desc) = &result.description {
                            Text::<Theme>::new(desc).size(14)
                        } else {
                            Text::<Theme>::new("").size(14)
                        });

                    if is_selected {
                        // For selected item, use a custom style without a closure
                        container(result_row)
                            .style(iced::theme::Container::Custom(Box::new(SelectedItemStyle)))
                            .width(Length::Fill)
                            .padding(5)
                            .into()
                    } else {
                        container(result_row)
                            .width(Length::Fill)
                            .padding(5)
                            .into()
                    }
                })
                .collect();

            scrollable(
                column(results_widgets)
                    .spacing(2)
                    .width(Length::Fill)
            )
            .height(Length::Fill)
            .into()
        };

        column![
            search_input,
            results_list,
        ]
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn set_results(&mut self, results: Vec<SearchResult>) {
        self.results = results;
        if !self.results.is_empty() && self.selected_idx.is_none() {
            self.selected_idx = Some(0);
        } else if self.results.is_empty() {
            self.selected_idx = None;
        } else if let Some(idx) = self.selected_idx {
            if idx >= self.results.len() {
                self.selected_idx = Some(self.results.len() - 1);
            }
        }
    }

    pub fn get_search_query(&self) -> SearchQuery {
        SearchQuery {
            text: self.input_value.clone(),
            max_results: 10,
        }
    }

    pub fn get_selected_result(&self) -> Option<&SearchResult> {
        self.selected_idx.and_then(|idx| self.results.get(idx))
    }

    pub fn select_next(&mut self) {
        if self.results.is_empty() {
            return;
        }

        if let Some(idx) = self.selected_idx {
            if idx < self.results.len() - 1 {
                self.selected_idx = Some(idx + 1);
            }
        } else {
            self.selected_idx = Some(0);
        }
    }

    pub fn select_previous(&mut self) {
        if self.results.is_empty() {
            return;
        }

        if let Some(idx) = self.selected_idx {
            if idx > 0 {
                self.selected_idx = Some(idx - 1);
            }
        } else {
            self.selected_idx = Some(0);
        }
    }
}
