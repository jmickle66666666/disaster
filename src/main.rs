use macroquad::prelude::*;
use deno_core::*;
// use core::cell::RefCell;
// use std::rc::Rc;
use std::io::Read;
// use std::sync::Mutex;

mod draw;

fn log(_: &mut OpState, val: serde_json::value::Value, _: &mut [ZeroCopyBuf]) -> Result<serde_json::value::Value, error::AnyError> {
    let message = val.as_str().unwrap();
    println!("{}", message);
    return Ok(serde_json::value::Value::Null);
}

fn readfile(path:&str) -> Result<String, String> {
    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(error) => panic!(error),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => return Ok(contents),
        Err(error) => panic!(error),
    };
}

#[macroquad::main("Test")]
async fn main() {
    

    let mut scripts_to_load:Vec<String> = Vec::new();
    let mut basepath = "\\base\\".to_string();

    // read config
    let config = readfile("disaster.cfg").unwrap().to_string();
    for l in config.lines() {
        let tokens = l.split(" ").collect::<Vec<&str>>();
        if tokens[0] == "dataPath" {
            // read scripts
            basepath = tokens[1].to_string() + &"\\base\\";
            let scripts_path = tokens[1].to_string() + &"\\base\\" + "scripts.cfg";
            
            match readfile(&scripts_path) {
                Ok(scripts_file) => {
                    let scripts = scripts_file.lines().map(|x| x.to_string());
                    for s in scripts {
                        scripts_to_load.push(s);
                    }
                },
                Err(error) => panic!(error),
            };
        }
    }

    let mut js_runtime = JsRuntime::new(Default::default());

    // this is the canvas for a software renderer
    let mut draw = draw::init_canvas(320, 240); 

    // let mutex = Mutex::new(draw);

    // let draw_ref = js_runtime.op_state().borrow().resource_table.add(draw);

    // // this is to create a function i can call from js, to draw to the software renderer
    // js_runtime.register_op("draw_clear", json_op_sync(
    //     |state: &mut OpState, val: serde_json::value::Value, _: &mut [ZeroCopyBuf]| {
            
    //         let d:draw::Draw = std::rc::Rc::get_mut::<draw::Draw>(state.resource_table.get::<draw::Draw>(draw_ref).unwrap()).unwrap();
    //         draw.draw_clear(); // it doesn't like that its accessing the outer variable
    //         return Ok(serde_json::value::Value::Null);
    //     }
    // ));

    // log things from js to the console
    js_runtime.register_op("log", json_op_sync(log));

    js_runtime.execute("src/api.js", &readfile("src/api.js").unwrap()).unwrap();

    for s in scripts_to_load
    {
        let script = basepath.to_string() + &s + ".js";
        // println!("{}", script);
        js_runtime.execute(&script, &readfile(&script).unwrap()).unwrap();
    }

    // js_runtime.execute("_", "coolFunc(\"hello from rust\");").unwrap();

    js_runtime.execute("_", &format!("screenSize = {{x:{}, y:{}}};", 320, 240)).unwrap();
    
    let ship_tex = load_image("res/ship1.png").await;

    let mut pause_js_execution = false;

    loop {
        let t = get_time();

        if !pause_js_execution {
            match js_runtime.execute("_", &format!("dt = {}; update();", t)) {
                Err(error) => {
                    pause_js_execution = true;
                    println!("{}", error);
                },
                Ok(_) => pause_js_execution = false,
            };
        }

        clear_background(ORANGE);
        // draw.draw_clear_color(BLACK);

        let x0 = 160 + ((6.28 / 11. + t).sin()*160.) as i32;
        let x1 = 160 + ((6.28 / 5.  + t).sin()*160.) as i32;
        let x2 = 160 + ((6.28 / 17. + t).sin()*160.) as i32;
        let y0 = 120 + ((6.28 / 23. + t).sin()*120.) as i32;
        let y1 = 120 + ((6.28 / 31. + t).sin()*120.) as i32;
        let y2 = 120 + ((6.28 / 13. + t).sin()*120.) as i32;

        draw.draw_line( x0, y0, x1, y1, ORANGE );
        draw.draw_line( x1, y1, x2, y2, ORANGE );
        draw.draw_line( x2, y2, x0, y0, ORANGE );
        
        draw.draw_texture(160, 120, &ship_tex);
        
        draw.draw_screen();
        draw_text("disaster engine", 100., 100., 16., ORANGE);
        next_frame().await
    }
}
