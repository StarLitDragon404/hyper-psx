/*
 * Copyright (c) 2023, SkillerRaptor
 *
 * SPDX-License-Identifier: MIT
 */

use cgmath::Vector2;
use glfw::{Action, Context, Glfw, InitError, Key, WindowEvent, WindowMode};
use std::sync::mpsc::Receiver;
use thiserror::Error;

/// The error type for the window
#[derive(Debug, Error)]
pub enum CreationError {
    /// If the window failed to create
    #[error("failed to create window")]
    WindowCreationFailure,

    /// If the GLFW failed to initialize
    #[error("failed to initialize glfw")]
    GlfwInitFailure(#[from] InitError),
}

/// The window
#[derive(Debug)]
pub(crate) struct Window {
    /// The interna glfw representation
    glfw: Glfw,

    /// The internal handle
    window: glfw::Window,

    /// The event receiver
    events: Receiver<(f64, WindowEvent)>,
}

impl Window {
    /// Creates a new window
    pub(crate) fn new() -> Result<Self, CreationError> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

        let (mut window, events) = glfw
            .create_window(1024, 512, "Hyper-PSX", WindowMode::Windowed)
            .ok_or(CreationError::WindowCreationFailure)?;

        window.set_key_polling(true);
        window.set_size_polling(true);
        window.make_current();

        Ok(Self {
            glfw,
            window,
            events,
        })
    }

    /// Polls the latest events
    pub(crate) fn poll_events(&mut self) {
        self.glfw.poll_events();
    }

    /// Handles all events
    ///
    /// Arguments:
    ///
    /// * `event_handler`: The event handler
    pub(crate) fn handle_events<F>(&mut self, mut event_handler: F)
    where
        F: FnMut(&WindowEvent),
    {
        for (_, event) in glfw::flush_messages(&self.events) {
            event_handler(&event);

            if let WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                self.window.set_should_close(true);
            }
        }
    }

    /// Tells if the window should close
    pub(crate) fn should_close(&self) -> bool {
        self.window.should_close()
    }

    /// Returns the window size
    pub(super) fn size(&self) -> Vector2<u32> {
        let size = self.window.get_size();

        Vector2 {
            x: size.0 as u32,
            y: size.1 as u32,
        }
    }

    /// Returns the internal handle
    pub(super) fn internal(&self) -> &glfw::Window {
        &self.window
    }
}
