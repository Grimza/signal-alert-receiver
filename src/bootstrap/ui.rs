use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tray_icon::{
    Icon, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuItem},
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

pub struct App {
    tray: Option<tray_icon::TrayIcon>,
    activation: MenuItem,
    quit_item: MenuItem,
    is_active: Arc<AtomicBool>,
}

impl App {
    pub fn new(is_active: Arc<AtomicBool>) -> Self {
        Self {
            tray: None,
            activation: MenuItem::with_id("power", "Activate", true, None),
            quit_item: MenuItem::with_id("exit", "Exit", true, None),
            is_active,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        let logo_bytes = include_bytes!("../assets/icons/logo_icon_32.png");

        let icon = load_icon(logo_bytes);

        let menu = Menu::new();
        menu.append(&self.activation).unwrap();
        menu.append(&self.quit_item).unwrap();

        let tray = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_icon(icon)
            .with_tooltip("Signal Alert Receiver")
            .build()
            .unwrap();

        let menu_channel = MenuEvent::receiver();

        // let activation = self.activation.clone();
        let is_active = self.is_active.clone();
        
        std::thread::spawn(move || {
            loop {
                if let Ok(event) = menu_channel.try_recv() {
                    if event.id == "power" {
                        let current = is_active.load(Ordering::Relaxed);
                        let new_state = !current;
                        is_active.store(new_state, Ordering::Relaxed);
                    } else if event.id == "exit" {
                        std::process::exit(0);
                    }
                }
            }
        });

        self.tray = Some(tray);
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _id: WindowId, _event: WindowEvent) {}
}

fn load_icon(bytes: &[u8]) -> Icon {
    let img = image::load_from_memory(bytes).unwrap().into_rgba8();
    let (width, height) = img.dimensions();
    Icon::from_rgba(img.into_raw(), width, height).unwrap()
}

pub fn run_tray(is_active: Arc<AtomicBool>) {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new(is_active);
    event_loop.run_app(&mut app).unwrap();
}
