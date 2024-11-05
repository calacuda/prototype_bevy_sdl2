// mod converters;
use crate::sdl2_windows::Sdl2Windows;
use bevy::prelude::*;
// use bevy_app::prelude::*;
use bevy_app::{AppExit, PluginsState};
// use bevy_ecs::storage::Resources;
// use bevy_input::keyboard::{ElementState, KeyboardInput};
// use bevy_input::mouse::{MouseButtonInput, MouseMotion};
// use bevy_math::prelude::*;
use bevy_window::{CursorMoved, PrimaryWindow, WindowCloseRequested, WindowCreated, WindowResized};
// use sdl2_windows::Sdl2WindowArgs;
// use sdl2::sys::Window;

mod sdl2_windows;

#[derive(Event)]
pub struct Sdl2Event(sdl2::event::Event);

/// Adds SDL2 windowing backend to Apps.
#[derive(Default)]
pub struct Sdl2Plugin;

impl Plugin for Sdl2Plugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.set_runner(sdl2_runner);
    }
}

pub fn sdl2_runner(mut app: App) -> AppExit {
    // Setup SDL
    let context = sdl2::init().expect("Failed to initialize sdl2");
    let video_subsystem = context
        .video()
        .expect("Failed to create sdl video subsystem");

    log::info!("Starting SDL2 window event loop");
    let mut event_pump = context
        .event_pump()
        .expect("Could not create sdl event pump");

    // This is not stored as a resource because SDL2 windows are not Send
    let mut sdl2_windows = Sdl2Windows::default();

    // let mut create_window_event_reader = EventReader::<CreateWindow>::default();
    // let mut app_exit_event_reader = EventReader::<AppExit>::default();

    // let mut create_window_event_reader = app.add_event::<Create>();

    create_primary_window(
        app.world_mut(),
        &video_subsystem,
        &mut sdl2_windows,
        // &mut create_window_event_reader,
    );

    // let window = Sdl2Window::new(
    //     &video_subsystem,
    //     Sdl2WindowArgs {
    //         title: "bevy-sdl".into(),
    //         width: 1280,
    //         height: 720,
    //     },
    //     &mut app,
    // );

    // let title = "bevy-sdl2";
    //
    // let sdl2_window = video_subsystem
    //     .window(&title, 1280, 720)
    //     .position_centered()
    //     .resizable()
    //     .build()
    //     .expect("Failed to create window");
    //
    // let bevy_window = Window {
    //     mode: bevy_window::WindowMode::Fullscreen,
    //     title: title.into(),
    //     name: Some(title.into()),
    //     focused: true,
    //     ..default()
    // };
    //
    // let mut commands = app.world_mut().commands();
    //
    // let window = commands.spawn((bevy_window, PrimaryWindow));
    //
    // app.world().send_event(WindowCreated {
    //     window: window.id(),
    // });

    // SDL2 has a lot of events we can support, expose them all via an event
    app.add_event::<Sdl2Event>();

    if app.plugins_state() == PluginsState::Ready {
        app.finish();
        app.cleanup();
    }

    log::debug!("Entering SDL2 event loop");

    loop {
        // if let Some(app_exit_events) = app.resources.get_mut::<Events<AppExit>>() {
        //     if app_exit_event_reader.latest(&app_exit_events).is_some() {
        //         break 'running;
        //     }
        // }
        if let Some(exit) = app.should_exit() {
            eprintln!("exiting form runner loop");
            return exit;
        }

        {
            let mut sdl2_events = app
                .world_mut()
                // .iter_resources_mut()
                .resource_mut::<Events<Sdl2Event>>();

            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Window {
                        window_id: _,
                        win_event,
                        ..
                    } => match win_event {
                        sdl2::event::WindowEvent::Close => {
                            // let mut window_close_requested_events = app
                            //     // .resources
                            //     .world_mut()
                            //     .resource_mut::<Events<WindowCloseRequested>>();
                            // let window_id = sdl2_windows.get_window_id(window_id).unwrap();
                            // window_close_requested_events
                            //     .send(WindowCloseRequested { id: window_id });
                            log::info!("window closed, exiting event loop...");
                            return AppExit::Success;
                        }
                        // sdl2::event::WindowEvent::Resized(width, height) => {
                        //     // let mut windows = app.world_mut().resource_mut::>();
                        //     let bevy_window_id = sdl2_windows.get_window_id(window_id).unwrap();
                        //     let mut window = windows.get_mut(bevy_window_id).unwrap();
                        //     window.width = width as u32;
                        //     window.height = height as u32;
                        //
                        //     let mut resize_events =
                        //         app.resources.get_mut::<Events<WindowResized>>().unwrap();
                        //     resize_events.send(WindowResized {
                        //         id: bevy_window_id,
                        //         height: window.height as usize,
                        //         width: window.width as usize,
                        //     });
                        // }
                        _ => {}
                    },
                    // sdl2::event::Event::KeyDown {
                    //     keycode, scancode, ..
                    // } => {
                    //     send_key_event(&app, keycode, scancode, ElementState::Pressed);
                    // }
                    // sdl2::event::Event::KeyUp {
                    //     keycode, scancode, ..
                    // } => {
                    //     send_key_event(&app, keycode, scancode, ElementState::Released);
                    // }
                    // sdl2::event::Event::MouseMotion {
                    //     window_id,
                    //     x,
                    //     y,
                    //     xrel,
                    //     yrel,
                    //     ..
                    // } => {
                    //     let mut cursor_moved_events =
                    //         app.resources.get_mut::<Events<CursorMoved>>().unwrap();
                    //
                    //     let bevy_window_id = sdl2_windows.get_window_id(window_id).unwrap();
                    //     let window = sdl2_windows.get_window(bevy_window_id).unwrap();
                    //     let (_width, height) = window.size();
                    //     // move origin to bottom left
                    //     let y_position = height as i32 - y;
                    //     cursor_moved_events.send(CursorMoved {
                    //         id: bevy_window_id,
                    //         position: Vec2::new(x as f32, y_position as f32),
                    //     });
                    //
                    //     let mut mouse_motion_events =
                    //         app.resources.get_mut::<Events<MouseMotion>>().unwrap();
                    //     mouse_motion_events.send(MouseMotion {
                    //         delta: Vec2::new(xrel as f32, yrel as f32),
                    //     });
                    // }
                    // sdl2::event::Event::MouseButtonDown { mouse_btn, .. } => {
                    //     send_mouse_event(
                    //         &app,
                    //         mouse_btn,
                    //         bevy_input::keyboard::ElementState::Pressed,
                    //     );
                    // }
                    // sdl2::event::Event::MouseButtonUp { mouse_btn, .. } => {
                    //     send_mouse_event(
                    //         &app,
                    //         mouse_btn,
                    //         bevy_input::keyboard::ElementState::Released,
                    //     );
                    // }
                    _ => {}
                }

                sdl2_events.send(Sdl2Event(event));
            }
        }

        // handle_create_window_events(
        //     &mut app.resources,
        //     &video_subsystem,
        //     &mut sdl2_windows,
        //     &mut create_window_event_reader,
        // );
        app.update();
    }
}

