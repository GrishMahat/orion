use iced::{alignment, Background, Color, Element, Length, Theme};
use iced::widget::{
    button, checkbox, column, container, horizontal_space, row, slider, text, text_input, Space,
    vertical_space, pick_list, scrollable,
};
use iced::theme;

use crate::app::AppMessage;
use crate::state::{AppTheme, State, Tab};

// Define Color Constants

// Dark Theme Colors
const DARK_BACKGROUND: Color = Color::from_rgb(0.11, 0.12, 0.14);
const DARK_SIDEBAR: Color = Color::from_rgb(0.14, 0.15, 0.18);
const DARK_CONTENT: Color = Color::from_rgb(0.13, 0.14, 0.17);
const DARK_TEXT_PRIMARY: Color = Color::from_rgb(0.97, 0.97, 0.97);
const DARK_TEXT_SECONDARY: Color = Color::from_rgb(0.75, 0.75, 0.75);
const DARK_BORDER: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.18);
const DARK_HOVER: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.1);
const DARK_CARD: Color = Color::from_rgb(0.16, 0.17, 0.2);

// Light Theme Colors
const LIGHT_BACKGROUND: Color = Color::from_rgb(0.96, 0.97, 0.98);
const LIGHT_SIDEBAR: Color = Color::from_rgb(0.90, 0.91, 0.93);
const LIGHT_CONTENT: Color = Color::from_rgb(0.99, 0.99, 0.99);
const LIGHT_TEXT_PRIMARY: Color = Color::from_rgb(0.1, 0.1, 0.1);
const LIGHT_TEXT_SECONDARY: Color = Color::from_rgb(0.35, 0.35, 0.35);
const LIGHT_BORDER: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.18);
const LIGHT_HOVER: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.08);
const LIGHT_CARD: Color = Color::from_rgb(1.0, 1.0, 1.0);

