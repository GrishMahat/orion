use iced::{alignment, Background, Color, Element, Length, Theme};
use iced::widget::{
    button, checkbox, column, container, horizontal_space, row, slider, text, text_input, Space,
    vertical_space,
};
use iced::theme;

use crate::app::Message;
use crate::state::{AppTheme, State, Tab};

// Define Color Constants
const ACCENT_COLOR: Color = Color::from_rgb(0.35, 0.56, 0.98); // Shared Accent

// Dark Theme Colors
const DARK_BACKGROUND: Color = Color::from_rgb(0.13, 0.14, 0.17);
const DARK_SIDEBAR: Color = Color::from_rgb(0.16, 0.17, 0.2);
const DARK_TEXT_PRIMARY: Color = Color::from_rgb(0.9, 0.9, 0.9);
const DARK_TEXT_SECONDARY: Color = Color::from_rgb(0.7, 0.7, 0.7);
const DARK_BORDER: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.1);
const DARK_HOVER: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.05);

// Light Theme Colors
const LIGHT_BACKGROUND: Color = Color::from_rgb(0.96, 0.97, 0.98);
const LIGHT_SIDEBAR: Color = Color::from_rgb(0.9, 0.91, 0.93);
const LIGHT_TEXT_PRIMARY: Color = Color::from_rgb(0.1, 0.1, 0.1);
const LIGHT_TEXT_SECONDARY: Color = Color::from_rgb(0.3, 0.3, 0.3);
const LIGHT_BORDER: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.1);
const LIGHT_HOVER: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.05);

// --- Style Definitions ---

#[derive(Clone, Copy, Default)]
pub struct AppContainerStyle {
    theme: AppTheme,
}

impl container::StyleSheet for AppContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        let bg_color = match self.theme {
            AppTheme::Light => LIGHT_BACKGROUND,
            AppTheme::Dark | AppTheme::System => DARK_BACKGROUND,
        };
        container::Appearance {
            background: Some(Background::Color(bg_color)),
            text_color: None, // Inherited
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct SidebarContainerStyle {
    theme: AppTheme,
}

impl container::StyleSheet for SidebarContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        let bg_color = match self.theme {
            AppTheme::Light => LIGHT_SIDEBAR,
            AppTheme::Dark | AppTheme::System => DARK_SIDEBAR,
        };
        container::Appearance {
            background: Some(Background::Color(bg_color)),
            text_color: None, // Inherited
            border: iced::Border::default(), // No border for sidebar itself
            shadow: iced::Shadow::default(),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct ContentContainerStyle {
    theme: AppTheme,
}

impl container::StyleSheet for ContentContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        let bg_color = match self.theme {
            AppTheme::Light => LIGHT_BACKGROUND,
            AppTheme::Dark | AppTheme::System => DARK_BACKGROUND,
        };
        container::Appearance {
            background: Some(Background::Color(bg_color)),
            text_color: None, // Inherited
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct TabButtonStyle {
    theme: AppTheme,
    accent_color: Color,
    is_selected: bool,
}

impl button::StyleSheet for TabButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let (text_color, background) = if self.is_selected {
            (
                self.accent_color,
                Some(Background::Color(Color {
                    a: 0.2, ..self.accent_color
                })),
            )
        } else {
            (
                match self.theme {
                    AppTheme::Light => LIGHT_TEXT_SECONDARY,
                    AppTheme::Dark | AppTheme::System => DARK_TEXT_SECONDARY,
                },
                None,
            )
        };

        button::Appearance {
            background,
            text_color,
            border: iced::Border {
                radius: 6.0.into(),
                ..Default::default()
            },
            shadow: iced::Shadow::default(),
            shadow_offset: iced::Vector::default(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        if !self.is_selected {
            button::Appearance {
                background: Some(Background::Color(match self.theme {
                    AppTheme::Light => LIGHT_HOVER,
                    AppTheme::Dark | AppTheme::System => DARK_HOVER,
                })),
                text_color: match self.theme {
                    AppTheme::Light => LIGHT_TEXT_PRIMARY,
                    AppTheme::Dark | AppTheme::System => DARK_TEXT_PRIMARY,
                },
                ..active
            }
        } else {
            active
        }
    }
}

#[derive(Clone, Copy)]
pub struct ColorButtonStyle {
    color: Color,
    theme: AppTheme,
}

impl button::StyleSheet for ColorButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let border_color = match self.theme {
            AppTheme::Light => LIGHT_BORDER,
            AppTheme::Dark | AppTheme::System => DARK_BORDER,
        };
        button::Appearance {
            background: Some(Background::Color(self.color)),
            text_color: Color::TRANSPARENT, // No text
            border: iced::Border {
                radius: 4.0.into(),
                width: 1.0,
                color: border_color,
            },
            shadow: iced::Shadow::default(),
            shadow_offset: iced::Vector::default(),
        }
    }
}

// Helper to get text color based on theme
fn get_text_color(theme: AppTheme) -> Color {
    match theme {
        AppTheme::Light => LIGHT_TEXT_PRIMARY,
        AppTheme::Dark | AppTheme::System => DARK_TEXT_PRIMARY,
    }
}

fn get_text_secondary_color(theme: AppTheme) -> Color {
    match theme {
        AppTheme::Light => LIGHT_TEXT_SECONDARY,
        AppTheme::Dark | AppTheme::System => DARK_TEXT_SECONDARY,
    }
}

pub fn view(state: &State) -> Element<Message> {
    let theme = state.theme;
    let accent_color = state.accent_color;
    let text_color = get_text_color(theme);
    let text_secondary_color = get_text_secondary_color(theme);

    let title = row![
        text("Orion").size(28).style(accent_color),
        text(" Settings").size(28).style(text_color),
    ]
    .spacing(5)
    .align_items(alignment::Alignment::Center);

    // Sidebar with navigation tabs
    let tab_button = |label: &str, tab: Tab, icon: &str| {
        let is_selected = state.active_tab == tab;

        let icon_text = text(icon).size(16).style(if is_selected {
            accent_color
        } else {
            text_secondary_color
        });

        let btn_label = text(label)
            .size(14)
            .horizontal_alignment(alignment::Horizontal::Left)
            .style(if is_selected {
                accent_color
            } else {
                text_secondary_color
            });

        button(
            row![
                icon_text,
                Space::with_width(Length::Fixed(10.0)),
                btn_label,
            ]
            .spacing(8)
            .align_items(alignment::Alignment::Center)
            .width(Length::Fill),
        )
        .padding(12)
        .width(Length::Fill)
        .style(theme::Button::Custom(Box::new(TabButtonStyle {
            theme,
            accent_color,
            is_selected,
        })))
        .on_press(Message::TabSelected(tab))
    };

    let sidebar = column![
        title,
        vertical_space().height(Length::from(30)),
        tab_button("General", Tab::General, "⚙"),
        tab_button("Hotkeys", Tab::Hotkeys, "⌨"),
        tab_button("Appearance", Tab::Appearance, "🎨"),
        tab_button("Advanced", Tab::Advanced, "🔧"),
        vertical_space().height(Length::Fill),
        text("v0.1.0") // Update version if needed
            .size(12)
            .style(text_secondary_color)
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center),
        vertical_space().height(Length::from(10)),
    ]
    .padding([20, 20, 10, 20]) // Adjust bottom padding
    .spacing(8)
    .width(Length::Fixed(200.0))
    .height(Length::Fill);

    let sidebar_container = container(sidebar)
        .width(Length::Fixed(200.0))
        .height(Length::Fill)
        .style(theme::Container::Custom(Box::new(
            SidebarContainerStyle { theme },
        )));

    // Content based on selected tab
    let content = match state.active_tab {
        Tab::General => general_tab(state),
        Tab::Hotkeys => hotkeys_tab(state),
        Tab::Appearance => appearance_tab(state),
        Tab::Advanced => advanced_tab(state),
    };

    let content_container = container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(30) // Increased padding
        .style(theme::Container::Custom(Box::new(
            ContentContainerStyle { theme },
        )));

    // Main layout
    let layout = row![sidebar_container, content_container]
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(1); // Minimal spacing for a subtle border effect

    container(layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::Container::Custom(Box::new(AppContainerStyle {
            theme,
        })))
        .into()
}