// fn send_key_event(
//     app: &App,
//     keycode: Option<sdl2::keyboard::Keycode>,
//     scancode: Option<sdl2::keyboard::Scancode>,
//     element_state: bevy_input::keyboard::ElementState,
// ) {
//     let mut keyboard_input_events = app.resources.get_mut::<Events<KeyboardInput>>().unwrap();
//
//     // These options are due to conversion from C types to i32.
//     // - Keycode is typedeffed as i32
//     // - Scancode is a C array with max value of 512. It is intended that end-users
//     //   can make arrays of this length.
//     //
//     // So it is expected that the i32 conversion will always succeed and these unwrap
//     // are infallible
//     let keycode = keycode.unwrap();
//     let scancode = scancode.unwrap();
//     keyboard_input_events.send(converters::convert_keyboard_input(
//         keycode,
//         scancode,
//         element_state,
//     ));
// }
//
// fn send_mouse_event(
//     app: &App,
//     mouse_btn: sdl2::mouse::MouseButton,
//     element_state: bevy_input::keyboard::ElementState,
// ) {
//     let mut mouse_button_input_events =
//         app.resources.get_mut::<Events<MouseButtonInput>>().unwrap();
//     mouse_button_input_events.send(MouseButtonInput {
//         button: converters::convert_mouse_button(mouse_btn),
//         state: element_state,
//     });
// }

// fn handle_create_window_events(
//     app: &mut App,
//     sdl2_video_subsystem: &sdl2::VideoSubsystem,
//     sdl2_window: &mut Sdl2Window,
//     // create_window_event_reader: &mut EventReader<CreateWindow>,
// ) {
//     // let mut windows = resources.get_mut::<Windows>().unwrap();
//     // let create_window_events = resources.get::<Events<CreateWindow>>().unwrap();
//     // let create_window_events = world.get_resource_mut::<Events<CreateWindow>>().unwrap();
//     // let mut window_created_events = world.get_resource_mut::<Events<WindowCreated>>().unwrap();
//
//     // for create_window_event in create_window_event_reader.iter(&create_window_events) {
//     // for create_window_event in  {
//     let window = sdl2_window::new(sdl2_video_subsystem);
//     // let window_id = window.id;
//     // windows.add(window);
//
//     // window_created_events.send(WindowCreated { window });
//     // }
//     app.insert_resource(window)
// }

fn do_create_primary_window(
    world: &mut World,
    sdl2_video_subsystem: &sdl2::VideoSubsystem,
    sdl2_windows: &mut Sdl2Windows,
    // create_window_event_reader: &mut EventReader<WindowCreated>,
) -> Entity {
    // let mut windows = world.spawn().unwrap();
    // let create_window_events = resources.get::<Events<CreateWindow>>().unwrap();
    // let mut window_created_events = resources.get_mut::<Events<WindowCreated>>().unwrap();
    // for create_window_event in create_window_event_reader.iter(&create_window_events) {

    let mut entity = world.spawn_empty();
    let window = sdl2_windows.create_window(sdl2_video_subsystem, entity.id());
    // let window_id = window;
    entity.insert((window, PrimaryWindow));
    // windows.add(window);
    // world.spawn()
    // world.send_event(WindowCreated {
    //     window: entity.id(),
    // });

    // .send(WindowCreated { window: window });
    // }
    entity.id()
}

fn create_primary_window(
    world: &mut World,
    sdl2_video_subsystem: &sdl2::VideoSubsystem,
    sdl2_windows: &mut Sdl2Windows,
    // create_window_event_reader: &mut EventReader<WindowCreated>,
) {
    let w_id = do_create_primary_window(world, sdl2_video_subsystem, sdl2_windows);

    world.send_event(WindowCreated { window: w_id });
    // }
}
