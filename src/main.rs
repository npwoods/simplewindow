use anyhow::Result;
use clap::Parser;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
//use std::mem::MaybeUninit;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowId;
//use x11::xlib;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// Window width in pixels
	#[arg(long, default_value = "300")]
	width: u32,

	/// Window height in pixels
	#[arg(long, default_value = "200")]
	height: u32,

	/// Window title
	#[arg(long, default_value = "Simple Window")]
	title: String,
}

struct App {
	args: Args,
	window: Option<Window>,
}

impl App {
	fn new(args: Args) -> Self {
		Self { args, window: None }
	}
}

impl ApplicationHandler for App {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		// Create the window once when the app is resumed
		if self.window.is_some() {
			return;
		}

		let window_attributes = Window::default_attributes()
			.with_title(self.args.title.clone())
			.with_inner_size(PhysicalSize::new(self.args.width, self.args.height));

		match event_loop.create_window(window_attributes) {
			Ok(win) => {
				// Print platform-specific identifier
				match win.window_handle() {
					Ok(wh) => {
						match wh.as_raw() {
							RawWindowHandle::Win32(handle) => {
								// NonZeroIsize -> get() returns isize
								let hwnd = handle.hwnd.get() as usize;
								println!("HWND: 0x{:X} ({})", hwnd, hwnd);
							}
							RawWindowHandle::Xlib(handle) => {
								let window = handle.window as usize;
								println!("X11 Window ID (Xlib): 0x{:X} ({})", window, window);

								/*
								// Unselect specific event masks that winit may have set
								unsafe {
									if !handle.display.is_null() {
										let display = handle.display as *mut xlib::Display;
										let mut attrs = MaybeUninit::<xlib::XWindowAttributes>::uninit();
										let status = xlib::XGetWindowAttributes(display, handle.window as xlib::Window, attrs.as_mut_ptr());
										if status != 0 {
											let attrs = attrs.assume_init();
											let current_mask = attrs.your_event_mask;
											let masks_to_unset = (xlib::StructureNotifyMask as xlib::long)
												| (xlib::ExposureMask as xlib::long)
												| (xlib::KeyPressMask as xlib::long)
												| (xlib::KeyReleaseMask as xlib::long)
												| (xlib::FocusChangeMask as xlib::long);
											let new_mask = current_mask & !masks_to_unset;
											xlib::XSelectInput(display, handle.window as xlib::Window, new_mask);
											xlib::XFlush(display);
										}
									}
								}
								 */
							}
							RawWindowHandle::Xcb(handle) => {
								let window = handle.window.get() as usize;
								println!("X11 Window ID (XCB): 0x{:X} ({})", window, window);
							}
							RawWindowHandle::Wayland(handle) => {
								let surface_ptr = handle.surface.as_ptr() as usize;
								println!("Wayland surface: 0x{:X} ({:?})", surface_ptr, handle.surface);
							}
							other => {
								println!("RawWindowHandle: {:?}", other);
							}
						}
					}
					Err(e) => {
						eprintln!("Failed to get window handle: {:?}", e);
					}
				}

				self.window = Some(win);
			}
			Err(e) => {
				eprintln!("Failed to create window: {:?}", e);
			}
		}
	}

	fn window_event(&mut self, event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
		if let WindowEvent::CloseRequested = event {
			event_loop.exit();
		}
	}
}

fn main() -> Result<()> {
	let args = Args::parse();
	let event_loop = create_event_loop()?;
	let mut app = App::new(args);
	event_loop.run_app(&mut app)?;
	Ok(())
}

#[cfg(target_os = "linux")]
fn create_event_loop() -> Result<EventLoop<()>, winit::error::EventLoopError> {
	use winit::platform::x11::EventLoopBuilderExtX11;
	// Prefer X11 builder first to avoid initializing Wayland and causing "EventLoop can't be recreated" errors.
	let mut builder = EventLoop::builder();
	builder.with_x11();
	builder.build().or_else(|_| EventLoop::new())
}

#[cfg(not(target_os = "linux"))]
fn create_event_loop() -> Result<EventLoop<()>, winit::error::EventLoopError> {
	EventLoop::new()
}
