/* LICENSE BEGIN
    This file is part of the SixtyFPS Project -- https://sixtyfps.io
    Copyright (c) 2020 Olivier Goffart <olivier.goffart@sixtyfps.io>
    Copyright (c) 2020 Simon Hausmann <simon.hausmann@sixtyfps.io>

    SPDX-License-Identifier: GPL-3.0-only
    This file is also available under commercial licensing terms.
    Please contact info@sixtyfps.io for more information.
LICENSE END */
#![warn(missing_docs)]
//! Exposed Window API

use crate::component::ComponentRc;
use crate::graphics::Point;
use crate::input::{KeyEvent, MouseEventType};
use crate::items::{ItemRc, ItemRef};
use crate::slice::Slice;
use core::pin::Pin;
use std::rc::Rc;

/// This trait represents the interface that the generated code and the run-time
/// require in order to implement functionality such as device-independent pixels,
/// window resizing and other typicaly windowing system related tasks.
///
/// [`crate::graphics`] provides an implementation of this trait for use with [`crate::graphics::GraphicsBackend`].
pub trait GenericWindow {
    /// Associates this window with the specified component. Further event handling and rendering, etc. will be
    /// done with that component.
    fn set_component(self: Rc<Self>, component: &ComponentRc);

    /// Draw the items of the specified `component` in the given window.
    fn draw(self: Rc<Self>);
    /// Receive a mouse event and pass it to the items of the component to
    /// change their state.
    ///
    /// Arguments:
    /// * `pos`: The position of the mouse event in window physical coordinates.
    /// * `what`: The type of mouse event.
    /// * `component`: The SixtyFPS compiled component that provides the tree of items.
    fn process_mouse_input(
        self: Rc<Self>,
        pos: winit::dpi::PhysicalPosition<f64>,
        what: MouseEventType,
    );
    /// Receive a key event and pass it to the items of the component to
    /// change their state.
    ///
    /// Arguments:
    /// * `event`: The key event received by the windowing system.
    /// * `component`: The SixtyFPS compiled component that provides the tree of items.
    fn process_key_input(self: Rc<Self>, event: &KeyEvent);
    /// Calls the `callback` function with the underlying winit::Window that this
    /// GenericWindow backs.
    fn with_platform_window(&self, callback: &mut dyn FnMut(&winit::window::Window));
    /// Requests for the window to be mapped to the screen.
    ///
    /// Arguments:
    /// * `event_loop`: The event loop used to drive further event handling for this window
    ///   as it will receive events.
    /// * `component`: The component that holds the root item of the scene. If the item is a [`crate::items::Window`], then
    ///   the `width` and `height` properties are read and the values are passed to the windowing system as request
    ///   for the initial size of the window. Then bindings are installed on these properties to keep them up-to-date
    ///   with the size as it may be changed by the user or the windowing system in general.
    fn map_window(self: Rc<Self>, event_loop: &crate::eventloop::EventLoop);
    /// Removes the window from the screen. The window is not destroyed though, it can be show (mapped) again later
    /// by calling [`GenericWindow::map_window`].
    fn unmap_window(self: Rc<Self>);
    /// Issue a request to the windowing system to re-render the contents of the window. This is typically an asynchronous
    /// request.
    fn request_redraw(&self);
    /// Returns the scale factor set on the window, as provided by the windowing system.
    fn scale_factor(&self) -> f32;
    /// Sets an overriding scale factor for the window. This is typically only used for testing.
    fn set_scale_factor(&self, factor: f32);
    /// Sets the size of the window to the specified `width`. This method is typically called in response to receiving a
    /// window resize event from the windowing system.
    fn set_width(&self, width: f32);
    /// Sets the size of the window to the specified `height`. This method is typically called in response to receiving a
    /// window resize event from the windowing system.
    fn set_height(&self, height: f32);
    /// Returns the geometry of the window
    fn get_geometry(&self) -> crate::graphics::Rect;

    /// This function is called by the generated code when a component and therefore its tree of items are destroyed. The
    /// implementation typically uses this to free the underlying graphics resources cached via [`crate::graphics::RenderingCache`].
    fn free_graphics_resources<'a>(self: Rc<Self>, items: &Slice<'a, Pin<ItemRef<'a>>>);
    /// Installs a binding on the specified property that's toggled whenever the text cursor is supposed to be visible or not.
    fn set_cursor_blink_binding(&self, prop: &crate::properties::Property<bool>);

