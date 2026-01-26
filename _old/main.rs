#![no_main]
use plotters::prelude::*;
use plotters_canvas::CanvasBackend;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().expect("no window");
    let document = window.document().expect("no document");
    let canvas = document.get_element_by_id("plot-canvas").expect("no canvas");
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>()
        .map_err(|_| JsValue::from_str("Failed to cast to HtmlCanvasElement"))?;

    draw_on_canvas(&canvas)?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        draw_on_canvas(&canvas).unwrap();
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .expect("no window")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("request_animation_frame failed");
}

fn draw_on_canvas(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    let backend = CanvasBackend::with_canvas_object(canvas.clone())
        .ok_or_else(|| JsValue::from_str("Failed to create backend"))?;

    let root = backend.into_drawing_area();
    root.fill(&WHITE)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    let data: Vec<(f64, f64)> = (0..100).map(|x| (x as f64, (x as f64 / 10.0).sin())).collect();

    let mut chart = ChartBuilder::on(&root)
        .caption("Real-time Data", ("sans-serif", 30))
        .build_cartesian_2d(0.0..100.0, -1.0..1.0)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    chart.draw_series(LineSeries::new(data, &RED))
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    chart.configure_mesh().draw()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    root.present()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(())
}