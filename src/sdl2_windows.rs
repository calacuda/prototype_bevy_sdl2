use bevy::{
    prelude::{BuildChildren, Entity, Event, EventWriter},
    utils::default,
};
use bevy_app::App;
use bevy_window::{PrimaryWindow, Window, WindowCreated};
use raw_window_handle::HasRawWindowHandle;
use std::collections::HashMap;

pub struct Sdl2WindowArgs {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Default)]
pub struct Sdl2Windows {
    pub windows: HashMap<u32, sdl2::video::Window>,
    pub bevy_id_to_sdl2_id: HashMap<Entity, u32>,
    pub sdl2_id_to_bevy_id: HashMap<u32, Entity>,
}

impl Sdl2Windows {
    pub fn create_window(
        &mut self,
        sdl2_video_subsystem: &sdl2::VideoSubsystem,
        // window_event: &CreateWindow,
        window_id: Entity,
    ) -> Window {
        let sdl2_window = sdl2_video_subsystem
            .window(
                &"App", // window_event.descriptor.width,
                1280,   // window_event.descriptor.height,
                720,
            )
            .position_centered()
            .resizable()
            .build()
            .expect("Failed to create window");

        // let raw_window_handle = BevyRawWindowHandle(sdl2_window.raw_window_handle());

        let sdl2_id = sdl2_window.id();
        self.windows.insert(sdl2_id, sdl2_window);
        self.bevy_id_to_sdl2_id.insert(window_id, sdl2_id);
        self.sdl2_id_to_bevy_id.insert(sdl2_id, window_id);

        // Window::new(window_event.id, raw_window_handle, &window_event.descriptor)
        Window {
            mode: bevy_window::WindowMode::Fullscreen,
            // title: args.title.clone(),
            // name: Some(args.title),
            focused: true,
            ..default()
        }
    }

    pub fn get_window(
        &self,
        id: Entity,
    ) -> Option<&sdl2::video::Window> {
        self.bevy_id_to_sdl2_id
            .get(&id)
            .and_then(|id| self.windows.get(id))
    }

    pub fn get_window_id(
        &self,
        id: u32,
    ) -> Option<Entity> {
        self.sdl2_id_to_bevy_id.get(&id).map(|x| *x)
    }
}

// // #[derive(Default)]
// pub struct Sdl2Window {
//     // pub windows: HashMap<u32, sdl2::video::Window>,
//     // pub bevy_id_to_sdl2_id: HashMap<WindowId, u32>,
//     // pub sdl2_id_to_bevy_id: HashMap<u32, WindowId>,
//     pub window: sdl2::video::Window,
// }
//
// impl Sdl2Window {
//     pub fn new(
//         // &mut self,
//         sdl2_video_subsystem: &sdl2::VideoSubsystem,
//         // window_event: &CreateWindow,
//         args: Sdl2WindowArgs,
//         app: &mut App,
//     ) -> Self {
//         let sdl2_window = sdl2_video_subsystem
//             .window(&args.title, args.width, args.height)
//             .position_centered()
//             .resizable()
//             .build()
//             .expect("Failed to create window");
//
//         // let raw_window_handle = BevyRawWindowHandle(sdl2_window.raw_window_handle());
//
//         // let sdl2_id = sdl2_window.id();
//         // self.windows.insert(sdl2_id, sdl2_window);
//         // self.bevy_id_to_sdl2_id.insert(window_event.id, sdl2_id);
//         // self.sdl2_id_to_bevy_id.insert(sdl2_id, window_event.id);
//
//         // Window::new(window_event.id, raw_window_handle, &window_event.descriptor)
//         let bevy_window = Window {
//             mode: bevy_window::WindowMode::Fullscreen,
//             title: args.title.clone(),
//             name: Some(args.title),
//             focused: true,
//             ..default()
//         };
//
//         let window = app
//             .world_mut()
//             .commands()
//             .spawn((bevy_window, PrimaryWindow));
//
//         app.world_mut().send_event(WindowCreated {
//             window: window.id(),
//         });
//
//         Self {
//             window: sdl2_window,
//         }
//     }
//
//     // pub fn get_window(
//     //     &self,
//     //     id: WindowId,
//     // ) -> Option<&sdl2::video::Window> {
//     //     self.bevy_id_to_sdl2_id
//     //         .get(&id)
//     //         .and_then(|id| self.windows.get(id))
//     // }
//
//     // pub fn get_window_id(
//     //     &self,
//     //     id: u32,
//     // ) -> Option<WindowId> {
//     //     self.sdl2_id_to_bevy_id.get(&id).map(|x| *x)
//     // }
// }