// --- Helper Widgets ---

fn section_title(title: &str, theme: AppTheme) -> Element<'static, Message> {
    let text_color = get_text_color(theme);
    row![text(title).size(20).style(text_color),]
        .padding([5, 0, 15, 0]) // Adjusted padding
        .width(Length::Fill)
        .into()
}

fn setting_row<'a>(
    label: &str,
    component: Element<'a, Message>,
    theme: AppTheme,
) -> Element<'a, Message> {
    let text_color = get_text_color(theme);
    row![
        text(label).size(14).style(text_color),
        horizontal_space().width(Length::Fill),
        component
    ]
    .padding(15) // Increased padding
    .spacing(15)
    .width(Length::Fill)
    .align_items(alignment::Alignment::Center) // Center items vertically
    .into()
}

// --- Tab Implementations ---

fn general_tab(state: &State) -> Element<Message> {
    let theme = state.theme;
    let voice_toggle = setting_row(
        "Enable voice",
        checkbox("", state.voice_enabled)
            .on_toggle(Message::ToggleVoice)
            .into(),
        theme,
    );

    let auto_update = setting_row(
        "Auto updates",
        checkbox("", true) // Placeholder value
            // .on_toggle(|_| Message::UpdateSetting("auto_update".to_string(), "true".to_string())) // Add logic if needed
            .into(),
        theme,
    );

    let startup = setting_row(
        "Launch on startup",
        checkbox("", false) // Placeholder value
            // .on_toggle(|_| Message::UpdateSetting("launch_on_startup".to_string(), "false".to_string())) // Add logic if needed
            .into(),
        theme,
    );

    column![
        section_title("General Settings", theme),
        voice_toggle,
        auto_update,
        startup,
    ]
    .spacing(5)
    .width(Length::Fill)
    .into()
}