// Common element radius
const BORDER_RADIUS: f32 = 10.0;

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
            border: iced::Border {
                radius: BORDER_RADIUS.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 5.0,
            },
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
            AppTheme::Light => LIGHT_CONTENT,
            AppTheme::Dark | AppTheme::System => DARK_CONTENT,
        };
        container::Appearance {
            background: Some(Background::Color(bg_color)),
            text_color: None, // Inherited
            border: iced::Border {
                radius: BORDER_RADIUS.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 5.0,
            },
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct CardContainerStyle {
    theme: AppTheme,
}

impl container::StyleSheet for CardContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        let (bg_color, border_color) = match self.theme {
            AppTheme::Light => (LIGHT_CARD, LIGHT_BORDER),
            AppTheme::Dark | AppTheme::System => (DARK_CARD, DARK_BORDER),
        };
        container::Appearance {
            background: Some(Background::Color(bg_color)),
            text_color: None, // Inherited
            border: iced::Border {
                radius: (BORDER_RADIUS - 2.0).into(),
                width: 1.0,
                color: border_color,
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
                offset: iced::Vector::new(0.0, 3.0),
                blur_radius: 5.0,
            },
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
        let (text_color, background, border_color) = if self.is_selected {
            (
                self.accent_color,
                Some(Background::Color(Color {
                    a: 0.15, ..self.accent_color
                })),
                self.accent_color,
            )
        } else {
            (
                match self.theme {
                    AppTheme::Light => LIGHT_TEXT_SECONDARY,
                    AppTheme::Dark | AppTheme::System => DARK_TEXT_SECONDARY,
                },
                None,
                Color::TRANSPARENT,
            )
        };

        button::Appearance {
            background,
            text_color,
            border: iced::Border {
                radius: (BORDER_RADIUS - 2.0).into(),
                width: if self.is_selected { 1.0 } else { 0.0 },
                color: border_color,
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
            text_color: Color::WHITE,
            border: iced::Border {
                radius: (BORDER_RADIUS - 3.0).into(),
                width: 2.0,
                color: border_color,
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.2),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 3.0,
            },
            shadow_offset: iced::Vector::default(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            border: iced::Border {
                width: 2.0,
                color: Color::WHITE,
                ..active.border
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: iced::Vector::new(0.0, 3.0),
                blur_radius: 5.0,
            },
            ..active
        }
    }
}

#[derive(Clone, Copy)]
pub struct ActionButtonStyle {
    theme: AppTheme,
    accent_color: Color,
    is_primary: bool,
}

impl button::StyleSheet for ActionButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let (bg_color, text_color) = if self.is_primary {
            (
                self.accent_color,
                Color::WHITE,
            )
        } else {
            match self.theme {
                AppTheme::Light => (
                    Color::from_rgba(0.0, 0.0, 0.0, 0.05),
                    LIGHT_TEXT_PRIMARY,
                ),
                AppTheme::Dark | AppTheme::System => (
                    Color::from_rgba(1.0, 1.0, 1.0, 0.05),
                    DARK_TEXT_PRIMARY,
                ),
            }
        };

        button::Appearance {
            background: Some(Background::Color(bg_color)),
            text_color,
            border: iced::Border {
                radius: (BORDER_RADIUS - 3.0).into(),
                width: if self.is_primary { 0.0 } else { 1.0 },
                color: match self.theme {
                    AppTheme::Light => LIGHT_BORDER,
                    AppTheme::Dark | AppTheme::System => DARK_BORDER,
                },
            },
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 3.0,
            },
            shadow_offset: iced::Vector::default(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        button::Appearance {
            background: Some(Background::Color(Color {
                a: if self.is_primary { 0.9 } else { 0.08 },
                ..match active.background.unwrap() {
                    Background::Color(c) => c,
                    _ => self.accent_color,
                }
            })),
            shadow: iced::Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                offset: iced::Vector::new(0.0, 3.0),
                blur_radius: 5.0,
            },
            ..active
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

pub fn view(state: &State) -> Element<AppMessage> {
    let theme = state.theme;
    let accent_color = state.accent_color;
    let text_color = get_text_color(theme);
    let text_secondary_color = get_text_secondary_color(theme);

    // App title with logo
    let title = container(
        row![
            text("🔍").size(26),
            text("Orion").size(26).style(accent_color),
            text(" Settings").size(26).style(text_color),
        ]
        .spacing(6)
        .align_items(alignment::Alignment::Center)
    )
    .padding([8, 10])
    .width(Length::Fill);

    // Sidebar with navigation tabs
    let tab_button = |label: &str, tab: Tab, icon: &str| {
        let is_selected = state.active_tab == tab;

        // Create a container for the icon with background when selected
        let icon_container = container(
            text(icon)
                .size(20)
                .style(if is_selected {
                    accent_color
                } else {
                    text_secondary_color
                })
        )
        .width(Length::Fixed(32.0))
        .height(Length::Fixed(32.0))
        .center_x()
        .center_y()
        .style(if is_selected {
            theme::Container::Custom(Box::new(IconContainerStyle {
                theme,
                accent_color,
                is_selected,
            }))
        } else {
            theme::Container::Transparent
        });

        let btn_label = text(label)
            .size(15)
            .horizontal_alignment(alignment::Horizontal::Left)
            .style(if is_selected {
                accent_color
            } else {
                text_secondary_color
            });

        button(
            row![
                icon_container,
                Space::with_width(Length::Fixed(10.0)),
                btn_label,
            ]
            .spacing(5)
            .align_items(alignment::Alignment::Center)
            .width(Length::Fill),
        )
        .padding([12, 15])
        .width(Length::Fill)
        .style(theme::Button::Custom(Box::new(TabButtonStyle {
            theme,
            accent_color,
            is_selected,
        })))
        .on_press(AppMessage::TabSelected(tab))
    };

    let sidebar = column![
        title,
        vertical_space().height(Length::from(25)),
        tab_button("General", Tab::General, "⚙"),
        tab_button("Hotkeys", Tab::Hotkeys, "⌨"),
        tab_button("Appearance", Tab::Appearance, "🎨"),
        tab_button("Advanced", Tab::Advanced, "⚒"),
        vertical_space().height(Length::Fill),
        row![
            text(format!("v{}", env!("CARGO_PKG_VERSION")))
                .size(12)
                .style(text_secondary_color),
            horizontal_space().width(Length::Fill),
            text("© 2023")
                .size(12)
                .style(text_secondary_color)
        ]
        .width(Length::Fill),
        vertical_space().height(Length::from(10)),
    ]
    .padding([22, 22, 15, 22])
    .spacing(8)
    .width(Length::Fixed(220.0))
    .height(Length::Fill);

    let sidebar_container = container(sidebar)
        .width(Length::Fixed(220.0))
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
        .padding(30)
        .style(theme::Container::Custom(Box::new(
            ContentContainerStyle { theme },
        )));

    // Main layout
    let layout = row![sidebar_container, content_container]
        .width(Length::Fill)
        .height(Length::Fill);

    container(layout)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::Container::Custom(Box::new(AppContainerStyle {
            theme,
        })))
        .into()
}

// --- Helper Widgets ---

fn section_title(title: &str, theme: AppTheme) -> Element<'static, AppMessage> {
    let text_color = get_text_color(theme);
    row![text(title).size(20).style(text_color),]
        .padding([0, 0, 15, 0]) // Reduced top padding
        .width(Length::Fill)
        .into()
}

