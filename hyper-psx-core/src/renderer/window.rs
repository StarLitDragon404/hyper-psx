/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use std::{cell::RefCell, rc::Rc};

use thiserror::Error;
use winit::{
    dpi::LogicalSize,
    error::OsError,
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::{self, WindowBuilder},
};

/// The error type for the window
#[derive(Debug, Error)]
pub enum CreationError {
    /// If the Window failed to load
    #[error("failed to create window")]
    WindowCreationFailure(#[from] OsError),

    /// If the Pixles buffer failed to create
    #[error("failed to create pixels buffer")]
    PixelsCreationFailure(#[from] pixels::Error),
}

/// The window
#[derive(Debug)]
pub(crate) struct Window {
    /// The window event loop
    event_loop: Rc<RefCell<EventLoop<()>>>,

    /// The internal handle
    window: window::Window,
}

impl Window {
    /// Creates a new window
    pub(crate) fn new() -> Result<Self, CreationError> {
        let event_loop = EventLoop::new();
        let window = {
            let size = LogicalSize::new(1024, 512);
            WindowBuilder::new()
                .with_title("Hyper-PSX")
                .with_inner_size(size)
                .build(&event_loop)?
        };

        Ok(Self {
            event_loop: Rc::new(RefCell::new(event_loop)),
            window,
        })
    }

    /// Runs the event loop
    ///
    /// Arguments:
    ///
    /// * `handle`: The update handle
    pub(crate) fn run<F>(&mut self, mut handle: F)
    where
        F: FnMut(&Event<()>),
    {
        let event_loop = self.event_loop.clone();

        event_loop
            .borrow_mut()
            .run_return(|event, _, control_flow| {
                *control_flow = ControlFlow::Poll;

                handle(&event);

                match event {
                    Event::MainEventsCleared => {
                        self.window.request_redraw();
                    }
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }
                        WindowEvent::KeyboardInput { input, .. } => {
                            if input.state != ElementState::Pressed {
                                return;
                            }

                            let Some(virtual_key_code) = input.virtual_keycode else {
                                return;
                            };

                            if virtual_key_code == VirtualKeyCode::Escape {
                                *control_flow = ControlFlow::Exit;
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                }
            });
    }

    /// Returns the window size
    pub(super) fn size(&self) -> (u32, u32) {
        (
            self.window.inner_size().width,
            self.window.inner_size().height,
        )
    }

    /// Returns the internal handle
    pub(super) fn internal(&self) -> &window::Window {
        &self.window
    }
}
