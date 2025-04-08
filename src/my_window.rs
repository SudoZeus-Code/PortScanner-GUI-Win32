use winsafe::{self as w, prelude::*, gui};

use winsafe::co::ES;
use winsafe::co::WS;

// importing here and in scanner.rs? sure 
use std::sync::mpsc;


#[derive(Clone)]
pub struct MyWindow {
	wnd:        gui::WindowMain,
	lbl_name:   gui::Label,
	txt_name:   gui::Edit,
	sbar:       gui::StatusBar,
	// button to kick off scan
	btn_scan:	gui::Button,
	txt_output: gui::Edit,
}

impl MyWindow {
	pub fn new() -> Self {
		let wnd = gui::WindowMain::new( 
			gui::WindowMainOpts {
				title: "Port Scanner".to_owned(),
				class_icon: gui::Icon::Id(101),
				size: (580, 240),
				..Default::default()
			},
		);

		// From now on, create the child controls.

		// Note that we are creating each control manually, which means we must
		// specify the x,y coordinates by hand. This is very tedious. That's
		// why, when creating many controls, it's easier to use dialog boxes,
		// since we can place the controls visually by using a WYSIWYG resource
		// editor (like Visual Studio).

		let btn_scan = gui::Button::new(
			&wnd,
			gui::ButtonOpts{
				text: "Scan".to_owned(),
				position: (200,18),
				width: 80,
				..Default::default()
			},

		);

		let lbl_name = gui::Label::new(
			&wnd,
			gui::LabelOpts {
				position: (20, 23),
				text: "Target:".to_owned(),
				..Default::default()
			},
		);

		let txt_name = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (70, 20),
				width: 120,
				..Default::default()
			},
		);


		let sbar = gui::StatusBar::new(
			&wnd,
			&[
				gui::SbPart::Proportional(1),
				gui::SbPart::Fixed(160),
			],
		);

		let txt_output = gui::Edit::new(
			&wnd,
			gui::EditOpts {
				position: (20, 60),
				width: 520,
				height: 120,
				edit_style: ES::MULTILINE | ES::READONLY | ES::WANTRETURN,
				window_style: WS::VSCROLL | WS::VISIBLE | WS::CHILD,
				..Default::default()
			},

		);

		let new_self = Self {
			wnd,
			lbl_name,
			txt_name,
			sbar,
			btn_scan,
			txt_output,
		};

		new_self.events();
		new_self
	}

	pub fn run(&self) -> w::AnyResult<i32> {
		self.wnd.run_main(None)
	}

	fn events(&self) {
		let self2 = self.clone();
		self.wnd.on().wm_create(move |_| { // called once, right after the window is created
			self2.sbar.parts().get(0).set_text("68 61 63 6B 74 68 65 70 6C 61 6E 65 74");
			self2.sbar.parts().get(1).set_text("Sudo_Zeus");
			self2.txt_name.focus();
			Ok(0)
		});

		let self3 = self.clone();
		self.btn_scan.on().bn_clicked(move || {
			// no need to match somthing that is already a string. 
			let target = self3.txt_name.text(); // grab target from edit box

			// scan output channel
			let (tx,rx) = mpsc::channel::<String>();

			// Thread to update output box
			let output_control = self3.txt_output.clone();

			//create a new thread yo
			std::thread::spawn(move || {
				while let Ok(line) = rx.recv() {
					//output_control.append_text(&line);
					let existing = output_control.text();
					let updated = format!("{}{}", existing, line);
					let _ = output_control.set_text(&updated);
				}
			});

			std::thread::spawn(move || {
				crate::scanner::run_scan(target, tx);
			});

			Ok(())
		});
	}

}
