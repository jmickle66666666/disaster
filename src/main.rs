use macroquad::prelude::*;
use deno_core::*;
use core::cell::RefCell;
use std::rc::Rc;
use std::io::Read;
use deno_core::error::AnyError;
// use std::io::Error;
// use std::io::ErrorKind;
// use std::sync::Mutex;

mod draw;

fn log(_: &mut OpState, val: serde_json::value::Value, _: &mut [ZeroCopyBuf]) -> Result<serde_json::value::Value, error::AnyError> {
    let message = val.as_str().unwrap();
    println!("{}", message);
    return Ok(serde_json::value::Value::Null);
}

async fn draw_texture_part (state: Rc<RefCell<OpState>>, val: serde_json::value::Value, _: BufVec) -> Result<serde_json::value::Value, AnyError> {
    let draw_ref: u32 = val.get("draw_ref").unwrap().as_u64().unwrap() as u32;
    let draw_state = state.borrow_mut().resource_table.get::<draw::DrawContainer>(draw_ref).unwrap();

    let x: i32 = val.get("x").unwrap().as_i64().unwrap() as i32;
    let y: i32 = val.get("y").unwrap().as_i64().unwrap() as i32;
    let texture_name: &str = val.get("textureName").unwrap().as_str().unwrap();
    let x_start: u32 = val.get("xStart").unwrap().as_u64().unwrap() as u32;
    let y_start: u32 = val.get("yStart").unwrap().as_u64().unwrap() as u32;
    let width: u32 = val.get("width").unwrap().as_u64().unwrap() as u32;
    let height: u32 = val.get("height").unwrap().as_u64().unwrap() as u32;

    let mut draw = draw_state.refcell.borrow_mut();

    // we're trying to draw a texture called `texture_name`
    // so first, load the image if we haven't loaded it already
    if !draw.textures.contains_key(texture_name) {
        draw.textures.insert(
            texture_name.to_string(),
            load_image(texture_name).await
        );
    }

    // now we get it from the loaded images, so we can draw it
    let image = draw.textures.get(texture_name).unwrap();
    
    draw.draw_texture_part(
        x, y, 
        image,
        x_start, y_start,
        width, height
    );

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
    let draw = draw::init_canvas(320, 240); 
    let draw_refcell = draw::DrawContainer {
        refcell : RefCell::<draw::Draw>::new(draw),
    };
    
    

    // let mutex = Mutex::new(draw);

    // let draw_ref = js_runtime.op_state().borrow().resource_table.add(draw);
    // let draw_ref = js_runtime.op_state().borrow_mut().resource_table.add(draw);

    // // this is to create a function i can call from js, to draw to the software renderer
    js_runtime.register_op("draw_clear", json_op_sync(
        |state: &mut OpState, val: serde_json::value::Value, _: &mut [ZeroCopyBuf]| {
            let draw_ref: u32 = val.get("draw_ref").unwrap().as_u64().unwrap() as u32;
            let draw_state = state.resource_table.get::<draw::DrawContainer>(draw_ref).unwrap();
            
            let mut reference = draw_state.refcell.borrow_mut();
            reference.draw_clear();

            return Ok(serde_json::value::Value::Null);
        }
    ));

    js_runtime.register_op("draw_texture_part", json_op_async(
        
    ));

    // log things from js to the console
    js_runtime.register_op("log", json_op_sync(log));

    js_runtime.execute("src/api.js", &readfile("src/api.js").unwrap()).unwrap();

    let draw_ref = js_runtime.op_state().borrow_mut().resource_table.add(draw_refcell);
    
    let set_ref_script = format!(
        "Draw.ref = {}", 
        draw_ref
    );

    js_runtime.execute(
        "refset", 
        &set_ref_script  
    ).unwrap();

    for s in scripts_to_load
    {
        let script = basepath.to_string() + &s + ".js";
        // println!("{}", script);
        js_runtime.execute(&script, &readfile(&script).unwrap()).unwrap();
    }

    // js_runtime.execute("_", "coolFunc(\"hello from rust\");").unwrap();

    js_runtime.execute("_", &format!("screenSize = {{x:{}, y:{}}};", 320, 240)).unwrap();
    
    // let ship_tex = load_image("res/ship1.png").await;

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

        // let x0 = 160 + ((6.28 / 11. + t).sin()*160.) as i32;
        // let x1 = 160 + ((6.28 / 5.  + t).sin()*160.) as i32;
        // let x2 = 160 + ((6.28 / 17. + t).sin()*160.) as i32;
        // let y0 = 120 + ((6.28 / 23. + t).sin()*120.) as i32;
        // let y1 = 120 + ((6.28 / 31. + t).sin()*120.) as i32;
        // let y2 = 120 + ((6.28 / 13. + t).sin()*120.) as i32;

        // draw.draw_line( x0, y0, x1, y1, ORANGE );
        // draw.draw_line( x1, y1, x2, y2, ORANGE );
        // draw.draw_line( x2, y2, x0, y0, ORANGE );
        
        // draw.draw_texture(160, 120, &ship_tex);
        
        // draw.draw_screen();


        let d = js_runtime.op_state().borrow_mut().resource_table.get::<draw::DrawContainer>(draw_ref).unwrap();

        let mut draw_reference = d.refcell.borrow_mut();
        draw_reference.draw_screen();        

        draw_text("disaster engine", 100., 100., 16., ORANGE);
        next_frame().await
    }
}
