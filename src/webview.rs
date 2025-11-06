#![allow(dead_code)]

use std::sync::{Arc, Mutex};
use wry::{WebViewBuilder, WebView, Rect};
use wry::dpi::{LogicalPosition, LogicalSize};
use wry::raw_window_handle::HasWindowHandle;

pub struct WebViewHandle {
    pub id: usize,
    webview: Arc<Mutex<Option<WebView>>>,
}

impl WebViewHandle {
    pub fn close(&self) {
        let _ = self.webview.lock().unwrap().take();
    }

    pub fn load_url(&self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(webview) = self.webview.lock().unwrap().as_ref() {
            webview.load_url(url)?;
            Ok(())
        } else {
            Err("WebView is closed".into())
        }
    }

    // НОВОЕ: Метод для перемещения и изменения размера WebView
    pub fn set_bounds(&self, x: f64, y: f64, width: f64, height: f64) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(webview) = self.webview.lock().unwrap().as_ref() {
            webview.set_bounds(Rect {
                position: LogicalPosition::new(x, y).into(),
                size: LogicalSize::new(width, height).into(),
            })?;
            Ok(())
        } else {
            Err("WebView is closed".into())
        }
    }

    pub fn is_available(&self) -> bool {
        self.webview.lock().unwrap().is_some()
    }
}

// Менеджер для управления одним WebView
pub struct WebViewManager {
    active: Option<WebViewHandle>,
    next_id: usize,
}

impl WebViewManager {
    pub fn new() -> Self {
        Self {
            active: None,
            next_id: 1,
        }
    }

    pub fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn set(&mut self, handle: WebViewHandle) {
        self.active = Some(handle);
    }

    pub fn get(&self) -> Option<&WebViewHandle> {
        self.active.as_ref()
    }

    pub fn load_url(&self, url: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.active
            .as_ref()
            .ok_or("No active WebView")?
            .load_url(url)
    }

    pub fn is_active(&self) -> bool {
        self.active.is_some()
    }
}

// Структура для размеров WebView
#[derive(Clone, Copy, Debug)]
pub struct WebViewBounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl WebViewBounds {
    pub fn new(panel_width: f32, panel_height: f32, margin: f32) -> Self {
        Self {
            x: margin as f64,
            y: margin as f64,
            width: (panel_width - margin * 2.0).max(100.0) as f64,
            height: (panel_height - margin * 2.0).max(100.0) as f64,
        }
    }
}

pub fn create_webview(
    id: usize,
    url: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    parent_window_handle: &impl HasWindowHandle,
) -> Result<WebViewHandle, Box<dyn std::error::Error>> {
    let bounds = Rect {
        position: LogicalPosition::new(x, y).into(),
        size: LogicalSize::new(width, height).into(),
    };

    let webview = WebViewBuilder::new()
        .with_url(&url)
        .with_bounds(bounds)
        .with_devtools(false)
        .build_as_child(parent_window_handle)?;

    Ok(WebViewHandle {
        id,
        webview: Arc::new(Mutex::new(Some(webview))),
    })
}

// Создание с WebViewBounds
pub fn create_webview_with_bounds(
    id: usize,
    url: String,
    bounds: WebViewBounds,
    parent_window_handle: &impl HasWindowHandle,
) -> Result<WebViewHandle, Box<dyn std::error::Error>> {
    let rect = Rect {
        position: LogicalPosition::new(bounds.x, bounds.y).into(),
        size: LogicalSize::new(bounds.width, bounds.height).into(),
    };

    let webview = WebViewBuilder::new()
        .with_url(&url)
        .with_bounds(rect)
        .with_devtools(false)
        .build_as_child(parent_window_handle)?;

    Ok(WebViewHandle {
        id,
        webview: Arc::new(Mutex::new(Some(webview))),
    })
}

// НОВОЕ: Создание WebView за экраном для прогрева
pub fn create_prewarming_webview(
    parent_window_handle: &impl HasWindowHandle,
) -> Result<WebViewHandle, Box<dyn std::error::Error>> {
    // Создаём WebView за экраном с быстрым сайтом
    let webview = WebViewBuilder::new()
        .with_url("https://example.com")
        .with_bounds(Rect {
            position: LogicalPosition::new(-800.0, 0.0).into(),
            size: LogicalSize::new(800.0, 600.0).into(),
        })
        .with_devtools(false)
        .build_as_child(parent_window_handle)?;

    Ok(WebViewHandle {
        id: 0, // ID для прогретого WebView
        webview: Arc::new(Mutex::new(Some(webview))),
    })
}