fn setting_row<'a>(
    label: &str,
    component: Element<'a, AppMessage>,
    theme: AppTheme,
) -> Element<'a, AppMessage> {
    let text_color = get_text_color(theme);
    row![
        text(label).size(14).style(text_color),
        horizontal_space().width(Length::Fill),
        component
    ]
    .padding([12, 15, 12, 15]) // More balanced padding
    .spacing(15)
    .width(Length::Fill)
    .align_items(alignment::Alignment::Center) // Center items vertically
    .into()
}

fn card_container<'a>(
    content: Element<'a, AppMessage>,
    theme: AppTheme,
) -> Element<'a, AppMessage> {
    container(content)
        .width(Length::Fill)
        .padding(10)
        .style(theme::Container::Custom(Box::new(CardContainerStyle { theme })))
        .into()
}

fn section<'a>(
    title: &str, 
    content: impl Into<Element<'a, AppMessage>>, 
    theme: AppTheme
) -> Element<'a, AppMessage> {
    column![
        section_title(title, theme),
        card_container(content.into(), theme),
    ]
    .spacing(10)
    .width(Length::Fill)
    .into()
}

// --- Tab Implementations ---

fn general_tab(state: &State) -> Element<AppMessage> {
    let theme = state.theme;
    let voice_toggle = setting_row(
        "Enable voice",
        checkbox("", state.voice_enabled)
            .on_toggle(AppMessage::ToggleVoice)
            .into(),
        theme,
    );

    // Profile related UI
    let profile_selector = setting_row(
        "Current Profile",
        pick_list(
            state.profiles.clone(),
            Some(state.current_profile.clone()),
            AppMessage::SelectProfile
        )
        .width(Length::Fixed(200.0))
        .into(),
        theme,
    );

    let new_profile_row = setting_row(
        "Add new profile",
        row![
            text_input("New profile name", &state.new_profile_name)
                .on_input(AppMessage::UpdateNewProfileName)
                .padding(10)
                .width(Length::Fixed(200.0)),
            button(text("Add").size(14))
                .on_press(AppMessage::AddProfile)
                .padding([8, 15])
                .style(theme::Button::Custom(Box::new(ActionButtonStyle {
                    theme,
                    accent_color: state.accent_color,
                    is_primary: false,
                })))
        ]
        .spacing(15)
        .into(),
        theme,
    );

    // Profile list with delete buttons
    let profiles = state.profiles.iter().map(|profile| {
        setting_row(
            profile,
            if profile != "Default" {
                button(text("Delete").size(14))
                    .on_press(AppMessage::DeleteProfile(profile.clone()))
                    .padding([7, 12])
                    .style(theme::Button::Custom(Box::new(ActionButtonStyle {
                        theme,
                        accent_color: Color::from_rgb(0.9, 0.3, 0.3), // Red for delete
                        is_primary: false,
                    })))
                    .into()
            } else {
                // Don't allow deleting the Default profile
                Space::with_width(Length::Shrink).into()
            },
            theme,
        )
    }).collect::<Vec<_>>();

    let profiles_list: Element<'_, AppMessage> = if !profiles.is_empty() {
        container(
            scrollable(
                column(profiles)
                    .spacing(0) // No spacing as rows have padding
                    .width(Length::Fill)
            )
            .height(Length::Fixed(200.0))
            .width(Length::Fill)
        )
        .style(theme::Container::Custom(Box::new(CardContainerStyle { theme })))
        .width(Length::Fill)
        .into()
    } else {
        container(
            text("No profiles available")
                .style(theme::Text::Color(get_text_secondary_color(theme)))
                .width(Length::Fill)
                .horizontal_alignment(alignment::Horizontal::Center)
        )
        .padding(20)
        .style(theme::Container::Custom(Box::new(CardContainerStyle { theme })))
        .width(Length::Fill)
        .into()
    };

    column![
        section("General Settings", column![voice_toggle], theme),
        vertical_space().height(Length::Fixed(25.0)),
        section_title("Profile Management", theme),
        card_container(
            column![
                profile_selector,
                new_profile_row,
            ].into(),
            theme
        ),
        vertical_space().height(Length::Fixed(20.0)),
        profiles_list,
    ]
    .spacing(10)
    .width(Length::Fill)
    .into()
}