fn hotkeys_tab(state: &State) -> Element<Message> {
    let theme = state.theme;
    let hotkey_edit = setting_row(
        "Activation shortcut",
        text_input("Enter hotkey", &state.hotkey)
            .padding(10) // Adjusted padding
            .width(Length::Fixed(200.0))
            .on_input(Message::UpdateHotkey)
            .into(),
        theme,
    );

    let alt_hotkey = setting_row(
        "Alternative shortcut",
        text_input("Enter hotkey", "Alt+Space") // Placeholder
            .padding(10)
            .width(Length::Fixed(200.0))
            // .on_input(...) // Add handler if needed
            .into(),
        theme,
    );

    column![
        section_title("Keyboard Shortcuts", theme),
        hotkey_edit,
        alt_hotkey,
    ]
    .spacing(5)
    .width(Length::Fill)
    .into()
}

fn appearance_tab(state: &State) -> Element<Message> {
    let theme = state.theme;
    let accent_color = state.accent_color;

    let theme_button = |label: &str, value: AppTheme, current: AppTheme| {
        let is_selected = value == current;
        button(text(label).size(14))
            .padding(10)
            .style(if is_selected {
                theme::Button::Primary
            } else {
                theme::Button::Secondary
            })
            .on_press(Message::SetTheme(value))
    };

    let theme_selector = setting_row(
        "Theme",
        row![
            theme_button("Light", AppTheme::Light, theme),
            theme_button("Dark", AppTheme::Dark, theme),
            // theme_button("System", AppTheme::System, theme), // Add when implemented
        ]
        .spacing(10)
        .into(),
        theme,
    );

    let sensitivity_slider = setting_row(
        "Sensitivity",
        column![
            slider(0.0..=1.0, state.sensitivity, Message::UpdateSensitivity)
                .width(Length::Fixed(200.0)),
            row![text(format!(
                "{}%",
                (state.sensitivity * 100.0) as i32
            ))
            .size(12)
            .style(get_text_secondary_color(theme))]
            .width(Length::Fixed(200.0))
            .align_items(alignment::Alignment::Center),
        ]
        .spacing(5)
        .into(),
        theme,
    );

    let color_button = |color: Color, current_accent: Color| -> Element<Message> {
        let is_selected = color == current_accent; // TODO: Check color equality properly
        button(
            // Optionally add a checkmark or visual indicator if selected
            text(if is_selected { "✔" } else { "" })
                .size(14)
                .style(theme::Text::Color(Color::WHITE)), // Specify Text color style
        )
        .width(Length::Fixed(32.0))
        .height(Length::Fixed(32.0))
        .style(theme::Button::Custom(Box::new(ColorButtonStyle {
            color,
            theme,
        })))
        .on_press(Message::SetAccentColor(color))
        .into() // Convert Button to Element here
    };

    let accent_colors = vec![
        Color::from_rgb(0.35, 0.56, 0.98), // Default Blue
        Color::from_rgb(0.9, 0.3, 0.3),    // Red
        Color::from_rgb(0.3, 0.8, 0.5),    // Green
        Color::from_rgb(0.8, 0.5, 0.9),    // Purple
    ];

    let accent_color_selector = setting_row(
        "Accent color",
        row(accent_colors
            .into_iter()
            .map(|color| color_button(color, accent_color))
            .collect::<Vec<_>>())
        .spacing(10)
        .into(),
        theme,
    );

    column![
        section_title("Appearance", theme),
        theme_selector,
        sensitivity_slider,
        accent_color_selector,
    ]
    .spacing(5)
    .width(Length::Fill)
    .into()
}

fn advanced_tab(state: &State) -> Element<Message> {
    let theme = state.theme;
    let text_secondary_color = get_text_secondary_color(theme);

    let log_level_button = |label: &str, is_selected: bool| {
        button(text(label).size(14))
            .padding(10)
            .style(if is_selected {
                theme::Button::Primary
            } else {
                theme::Button::Secondary
            })
        // .on_press(...) // Add handler
    };

    let log_level = setting_row(
        "Log level",
        row![
            log_level_button("Info", true), // Placeholder
            log_level_button("Debug", false),
            log_level_button("Trace", false),
        ]
        .spacing(10)
        .into(),
        theme,
    );

    let cache_setting = setting_row(
        "Clear cache",
        button(text("Clear").size(14))
            .padding(10)
            .style(theme::Button::Destructive)
            // .on_press(Message::ClearCache) // Add handler
            .into(),
        theme,
    );

    let export_setting = setting_row(
        "Export settings",
        button(text("Export").size(14))
            .padding(10)
            .style(theme::Button::Secondary)
            .on_press(Message::SaveConfig)
            .into(),
        theme,
    );

    column![
        section_title("Advanced Settings", theme),
        log_level,
        cache_setting,
        export_setting,
        vertical_space().height(Length::Fixed(20.0)),
        text("Warning: These settings are for advanced users only.")
            .size(12)
            .style(Color::from_rgb(0.9, 0.6, 0.2)), // Warning color (might need adjustment for light theme)
    ]
    .spacing(5)
    .width(Length::Fill)
    .into()
} 