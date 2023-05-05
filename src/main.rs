use std::{rc::Rc, sync::RwLock, time::Instant};

use k9::{
    console_command,
    debug_ui::{console::ConsoleCommandInterface, console::DebugUiWindow, ConsoleCommand},
    entity_component::Entity,
    graphics::{component::TexQuadBase, GraphicsComponent},
    system::{FirstCallState, FrameState},
    uuid::Uuid,
    System, SystemCallbacks,
};

fn main() {
    let user_systems = {
        let mut v: Vec<Box<dyn SystemCallbacks>> = Vec::new();
        v.push(Box::new(GameDirector::new()));
        v
    };

    let logger = env_logger::builder().build();

    k9::run(Some(k9::process::CreationArgs {
        max_fps: 120,
        use_vsync: false,
        dimensions: (1600, 900),
        user_systems,
        loggers: vec![Box::new(logger)],
        fullscreen: false,
        ..Default::default()
    }))
    .unwrap()
}

pub struct GameDirector {
    xyz: Rc<RwLock<(f32, f32, f32)>>,
    timer: Instant,
}
impl GameDirector {
    pub fn new() -> Self {
        Self {
            xyz: Rc::new(RwLock::new((0.0, 0.0, 0.0))),
            timer: Instant::now(),
        }
    }
}
impl System for GameDirector {
    const UUID: Uuid = k9::uuid::uuid!("6ee51c3f-1e07-40e2-a40d-1f6f16e17a6f");
}
impl SystemCallbacks for GameDirector {
    fn first_call(&mut self, first_call_state: FirstCallState, frame_state: FrameState) {
        let ents = frame_state.ents;
        let mut ent = Entity::new();
        let tex_comp = GraphicsComponent::TexQuad(TexQuadBase::new());
        ent.add_component(tex_comp);

        ents.add_new_entity(ent);

        log::info!("have some info");
        log::trace!("have some trace");
        log::debug!("have some debug");
        log::warn!("have some warn");
        log::error!("have some error");

        let xyz = self.xyz.clone();
        let cc = console_command!("sample foo command", {opt x: f32, opt y: f32, opt z: f32}, |_, x, y, z| {
            let mut xyz = xyz.write().unwrap();
            if let Some(x) = x { xyz.0 = x };
            if let Some(y) = y { xyz.1 = y };
            if let Some(z) = z { xyz.2 = z };
            Ok(())
        });

        first_call_state
            .console_commands
            .insert("foo".to_owned(), cc);

        let cc = console_command!("sample four command.", {x: f32, y: f32, z: f32,}, |_, x, y, z| {
            Ok(())
        });

        first_call_state
            .console_commands
            .insert("four".to_owned(), cc);

        let cc = console_command!("sample friday command.", {x: f32, y: f32, z: f32,}, |_, x, y, z| {
            Ok(())
        });

        first_call_state
            .console_commands
            .insert("friday".to_owned(), cc);

        for _ in 0..100 {
            let cc = console_command!("sample many command.", {x: f32, y: f32, z: f32,}, |_, x, y, z| {
                Ok(())
            });

            let random_bytes = {
                let len = rand::random::<u32>() % 50 + 5;
                let mut v = "".to_owned();
                for _ in 0..len {
                    let c = 'a' as u8 + rand::random::<u8>() % 26;
                    v.push(c as char);
                }
                v
            };

            first_call_state
                .console_commands
                .insert(format!("many_{random_bytes}"), cc);
        }

        first_call_state
            .debug_windows
            .insert("foo_window".to_owned(), Box::new(FooWindow {}));

        let cc_foo_window: ConsoleCommand = console_command!(
            "foo window command.",
            { open: bool },
            |mut ccf: ConsoleCommandInterface, open: bool| {
                ccf.set_open_debug_window(&("foo_window".to_owned()), open);
                Ok(())
            }
        );

        first_call_state
            .console_commands
            .insert("foo_window".to_owned(), cc_foo_window);
    }

    fn update(&mut self, _state: FrameState) {
        if self.timer.elapsed().as_secs() > 3 {
            self.timer = Instant::now();
            log::debug!("{:?}", self.xyz.read().unwrap());
        }
    }
    fn exiting(&mut self, _state: FrameState) {}
}

struct FooWindow;
impl DebugUiWindow for FooWindow {
    fn draw(&mut self, ui: &mut k9::egui::Ui) {
        ui.label("hello from foo window");
    }
}
