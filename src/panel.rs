use slint::{LogicalPosition, WindowPosition, Color};
use crate::constants::*;

slint::include_modules!();

#[derive(Clone, Copy, PartialEq)]
pub enum PanelDirection {
    LeftToRight,
    RightToLeft,
}

#[derive(Clone, Copy)]
pub struct PanelConfig {
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: (u8, u8, u8),
    pub opacity: f32,
    pub direction: PanelDirection,

    pub window_x_open: f32,
    pub window_x_closed: f32,
    pub rect_x_open: f32,
    pub rect_x_closed: f32,
}

pub fn create_side_panel(config: PanelConfig, panel_id: i32) -> Result<SidePanel, slint::PlatformError> {
    let panel = SidePanel::new()?;

    panel.set_panel_width(config.width);
    panel.set_panel_height(config.height);
    panel.set_panel_id(panel_id);

    panel.set_panel_color(Color::from_rgb_u8(
        config.color.0,
        config.color.1,
        config.color.2,
    ));
    panel.set_panel_opacity(config.opacity);

    let (shadow_offset, is_right) = match config.direction {
        PanelDirection::LeftToRight => (SHADOW_WIDTH, false),
        PanelDirection::RightToLeft => (-SHADOW_WIDTH, true),
    };

    panel.set_shadow_offset_x(shadow_offset);
    panel.set_shadow_blur(SHADOW_BLUR);
    panel.set_shadow_width_total(SHADOW_BLUR * 2.0);
    panel.set_is_right_panel(is_right);

    panel.set_target_x(config.rect_x_closed);

    panel.window().set_position(WindowPosition::Logical(
        LogicalPosition::new(config.window_x_closed, config.y)
    ));

    Ok(panel)
}

pub fn create_double_panel(config: PanelConfig, _panel_id: i32) -> Result<DoublePanel, slint::PlatformError> {
    let panel = DoublePanel::new()?;

    panel.set_panel_width(config.width);
    panel.set_panel_height(config.height);
    panel.set_shadow_offset_x(SHADOW_WIDTH);
    panel.set_shadow_blur(SHADOW_BLUR);

    panel.window().set_position(WindowPosition::Logical(
        LogicalPosition::new(config.window_x_closed, config.y)
    ));

    Ok(panel)
}

