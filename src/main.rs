#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::{LogicalPosition, WindowPosition, Timer, TimerMode};
use std::cell::{Cell, RefCell};
use std::rc::Rc;

mod constants;
mod panel;
mod webview;
mod screen_info;

use constants::*;
use panel::*;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let (scale_factor, screen_width, screen_height) = screen_info::get_primary_monitor_info();

    let logical_screen_width = screen_width as f32 / scale_factor as f32;
    let logical_screen_height = screen_height as f32 / scale_factor as f32;

    let panel_height = logical_screen_height - MENU_HEIGHT - TASKBAR_HEIGHT;

    let panel_window_width = (logical_screen_width * PANEL_WIDTH_PERCENT)
        .max(PANEL_WIDTH_MIN)
        .min(PANEL_WIDTH_MAX);

    let shift_y = DEFAULT_SHIFT_Y;

    let main_window = MainWindow::new()?;
    main_window.set_window_width(logical_screen_width);
    main_window.set_menu_height(MENU_HEIGHT);
    main_window.set_shift_y(shift_y);

    // WebViewManager только для Panel23
    let webview_manager_23 = Rc::new(RefCell::new(webview::WebViewManager::new()));

    let configs = generate_panel_configs(
        panel_window_width,
        logical_screen_width,
        panel_height,
        shift_y
    );

    let panels = [
        create_side_panel(configs[0], 1)?,
        create_side_panel(configs[1], 2)?,
        create_side_panel(configs[2], 3)?,
        create_side_panel(configs[3], 4)?,
        create_side_panel(configs[4], 5)?,
        create_side_panel(configs[5], 8)?,
        create_side_panel(configs[6], 7)?,
        create_side_panel(configs[7], 6)?,
    ];

    let double_panels = [
        create_double_panel(configs[8], 11)?,
        create_double_panel(configs[9], 12)?,
        create_double_panel(configs[10], 13)?,
        create_double_panel(configs[11], 14)?,
    ];

    let panel_23 = create_panel_23(configs[10], 23)?;

    // ИСПРАВЛЕНИЕ: Показываем главное меню СРАЗУ (как в старой версии)
    main_window.window().show()?;
    main_window.window().set_position(WindowPosition::Logical(
        LogicalPosition::new(0.0, shift_y)
    ));

    for panel in &panels {
        panel.window().show()?;
    }
    for panel in &double_panels {
        panel.window().show()?;
    }
    panel_23.window().show()?;

    // Состояния панелей (нужны для триггера)
    let panel_states: [Rc<Cell<bool>>; 8] = [
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
    ];

    // ============ ТРИГГЕРНОЕ ОКНО У ЛЕВОЙ ГРАНИЦЫ ============
    let trigger_window = TriggerWindow::new()?;
    trigger_window.set_trigger_height(logical_screen_height);
    trigger_window.window().set_position(WindowPosition::Logical(
        LogicalPosition::new(0.0, 0.0)
    ));
    trigger_window.window().show()?;


    // Таймер для триггера (проверка каждые 100ms)
    let trigger_timer = Rc::new(Timer::default());
    let trigger_weak = trigger_window.as_weak();
    let panel_1_weak = panels[0].as_weak();
    let panel_1_state = panel_states[0].clone();
    let config_0 = configs[0];
    let timer_clone = trigger_timer.clone();

    timer_clone.start(TimerMode::Repeated, std::time::Duration::from_millis(100), move || {
        if let Some(trigger) = trigger_weak.upgrade() {
            static mut HOVER_START: Option<std::time::Instant> = None;

            if trigger.get_cursor_in_zone() {
                unsafe {
                    if HOVER_START.is_none() {
                        HOVER_START = Some(std::time::Instant::now());
                    } else if HOVER_START.unwrap().elapsed() >= std::time::Duration::from_secs(2) {
                        println!("🔥 Триггер сработал! Открываю Панель 1");

                        // Открываем Панель 1
                        if let Some(panel) = panel_1_weak.upgrade() {
                            panel_1_state.set(true);
                            panel.window().set_position(WindowPosition::Logical(
                                LogicalPosition::new(config_0.window_x_open, config_0.y)
                            ));
                            panel.set_target_x(config_0.rect_x_open);
                        }

                        HOVER_START = None; // Сбрасываем
                    }
                }
            } else {
                unsafe { HOVER_START = None; } // Курсор ушел - сбрасываем
            }
        }
    });
    // ============ КОНЕЦ ДОБАВЛЕНИЯ ============


    main_window.set_status_text(format!("Scale: {:.2} | 12 panels | OPTIMIZED", scale_factor).into());

    // Предрасчитанные bounds для WebView23
    let webview_bounds_panel23 = webview::WebViewBounds::new(configs[10].width, configs[10].height, 50.0);

    // WebView state только для Panel23
    let webview_state_23: Rc<Cell<Option<i32>>> = Rc::new(Cell::new(None));

    // Состояние Panel23: открыта/закрыта
    let panel_23_open: Rc<Cell<bool>> = Rc::new(Cell::new(false));

    // Последняя нажатая кнопка для Panel23 ("W2", "V2" или "")
    let last_button_23: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));

    // Хранилище для активного таймера Panel23
    let panel_23_timer: Rc<RefCell<Option<Rc<Timer>>>> = Rc::new(RefCell::new(None));

    // Универсальная фабрика обработчиков для Panel23 WebView (внутри main для правильных типов)
    let create_panel23_handler = |url: &'static str, button_name: &'static str| {
        let webview_manager = webview_manager_23.clone();
        let panel_weak = panel_23.as_weak();
        let panel_open = panel_23_open.clone();
        let last_button = last_button_23.clone();
        let panel_timer = panel_23_timer.clone();
        let config = configs[10];

        move || {
            let last = last_button.borrow().clone();

            if last == button_name && panel_open.get() {
                // Закрываем Panel23
                let manager = webview_manager.borrow();
                if manager.is_active() {
                    let _ = manager.load_url("about:blank");
                }

                if let Some(panel) = panel_weak.upgrade() {
                    panel.window().set_position(WindowPosition::Logical(
                        LogicalPosition::new(config.window_x_closed, config.y)
                    ));
                    panel_open.set(false);
                    last_button.replace(String::new());
                }
            } else {
                // Загружаем URL
                let manager = webview_manager.borrow();
                if manager.is_active() {
                    let _ = manager.load_url(url);
                }

                // Если закрыта - создаём таймер и открываем через 300ms
                if !panel_open.get() {
                    let timer = Rc::new(Timer::default());
                    *panel_timer.borrow_mut() = Some(timer.clone());

                    let panel_weak_clone = panel_weak.clone();
                    let panel_open_clone = panel_open.clone();

                    timer.start(TimerMode::SingleShot, std::time::Duration::from_millis(300), move || {
                        if let Some(panel) = panel_weak_clone.upgrade() {
                            panel.window().set_position(WindowPosition::Logical(
                                LogicalPosition::new(config.window_x_open, config.y)
                            ));
                            panel_open_clone.set(true);
                        }
                    });
                }

                last_button.replace(button_name.to_string());
            }
        }
    };

    // Создаём WebView23 через 100ms после старта
    let _precreate_timer = {
        let panel_weak = panel_23.as_weak();
        let webview_manager = webview_manager_23.clone();
        let webview_state = webview_state_23.clone();
        let bounds = webview_bounds_panel23;

        let timer = Rc::new(Timer::default());
        timer.start(TimerMode::SingleShot, std::time::Duration::from_millis(300), move || {
            if let Some(panel) = panel_weak.upgrade() {
                let handle = panel.window().window_handle();

                match webview::create_webview_with_bounds(
                    1,
                    "about:blank".to_string(),
                    bounds,
                    &handle
                ) {
                    Ok(webview_handle) => {
                        webview_manager.borrow_mut().set(webview_handle);
                        webview_state.set(Some(1));
                        // println!("WebView23 pre-created (ready for instant use)");
                    }
                    Err(e) => eprintln!("Failed to pre-create WebView23: {}", e),
                }
            }
        });
        timer
    };

    let panel_weaks: [_; 8] = [
        panels[0].as_weak(),
        panels[1].as_weak(),
        panels[2].as_weak(),
        panels[3].as_weak(),
        panels[4].as_weak(),
        panels[5].as_weak(),
        panels[6].as_weak(),
        panels[7].as_weak(),
    ];

    let double_weaks: [_; 4] = [
        double_panels[0].as_weak(),
        double_panels[1].as_weak(),
        double_panels[2].as_weak(),
        double_panels[3].as_weak(),
    ];

    let double_states: [Rc<Cell<bool>>; 4] = [
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
        Rc::new(Cell::new(false)),
    ];

    // Оптимизированный макрос (ленивые таймеры, без +10ms)
    macro_rules! create_optimized_toggle_handler {
        ($panel_weak:expr, $panel_state:expr, $config_idx:expr) => {{
            let panel_weak = $panel_weak.clone();
            let panel_state = $panel_state.clone();
            let config = configs[$config_idx];

            move || {
                let opening = !panel_state.get();
                panel_state.set(opening);

                if let Some(panel) = panel_weak.upgrade() {
                    if opening {
                        panel.window().set_position(WindowPosition::Logical(
                            LogicalPosition::new(config.window_x_open, config.y)
                        ));
                        panel.set_target_x(config.rect_x_open);
                    } else {
                        panel.set_target_x(config.rect_x_closed);

                        let timer = Timer::default();
                        let panel_weak_clone = panel.as_weak();

                        timer.start(
                            TimerMode::SingleShot,
                            std::time::Duration::from_millis(ANIMATION_DURATION_MS),
                            move || {
                                if let Some(p) = panel_weak_clone.upgrade() {
                                    p.window().set_position(WindowPosition::Logical(
                                        LogicalPosition::new(config.window_x_closed, config.y)
                                    ));
                                }
                            }
                        );
                    }
                }
            }
        }};
    }

    // Обработчики для панелей 1-8
    main_window.on_toggle_panel_1(create_optimized_toggle_handler!(panel_weaks[0], panel_states[0], 0));

    // Обработчик кнопки закрытия для Панели 1
    panels[0].on_close_clicked({
        let panel_weak = panels[0].as_weak();
        let panel_state = panel_states[0].clone();
        let config = configs[0];

        move || {
            if let Some(panel) = panel_weak.upgrade() {
                panel_state.set(false);
                panel.set_target_x(config.rect_x_closed);

                let timer = Timer::default();
                let panel_weak_clone = panel.as_weak();

                timer.start(
                    TimerMode::SingleShot,
                    std::time::Duration::from_millis(ANIMATION_DURATION_MS),
                    move || {
                        if let Some(p) = panel_weak_clone.upgrade() {
                            p.window().set_position(WindowPosition::Logical(
                                LogicalPosition::new(config.window_x_closed, config.y)
                            ));
                        }
                    }
                );
            }
        }
    });

    main_window.on_toggle_panel_2(create_optimized_toggle_handler!(panel_weaks[1], panel_states[1], 1));
    main_window.on_toggle_panel_3(create_optimized_toggle_handler!(panel_weaks[2], panel_states[2], 2));
    main_window.on_toggle_panel_4(create_optimized_toggle_handler!(panel_weaks[3], panel_states[3], 3));
    main_window.on_toggle_panel_5(create_optimized_toggle_handler!(panel_weaks[4], panel_states[4], 4));
    main_window.on_toggle_panel_6(create_optimized_toggle_handler!(panel_weaks[5], panel_states[5], 5));
    main_window.on_toggle_panel_7(create_optimized_toggle_handler!(panel_weaks[6], panel_states[6], 6));
    main_window.on_toggle_panel_8(create_optimized_toggle_handler!(panel_weaks[7], panel_states[7], 7));

    // Обработчики для двойных панелей
    main_window.on_toggle_panel_11({
        let panel = double_weaks[0].clone();
        let state = double_states[0].clone();
        let config = configs[8];
        move || {
            let opening = !state.get();
            state.set(opening);
            if let Some(p) = panel.upgrade() {
                let x = if opening { config.window_x_open } else { config.window_x_closed };
                p.window().set_position(WindowPosition::Logical(LogicalPosition::new(x, config.y)));
            }
        }
    });

    main_window.on_toggle_panel_12({
        let panel = double_weaks[1].clone();
        let state = double_states[1].clone();
        let config = configs[9];
        move || {
            let opening = !state.get();
            state.set(opening);
            if let Some(p) = panel.upgrade() {
                let x = if opening { config.window_x_open } else { config.window_x_closed };
                p.window().set_position(WindowPosition::Logical(LogicalPosition::new(x, config.y)));
            }
        }
    });

    main_window.on_toggle_panel_13({
        let panel = double_weaks[2].clone();
        let state = double_states[2].clone();
        let config = configs[10];
        move || {
            let opening = !state.get();
            state.set(opening);
            if let Some(p) = panel.upgrade() {
                let x = if opening { config.window_x_open } else { config.window_x_closed };
                p.window().set_position(WindowPosition::Logical(LogicalPosition::new(x, config.y)));
            }
        }
    });

    main_window.on_toggle_panel_14({
        let panel = double_weaks[3].clone();
        let state = double_states[3].clone();
        let config = configs[11];
        move || {
            let opening = !state.get();
            state.set(opening);
            if let Some(p) = panel.upgrade() {
                let x = if opening { config.window_x_open } else { config.window_x_closed };
                p.window().set_position(WindowPosition::Logical(LogicalPosition::new(x, config.y)));
            }
        }
    });

    // ОПТИМИЗАЦИЯ: Обработчики W2 и V2 через единую фабрику-замыкание
    main_window.on_toggle_webview_23(create_panel23_handler(
        "https://www.rust-lang.org",
        "W2"
    ));

    main_window.on_change_webview_url_23(create_panel23_handler(
        "https://www.google.com/maps",
        "V2"
    ));

    main_window.run()
}
