use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

struct Ferris {
    // physics
    pos_x: i32,
    pos_y: i32,
    vel_x: i32,
    vel_y: i32,

    // html
    html: Option<web_sys::HtmlElement>,
}

const CANVAS_WIDTH: i32 = 800;
const CANVAS_HEIGHT: i32 = 600;
const FERRIS_WIDTH: i32 = 80;
const FERRIS_HEIGHT: i32 = 50;

const ANIM_DELAY: f64 = 0.025;

// in pxl/frame and pxl/frame **2
const INITIAL_VELOCITY_X: i32 = 10;
const INITIAL_VELOCITY_Y: i32 = 15;
const ACCELERATION_Y: i32 = -1;

impl Ferris {
    pub fn new(px: i32, py: i32) {
        // if out of the canvas, lets not even bother instantiate a Ferris

        if !Ferris::is_in_canvas(px, py) {
            return;
        }

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        // create ferris html element and rust struct

        let html = document
            .create_element("div")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        html.set_attribute("class", "ferris").unwrap();

        document
            .get_element_by_id("canvas")
            .unwrap()
            .append_child(&html)
            .unwrap();
        let mut ferris = Ferris {
            pos_x: px,
            pos_y: py,
            vel_x: INITIAL_VELOCITY_X,
            vel_y: INITIAL_VELOCITY_Y,
            html: Some(html),
        };

        ferris.update();

        // register call back

        let callback = Closure::wrap(Box::new(move || {
            ferris.update();
        }) as Box<dyn FnMut()>);

        window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                callback.as_ref().unchecked_ref(),
                (ANIM_DELAY * 1000.0) as i32,
            )
            .unwrap();

        callback.forget();
    }
    pub fn is_in_canvas(pos_x: i32, pos_y: i32) -> bool {
        (pos_x - FERRIS_WIDTH / 2 > 0)
            && (pos_x + FERRIS_WIDTH / 2 < CANVAS_WIDTH)
            && (pos_y - FERRIS_HEIGHT / 2 > 0)
            && (pos_y + FERRIS_HEIGHT / 2 < CANVAS_WIDTH)
    }
    pub fn update(self: &mut Ferris) {
        // if html element does not exist, do nothing and return

        let html = match &self.html {
            None => return,
            Some(html) => html,
        };

        // update vel and pos
        self.vel_y += ACCELERATION_Y;
        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;

        if !Ferris::is_in_canvas(self.pos_x, self.pos_y) {
            html.remove();
            self.html = None;
            return;
        }

        // display

        html.style()
            .set_property(
                "top",
                &format!("{:}px", (CANVAS_HEIGHT - self.pos_y) - FERRIS_HEIGHT / 2),
            )
            .unwrap();
        html.style()
            .set_property("left", &format!("{:}px", self.pos_x - FERRIS_WIDTH / 2))
            .unwrap();
    }
}

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();

    // Hello from Rust
    let p = document.create_element("p").unwrap();
    p.set_inner_html("Hello from Rust!");
    body.append_child(&p).unwrap();

    // create canvas
    let canvas = document.create_element("div").unwrap();
    canvas.set_id("canvas");
    body.append_child(&canvas).unwrap();

    // mousedown callback
    let callback = Closure::wrap(Box::new(|event: web_sys::MouseEvent| {
        Ferris::new(event.offset_x(), CANVAS_HEIGHT - event.offset_y());
    }) as Box<dyn Fn(_)>);

    canvas.add_event_listener_with_callback(
        "mousedown",
        callback.as_ref().unchecked_ref(),
    ).unwrap();
    callback.forget();
    //return
    Ok(())
}