pub fn generate_panel_configs(
    panel_window_width: f32,
    logical_screen_width: f32,
    panel_height: f32,
    shift_y: f32,
) -> [PanelConfig; 12] {
    let window_width_total = panel_window_width + SHADOW_BLUR * 2.0;

    // ИСПРАВЛЕНИЕ: для двойных панелей тень только справа
    let double_width = panel_window_width * 2.0; // 0.4 * ширина_экрана
    let double_window_width = double_width + SHADOW_BLUR; // + тень только справа

    let panel_y = MENU_HEIGHT + shift_y;
    let adjusted_panel_height = panel_height - shift_y;

    let base_config = PanelConfig {
        y: panel_y,
        width: panel_window_width,
        height: adjusted_panel_height,
        color: (245, 244, 237),
        opacity: 1.0,
        direction: PanelDirection::LeftToRight,
        window_x_open: 0.0,
        window_x_closed: 0.0,
        rect_x_open: 0.0,
        rect_x_closed: 0.0,
    };

    [
        // Панели 1-5 (левые) - БЕЗ ИЗМЕНЕНИЙ
        PanelConfig {
            direction: PanelDirection::LeftToRight,
            window_x_open: panel_window_width * 0.0 - SHADOW_BLUR,
            window_x_closed: -window_width_total -100.0,
            rect_x_open: SHADOW_BLUR,
            rect_x_closed: -window_width_total + SHADOW_BLUR,
            ..base_config
        },
        PanelConfig {
            direction: PanelDirection::LeftToRight,
            window_x_open: panel_window_width * 1.0 - SHADOW_BLUR,
            window_x_closed: -window_width_total -100.0,
            rect_x_open: SHADOW_BLUR,
            rect_x_closed: -window_width_total + SHADOW_BLUR,
            ..base_config
        },
        PanelConfig {
            direction: PanelDirection::LeftToRight,
            window_x_open: panel_window_width * 2.0 - SHADOW_BLUR,
            window_x_closed: -window_width_total -100.0,
            rect_x_open: SHADOW_BLUR,
            rect_x_closed: -window_width_total + SHADOW_BLUR,
            ..base_config
        },
        PanelConfig {
            direction: PanelDirection::LeftToRight,
            window_x_open: panel_window_width * 3.0 - SHADOW_BLUR,
            window_x_closed: -window_width_total -100.0,
            rect_x_open: SHADOW_BLUR,
            rect_x_closed: -window_width_total + SHADOW_BLUR,
            ..base_config
        },
        PanelConfig {
            direction: PanelDirection::LeftToRight,
            window_x_open: panel_window_width * 4.0 - SHADOW_BLUR,
            window_x_closed: -window_width_total -100.0,
            rect_x_open: SHADOW_BLUR,
            rect_x_closed: -window_width_total + SHADOW_BLUR,
            ..base_config
        },
        // Панели 8, 7, 6 (правые) - БЕЗ ИЗМЕНЕНИЙ
        PanelConfig {
            direction: PanelDirection::RightToLeft,
            window_x_open: logical_screen_width - panel_window_width - SHADOW_BLUR,
            window_x_closed: logical_screen_width,
            rect_x_open: SHADOW_BLUR,
            rect_x_closed: window_width_total + SHADOW_BLUR,
            ..base_config
        },
        PanelConfig {
            direction: PanelDirection::RightToLeft,
            window_x_open: logical_screen_width - panel_window_width * 2.0 - SHADOW_BLUR,
            window_x_closed: logical_screen_width,
            rect_x_open: SHADOW_BLUR,
            rect_x_closed: window_width_total + SHADOW_BLUR,
            ..base_config
        },
        PanelConfig {
            direction: PanelDirection::RightToLeft,
            window_x_open: logical_screen_width - panel_window_width * 3.0 - SHADOW_BLUR,
            window_x_closed: logical_screen_width,
            rect_x_open: SHADOW_BLUR,
            rect_x_closed: window_width_total + SHADOW_BLUR,
            ..base_config
        },
        // Двойные панели 11, 12, 13, 14 - ИСПРАВЛЕННАЯ ШИРИНА (тень только справа)
        PanelConfig {
            width: double_width, // 0.4 * ширина_экрана
            direction: PanelDirection::LeftToRight,
            window_x_open: double_width * 0.0, // без смещения для тени слева
            window_x_closed: -double_window_width -100.0, // правильная ширина с тенью справа
            rect_x_open: 0.0, // без смещения для тени
            rect_x_closed: -double_window_width,
            ..base_config
        },
        PanelConfig {
            width: double_width,
            direction: PanelDirection::LeftToRight,
            window_x_open: double_width * 0.5,
            window_x_closed: -double_window_width -100.0,
            rect_x_open: 0.0,
            rect_x_closed: -double_window_width,
            ..base_config
        },
        PanelConfig {
            width: double_width,
            direction: PanelDirection::LeftToRight,
            window_x_open: double_width * 1.0,
            window_x_closed: -double_window_width -100.0,
            rect_x_open: 0.0,
            rect_x_closed: -double_window_width,
            ..base_config
        },
        PanelConfig {
            width: double_width,
            direction: PanelDirection::LeftToRight,
            window_x_open: double_width * 1.5,
            window_x_closed: -double_window_width -100.0,
            rect_x_open: 0.0,
            rect_x_closed: -double_window_width,
            ..base_config
        },
    ]
}

pub fn create_panel_23(config: PanelConfig, _panel_id: i32) -> Result<Panel23, slint::PlatformError> {
    let panel = Panel23::new()?;

    panel.set_panel_width(config.width);
    panel.set_panel_height(config.height);
    panel.set_shadow_offset_x(SHADOW_WIDTH);
    panel.set_shadow_blur(SHADOW_BLUR);

    panel.window().set_position(WindowPosition::Logical(
        LogicalPosition::new(config.window_x_closed, config.y)
    ));

    Ok(panel)
}