    /// Returns the currently active keyboard notifiers.
    fn current_keyboard_modifiers(&self) -> crate::input::KeyboardModifiers;
    /// Sets the currently active keyboard notifiers. This is used only for testing or directly
    /// from the event loop implementation.
    fn set_current_keyboard_modifiers(&self, modifiers: crate::input::KeyboardModifiers);

    /// Sets the focus to the item pointed to by item_ptr. This will remove the focus from any
    /// currently focused item.
    fn set_focus_item(self: Rc<Self>, focus_item: &ItemRc);
    /// Sets the focus on the window to true or false, depending on the have_focus argument.
    /// This results in WindowFocusReceived and WindowFocusLost events.
    fn set_focus(self: Rc<Self>, have_focus: bool);

    /// Show a popup at the given position
    fn show_popup(&self, popup: &ComponentRc, position: Point);
    /// Close the active popup if any
    fn close_popup(&self);
}

/// The ComponentWindow is the (rust) facing public type that can render the items
/// of components to the screen.
#[repr(C)]
#[derive(Clone)]
pub struct ComponentWindow(pub std::rc::Rc<dyn crate::eventloop::GenericWindow>);

impl ComponentWindow {
    /// Creates a new instance of a CompomentWindow based on the given window implementation. Only used
    /// internally.
    pub fn new(window_impl: std::rc::Rc<dyn crate::eventloop::GenericWindow>) -> Self {
        Self(window_impl)
    }
    /// Spins an event loop and renders the items of the provided component in this window.
    pub fn run(&self) {
        let event_loop = crate::eventloop::EventLoop::new();

        self.0.clone().map_window(&event_loop);

        event_loop.run();

        self.0.clone().unmap_window();
    }

    /// Returns the scale factor set on the window.
    pub fn scale_factor(&self) -> f32 {
        self.0.scale_factor()
    }

    /// Sets an overriding scale factor for the window. This is typically only used for testing.
    pub fn set_scale_factor(&self, factor: f32) {
        self.0.set_scale_factor(factor)
    }

    /// This function is called by the generated code when a component and therefore its tree of items are destroyed. The
    /// implementation typically uses this to free the underlying graphics resources cached via [RenderingCache][`crate::graphics::RenderingCache`].
    pub fn free_graphics_resources<'a>(&self, items: &Slice<'a, Pin<ItemRef<'a>>>) {
        self.0.clone().free_graphics_resources(items);
    }

    /// Installs a binding on the specified property that's toggled whenever the text cursor is supposed to be visible or not.
    pub(crate) fn set_cursor_blink_binding(&self, prop: &crate::properties::Property<bool>) {
        self.0.clone().set_cursor_blink_binding(prop)
    }

    /// Sets the currently active keyboard notifiers. This is used only for testing or directly
    /// from the event loop implementation.
    pub(crate) fn set_current_keyboard_modifiers(
        &self,
        modifiers: crate::input::KeyboardModifiers,
    ) {
        self.0.clone().set_current_keyboard_modifiers(modifiers)
    }

    /// Returns the currently active keyboard notifiers.
    pub(crate) fn current_keyboard_modifiers(&self) -> crate::input::KeyboardModifiers {
        self.0.clone().current_keyboard_modifiers()
    }

    pub(crate) fn process_key_input(&self, event: &KeyEvent) {
        self.0.clone().process_key_input(event)
    }

    /// Clears the focus on any previously focused item and makes the provided
    /// item the focus item, in order to receive future key events.
    pub fn set_focus_item(&self, focus_item: &ItemRc) {
        self.0.clone().set_focus_item(focus_item)
    }

    /// Associates this window with the specified component, for future event handling, etc.
    pub fn set_component(&self, component: &ComponentRc) {
        self.0.clone().set_component(component)
    }

    /// Show a popup at the given position
    pub fn show_popup(&self, popup: &ComponentRc, position: Point) {
        self.0.clone().show_popup(popup, position)
    }
    /// Close the active popup if any
    pub fn close_popup(&self) {
        self.0.clone().close_popup()
    }
}
