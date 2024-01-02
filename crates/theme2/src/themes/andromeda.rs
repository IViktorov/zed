// This file was generated by the `theme_importer`.
// Be careful when modifying it by hand.

use gpui::rgba;

#[allow(unused)]
use crate::{
    Appearance, PlayerColor, PlayerColors, StatusColorsRefinement, ThemeColorsRefinement,
    UserFontStyle, UserFontWeight, UserHighlightStyle, UserSyntaxTheme, UserTheme, UserThemeFamily,
    UserThemeStylesRefinement,
};

pub fn andromeda() -> UserThemeFamily {
    UserThemeFamily {
        name: "Andromeda".into(),
        author: "Zed Industries".into(),
        themes: vec![UserTheme {
            name: "Andromeda".into(),
            appearance: Appearance::Dark,
            styles: UserThemeStylesRefinement {
                colors: ThemeColorsRefinement {
                    border: Some(rgba(0x2b2f39ff).into()),
                    border_variant: Some(rgba(0x2b2f39ff).into()),
                    border_focused: Some(rgba(0x183a34ff).into()),
                    border_selected: Some(rgba(0x183a34ff).into()),
                    border_transparent: Some(rgba(0x00000000).into()),
                    border_disabled: Some(rgba(0x292d37ff).into()),
                    elevated_surface_background: Some(rgba(0x21242bff).into()),
                    surface_background: Some(rgba(0x21242bff).into()),
                    background: Some(rgba(0x262a33ff).into()),
                    panel_background: Some(rgba(0x21242bff).into()),
                    element_background: Some(rgba(0x21242bff).into()),
                    element_hover: Some(rgba(0x252931ff).into()),
                    element_active: Some(rgba(0x2a2f39ff).into()),
                    element_selected: Some(rgba(0x2a2f39ff).into()),
                    element_disabled: Some(rgba(0x21242bff).into()),
                    drop_target_background: Some(rgba(0xaca8ae80).into()),
                    ghost_element_background: Some(rgba(0x00000000).into()),
                    ghost_element_hover: Some(rgba(0x252931ff).into()),
                    ghost_element_active: Some(rgba(0x2a2f39ff).into()),
                    ghost_element_selected: Some(rgba(0x2a2f39ff).into()),
                    ghost_element_disabled: Some(rgba(0x21242bff).into()),
                    text: Some(rgba(0xf7f7f8ff).into()),
                    text_muted: Some(rgba(0xaca8aeff).into()),
                    text_placeholder: Some(rgba(0x6b6b73ff).into()),
                    text_disabled: Some(rgba(0x6b6b73ff).into()),
                    text_accent: Some(rgba(0x11a793ff).into()),
                    icon: Some(rgba(0xf7f7f8ff).into()),
                    icon_muted: Some(rgba(0xaca8aeff).into()),
                    icon_disabled: Some(rgba(0x6b6b73ff).into()),
                    icon_placeholder: Some(rgba(0xaca8aeff).into()),
                    icon_accent: Some(rgba(0x11a793ff).into()),
                    status_bar_background: Some(rgba(0x262a33ff).into()),
                    title_bar_background: Some(rgba(0x262a33ff).into()),
                    toolbar_background: Some(rgba(0x1e2025ff).into()),
                    tab_bar_background: Some(rgba(0x21242bff).into()),
                    tab_inactive_background: Some(rgba(0x21242bff).into()),
                    tab_active_background: Some(rgba(0x1e2025ff).into()),
                    scrollbar_thumb_background: Some(rgba(0xf7f7f84c).into()),
                    scrollbar_thumb_hover_background: Some(rgba(0x252931ff).into()),
                    scrollbar_thumb_border: Some(rgba(0x252931ff).into()),
                    scrollbar_track_background: Some(rgba(0x1e2025ff).into()),
                    scrollbar_track_border: Some(rgba(0x21232aff).into()),
                    editor_foreground: Some(rgba(0xf7f7f8ff).into()),
                    editor_background: Some(rgba(0x1e2025ff).into()),
                    editor_gutter_background: Some(rgba(0x1e2025ff).into()),
                    editor_subheader_background: Some(rgba(0x21242bff).into()),
                    editor_active_line_background: Some(rgba(0x21242bbf).into()),
                    editor_highlighted_line_background: Some(rgba(0x21242bff).into()),
                    editor_line_number: Some(rgba(0xf7f7f859).into()),
                    editor_active_line_number: Some(rgba(0xf7f7f8ff).into()),
                    editor_invisible: Some(rgba(0xaca8aeff).into()),
                    editor_wrap_guide: Some(rgba(0xf7f7f80d).into()),
                    editor_active_wrap_guide: Some(rgba(0xf7f7f81a).into()),
                    editor_document_highlight_read_background: Some(rgba(0x11a7931a).into()),
                    editor_document_highlight_write_background: Some(rgba(0x64646d66).into()),
                    terminal_background: Some(rgba(0x1e2025ff).into()),
                    terminal_ansi_bright_black: Some(rgba(0x40434cff).into()),
                    terminal_ansi_bright_red: Some(rgba(0x8e103aff).into()),
                    terminal_ansi_bright_green: Some(rgba(0x457c38ff).into()),
                    terminal_ansi_bright_yellow: Some(rgba(0x958435ff).into()),
                    terminal_ansi_bright_blue: Some(rgba(0x1b5148ff).into()),
                    terminal_ansi_bright_magenta: Some(rgba(0x682781ff).into()),
                    terminal_ansi_bright_cyan: Some(rgba(0x018169ff).into()),
                    terminal_ansi_bright_white: Some(rgba(0xf7f7f8ff).into()),
                    terminal_ansi_black: Some(rgba(0x1e2025ff).into()),
                    terminal_ansi_red: Some(rgba(0xf82872ff).into()),
                    terminal_ansi_green: Some(rgba(0x96df72ff).into()),
                    terminal_ansi_yellow: Some(rgba(0xfee56dff).into()),
                    terminal_ansi_blue: Some(rgba(0x11a793ff).into()),
                    terminal_ansi_magenta: Some(rgba(0xc74decff).into()),
                    terminal_ansi_cyan: Some(rgba(0x09e7c6ff).into()),
                    terminal_ansi_white: Some(rgba(0xf7f7f8ff).into()),
                    link_text_hover: Some(rgba(0x11a793ff).into()),
                    ..Default::default()
                },
                status: StatusColorsRefinement {
                    conflict: Some(rgba(0xfee56dff).into()),
                    conflict_background: Some(rgba(0x5c5015ff).into()),
                    conflict_border: Some(rgba(0x796b26ff).into()),
                    created: Some(rgba(0x96df72ff).into()),
                    created_background: Some(rgba(0x194618ff).into()),
                    created_border: Some(rgba(0x306129ff).into()),
                    deleted: Some(rgba(0xf82872ff).into()),
                    deleted_background: Some(rgba(0x55051bff).into()),
                    deleted_border: Some(rgba(0x720a2bff).into()),
                    error: Some(rgba(0xf82872ff).into()),
                    error_background: Some(rgba(0x55051bff).into()),
                    error_border: Some(rgba(0x720a2bff).into()),
                    hidden: Some(rgba(0x6b6b73ff).into()),
                    hidden_background: Some(rgba(0x262a33ff).into()),
                    hidden_border: Some(rgba(0x292d37ff).into()),
                    hint: Some(rgba(0x11a793ff).into()),
                    hint_background: Some(rgba(0x122420ff).into()),
                    hint_border: Some(rgba(0x183a34ff).into()),
                    ignored: Some(rgba(0xaca8aeff).into()),
                    ignored_background: Some(rgba(0x262a33ff).into()),
                    ignored_border: Some(rgba(0x2b2f39ff).into()),
                    info: Some(rgba(0x11a793ff).into()),
                    info_background: Some(rgba(0x122420ff).into()),
                    info_border: Some(rgba(0x183a34ff).into()),
                    modified: Some(rgba(0xfee56dff).into()),
                    modified_background: Some(rgba(0x5c5015ff).into()),
                    modified_border: Some(rgba(0x796b26ff).into()),
                    predictive: Some(rgba(0x96df72ff).into()),
                    predictive_background: Some(rgba(0x194618ff).into()),
                    predictive_border: Some(rgba(0x306129ff).into()),
                    renamed: Some(rgba(0x11a793ff).into()),
                    renamed_background: Some(rgba(0x122420ff).into()),
                    renamed_border: Some(rgba(0x183a34ff).into()),
                    success: Some(rgba(0x96df72ff).into()),
                    success_background: Some(rgba(0x194618ff).into()),
                    success_border: Some(rgba(0x306129ff).into()),
                    unreachable: Some(rgba(0xaca8aeff).into()),
                    unreachable_background: Some(rgba(0x262a33ff).into()),
                    unreachable_border: Some(rgba(0x2b2f39ff).into()),
                    warning: Some(rgba(0xfee56dff).into()),
                    warning_background: Some(rgba(0x5c5015ff).into()),
                    warning_border: Some(rgba(0x796b26ff).into()),
                    ..Default::default()
                },
                player: Some(PlayerColors(vec![
                    PlayerColor {
                        cursor: rgba(0x11a793ff).into(),
                        background: rgba(0x11a793ff).into(),
                        selection: rgba(0x11a7933d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0xc74decff).into(),
                        background: rgba(0xc74decff).into(),
                        selection: rgba(0xc74dec3d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0xf29c14ff).into(),
                        background: rgba(0xf29c14ff).into(),
                        selection: rgba(0xf29c143d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0x8a3fa6ff).into(),
                        background: rgba(0x8a3fa6ff).into(),
                        selection: rgba(0x8a3fa63d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0x09e7c6ff).into(),
                        background: rgba(0x09e7c6ff).into(),
                        selection: rgba(0x09e7c63d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0xf82872ff).into(),
                        background: rgba(0xf82872ff).into(),
                        selection: rgba(0xf828723d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0xfee56dff).into(),
                        background: rgba(0xfee56dff).into(),
                        selection: rgba(0xfee56d3d).into(),
                    },
                    PlayerColor {
                        cursor: rgba(0x96df72ff).into(),
                        background: rgba(0x96df72ff).into(),
                        selection: rgba(0x96df723d).into(),
                    },
                ])),
                syntax: Some(UserSyntaxTheme {
                    highlights: vec![
                        (
                            "attribute".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x11a793ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "boolean".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x96df72ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "comment".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xafabb1ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "comment.doc".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xafabb1ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "constant".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x96df72ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "constructor".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x11a793ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "embedded".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf7f7f8ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "emphasis".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x11a793ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "emphasis.strong".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x11a793ff).into()),
                                font_weight: Some(UserFontWeight(700.0)),
                                ..Default::default()
                            },
                        ),
                        (
                            "enum".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf29c14ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "function".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfee56dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "hint".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x618399ff).into()),
                                font_weight: Some(UserFontWeight(700.0)),
                                ..Default::default()
                            },
                        ),
                        (
                            "keyword".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x11a793ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "label".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x11a793ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "link_text".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf29c14ff).into()),
                                font_style: Some(UserFontStyle::Italic),
                                ..Default::default()
                            },
                        ),
                        (
                            "link_uri".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x96df72ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "number".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x96df72ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "operator".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf29c14ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "predictive".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x315f70ff).into()),
                                font_style: Some(UserFontStyle::Italic),
                                ..Default::default()
                            },
                        ),
                        (
                            "preproc".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf7f7f8ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "primary".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf7f7f8ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "property".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x11a793ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xd8d5dbff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.bracket".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xd8d5dbff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.delimiter".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xd8d5dbff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.list_marker".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xd8d5dbff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.special".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xd8d5dbff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf29c14ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.escape".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xafabb1ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.regex".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf29c14ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.special".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf29c14ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "string.special.symbol".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf29c14ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "tag".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x11a793ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "text.literal".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf29c14ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "title".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf7f7f8ff).into()),
                                font_weight: Some(UserFontWeight(700.0)),
                                ..Default::default()
                            },
                        ),
                        (
                            "type".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x09e7c6ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "variable".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf7f7f8ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "variant".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x11a793ff).into()),
                                ..Default::default()
                            },
                        ),
                    ],
                }),
            },
        }],
    }
}
