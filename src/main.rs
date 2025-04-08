#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// GUI module
mod my_window;
//  p-scanner module
mod scanner;


use winsafe::{self as w, prelude::*, co};
use my_window::MyWindow;

fn main() {
	if let Err(e) = (|| MyWindow::new().run())() {
		w::HWND::NULL.MessageBox(
			&e.to_string(), "Uncaught error", co::MB::ICONERROR).unwrap();
	}
}
