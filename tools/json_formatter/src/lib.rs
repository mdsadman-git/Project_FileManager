use std::sync::Arc;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlButtonElement, HtmlElement, HtmlTextAreaElement};

use crate::console::*;
use crate::logic::*;

mod console;
mod logic;

fn window() -> web_sys::Window {
  web_sys::window().expect("No global 'window' exists!")
}

fn navigator() -> web_sys::Navigator {
	window().navigator()
}

fn document() -> web_sys::Document {
  window().document().expect("Should have a document on window!")
}

fn ta_element(id: &str) -> HtmlTextAreaElement {
	document().get_element_by_id(id).expect("Unable to get the text area element!").dyn_into::<HtmlTextAreaElement>().unwrap()
}

fn btn_element(id: &str) -> HtmlButtonElement {
	document().get_element_by_id(id).expect("Unable to get the button element!").dyn_into::<HtmlButtonElement>().unwrap()
}

fn any_element(id: &str) -> HtmlElement {
	document().get_element_by_id(id).expect("Unable to get the html element!").dyn_into::<HtmlElement>().unwrap()
}

#[wasm_bindgen(start)]
pub fn start() {
	let ta_input = ta_element("TxtAreaInput");
	let ta_result = any_element("DivResult");
	
	let btn_paste_to_input = btn_element("BtnPasteToInput");
	let btn_tab_length = btn_element("BtnTabLength");
	let btn_format = btn_element("BtnFormat");
	let btn_clear = btn_element("BtnClear");

	let arc_ta_input = Arc::new(ta_input);

	let cell_ta_input_1 = arc_ta_input.clone();
  let then_clipboard_read_text_cl = Closure::<(dyn FnMut(JsValue) + 'static)>::new(move |v: JsValue| {
		if let Some(copied_value) = v.as_string() {
			clog!("Got the value from clipboard");
			cell_ta_input_1.set_value(&copied_value);
		}
  });
	
  let catch_clipboard_read_text_cl = Closure::<(dyn FnMut(JsValue) + 'static)>::new(move |_: JsValue| {
		cerr!("Error while getting data from clipboard!");
  });

	let btn_paste_to_input_cl = Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
		let _ = navigator().clipboard().read_text()
			.then(&then_clipboard_read_text_cl)
			.catch(&catch_clipboard_read_text_cl);
	});

	let anim_end_btn_tab_length_cl = Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
		let btn_tab_length_text_flip = document().get_element_by_id("BtnTabLengthTextFlip").expect("").dyn_into::<HtmlElement>().unwrap();
		btn_tab_length_text_flip.class_list().remove_1("anim_flip_down_to_up").unwrap();
	});

	let btn_tab_length_cl = Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
		let btn_tab_length_text_flip = document().get_element_by_id("BtnTabLengthTextFlip").expect("").dyn_into::<HtmlElement>().unwrap();
		btn_tab_length_text_flip.class_list().add_1("anim_flip_down_to_up").unwrap();
		btn_tab_length_text_flip.add_event_listener_with_callback("animationend", anim_end_btn_tab_length_cl.as_ref().unchecked_ref()).unwrap();

		let timeout_closure = Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
			let btn_tab_length_text_flip = document().get_element_by_id("BtnTabLengthTextFlip").expect("").dyn_into::<HtmlElement>().unwrap();
			match btn_tab_length_text_flip.inner_text().parse::<i32>() {
				Ok(v)	=> {
					let tab_length = match v {
						0..8 => v + 1,
						_ => 2
					};

					btn_tab_length_text_flip.set_inner_text(&tab_length.to_string());
				},
				Err(e) => panic!("Invalid value found! {}", e)
			}
		});

		window().set_timeout_with_callback_and_timeout_and_arguments_0(timeout_closure.as_ref().unchecked_ref(), 125).unwrap();
		timeout_closure.forget();
	});

	let cell_ta_input_2 = arc_ta_input.clone();
  let btn_format_cl = Closure::<dyn FnMut(_)>::new(move |_: web_sys::Event| {
		let btn_tab_length_text_flip = document().get_element_by_id("BtnTabLengthTextFlip").expect("").dyn_into::<HtmlElement>().unwrap();
		let input_text = cell_ta_input_2.value();
		let tab_size = match btn_tab_length_text_flip.inner_text().parse::<i32>() {
			Ok(v) => v,
			Err(e) => {
				cerr!("Invalid value found! {:?}", e);
				panic!("{}", e);
			}
		};

		ta_result.set_inner_html(&Logic::format(&input_text, tab_size));
  });

	let cell_ta_input_3 = arc_ta_input.clone();
  let btn_clear_cl = Closure::<(dyn FnMut(JsValue))>::new(move |_: JsValue| {
		cell_ta_input_3.set_value("");
  });

	btn_paste_to_input.set_onclick(Some(btn_paste_to_input_cl.as_ref().unchecked_ref()));
	btn_tab_length.set_onclick(Some(btn_tab_length_cl.as_ref().unchecked_ref()));
	btn_format.set_onclick(Some(btn_format_cl.as_ref().unchecked_ref()));
	btn_clear.set_onclick(Some(btn_clear_cl.as_ref().unchecked_ref()));

	btn_tab_length_cl.forget();
	btn_paste_to_input_cl.forget(); 
	btn_format_cl.forget(); 
	btn_clear_cl.forget();
}