fn hotkeys_tab(state: &State) -> Element<AppMessage> {
    let theme = state.theme;
    let hotkey_edit = setting_row(
        "Activation shortcut",
        text_input("Enter hotkey", &state.hotkey)
            .padding(10)
            .width(Length::Fixed(200.0))
            .on_input(AppMessage::UpdateHotkey)
            .into(),
        theme,
    );

    column![
        section("Keyboard Shortcuts", column![hotkey_edit], theme),
    ]
    .spacing(10)
    .width(Length::Fill)
    .into()
}

fn appearance_tab(state: &State) -> Element<AppMessage> {
    let theme = state.theme;
    let accent_color = state.accent_color;

    let theme_selector = setting_row(
        "Theme",
        pick_list(
            vec![AppTheme::Light, AppTheme::Dark, AppTheme::System],
            Some(theme),
            AppMessage::SetTheme
        )
        .width(Length::Fixed(200.0))
        .into(),
        theme,
    );

    let color_button = |color: Color, current_accent: Color| -> Element<AppMessage> {
        let is_selected = color == current_accent;
        let content = if is_selected {
            container(
                text("✓")
                    .size(18)
                    .style(theme::Text::Color(Color::WHITE))
            )
            .center_x()
            .center_y()
            .width(Length::Fixed(40.0))
            .height(Length::Fixed(40.0))
        } else {
            container(Space::with_width(Length::Fixed(40.0)))
                .height(Length::Fixed(40.0))
        };
        
        button(content)
            .style(theme::Button::Custom(Box::new(ColorButtonStyle {
                color,
                theme,
            })))
            .on_press(AppMessage::SetAccentColor(color))
            .into()
    };

    let accent_colors = vec![
        Color::from_rgb(0.35, 0.56, 0.98), // Default Blue
        Color::from_rgb(0.9, 0.3, 0.3),    // Red
        Color::from_rgb(0.3, 0.8, 0.5),    // Green
        Color::from_rgb(0.8, 0.5, 0.9),    // Purple
        Color::from_rgb(0.95, 0.6, 0.1),   // Orange
        Color::from_rgb(0.5, 0.6, 0.7),    // Slate
    ];

    let accent_color_selector = setting_row(
        "Accent color",
        row(accent_colors
            .into_iter()
            .map(|color| color_button(color, accent_color))
            .collect::<Vec<_>>())
        .spacing(15)
        .into(),
        theme,
    );

    column![
        section(
            "Appearance", 
            column![
                theme_selector,
                accent_color_selector,
            ],
            theme
        ),
    ]
    .spacing(10)
    .width(Length::Fill)
    .into()
}

fn advanced_tab(state: &State) -> Element<AppMessage> {
    let theme = state.theme;
    let accent_color = state.accent_color;

    let sensitivity_slider = setting_row(
        "Voice Sensitivity",
        column![
            slider(0.0..=1.0, state.sensitivity, AppMessage::AdjustSensitivity)
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

    let action_buttons = row![
        button(text("Reset to Defaults").size(14))
            .on_press(AppMessage::ResetSettings)
            .padding([10, 15])
            .style(theme::Button::Custom(Box::new(ActionButtonStyle {
                theme,
                accent_color,
                is_primary: false,
            }))),
        button(text("Save Changes").size(14))
            .on_press(AppMessage::SaveSettings)
            .padding([10, 15])
            .style(theme::Button::Custom(Box::new(ActionButtonStyle {
                theme,
                accent_color,
                is_primary: true,
            })))
    ]
    .spacing(15)
    .width(Length::Fill);

    column![
        section(
            "Advanced Settings",
            column![sensitivity_slider],
            theme
        ),
        vertical_space().height(Length::Fixed(15.0)),
        container(
            column![
                text("Warning: These settings are for advanced users only.")
                    .size(12)
                    .style(Color::from_rgb(0.9, 0.6, 0.2)),
                vertical_space().height(Length::Fixed(15.0)),
                action_buttons,
            ]
            .spacing(5)
            .padding(15)
        )
        .style(theme::Container::Custom(Box::new(CardContainerStyle { theme })))
        .width(Length::Fill),
    ]
    .spacing(15)
    .width(Length::Fill)
    .into()
}

// TabUI struct to handle UI rendering
pub struct TabUI {}

impl TabUI {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view<'a>(&self, state: &'a State) -> Element<'a, AppMessage> {
        view(state)
    }
}

// Add a new style for icon containers
#[derive(Clone, Copy)]
pub struct IconContainerStyle {
    theme: AppTheme,
    accent_color: Color,
    is_selected: bool,
}

impl container::StyleSheet for IconContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color {
                a: 0.15,
                ..self.accent_color
            })),
            text_color: None,
            border: iced::Border {
                radius: 8.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
            shadow: iced::Shadow::default(),
        }
    }
}
