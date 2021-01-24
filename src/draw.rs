use macroquad::prelude::*;
use deno_core::Resource;

pub struct Draw {
    pub canvas:Image,
    pub screen:Texture2D,
    pub width:u32,
    pub height:u32,
    // pub colors:[Color; 320*240],
}

impl Resource for Draw {

}

pub fn init_canvas(width:u16, height:u16) -> Draw {
    let canvas = Image::gen_image_color(width, height, Color::new(0.1, 0.1, 0.1, 1.0));
    let screen = load_texture_from_image(&canvas);
    set_texture_filter(screen, FilterMode::Nearest);
    return Draw{
        canvas : canvas,
        screen : screen,
        width: width as u32,
        height: height as u32
    };
}

// allow dead code until the js api is implemented
#[allow(dead_code)]
impl Draw {
    
    pub fn draw_pixel(&mut self, x:u32, y:u32, color:Color)
    {
        self.canvas.set_pixel(x, y, color);
    }

    pub fn draw_clear(&mut self)
    {
        self.draw_rect(0, 0, self.width, self.height, BLACK)
    }

    pub fn draw_clear_color(&mut self, color:Color)
    {
        self.draw_rect(0, 0, self.width, self.height, color)
    }

    pub fn draw_rect(&mut self, x:u32, y:u32, w:u32, h:u32, color:Color)
    {
        for v in y..y+h {
            for u in x..x+w {
                self.canvas.set_pixel(u, v, color);
            }
        }
    }

    pub fn draw_texture(&mut self, x0:i32, y0:i32, texture: &Image)
    {
        for u in 0..texture.width as i32 {

            // safety
            if u + x0 < 0 { continue; }
            if u + x0 >= self.canvas.width as i32 { break; }

            for v in 0..texture.height as i32 {

                if v + y0 < 0 { continue; }
                if v + y0 >= self.canvas.height as i32 { break; }

                let c = texture.get_pixel(u as u32, v as u32);
                if c.a > 0.5 {
                    self.canvas.set_pixel((x0 + u) as u32, (y0 + v) as u32, c);
                }
            }
        }
    }

    pub fn draw_texture_part(&mut self, x0:i32, y0:i32, texture: &Image, x_start:u32, y_start:u32, w:u32, h:u32)
    {
        for u in x_start..x_start+w {

            // safety
            if u as i32 + x0 < 0 { continue; }
            if u as i32 + x0 >= self.canvas.width as i32 { break; }

            for v in y_start..y_start+h {

                if v as i32 + y0 < 0 { continue; }
                if v as i32 + y0 >= self.canvas.height as i32 { break; }

                let c = texture.get_pixel(u as u32, v as u32);
                if c.a > 0.5 {
                    self.canvas.set_pixel(x0 as u32 + u, y0 as u32 + v, c);
                }
            }
        }
    }

    pub fn draw_line(&mut self, mut x0:i32, mut y0:i32, x1:i32, y1:i32, color:Color)
    {
        let dx = (x1 - x0).abs();
        let sx;
        if x0 < x1 { sx = 1; } else { sx = -1; }

        let dy = (y1 - y0).abs();
        let sy;
        if y0 < y1 { sy = 1; } else { sy = -1; }

        let mut err;
        if dx > dy { err = dx / 2; } else { err = -dy / 2; }
        
        let mut e2:i32;

        loop 
        {
            if y0 >= 0 && x0 >= 0 && x0 < self.width as i32 && y0 < self.height as i32 {
                self.canvas.set_pixel(x0 as u32, y0 as u32, color);
            }

            if x0 == x1 && y0 == y1 { break; }

            e2 = err;
            if e2 > -dx { err -= dy; x0 += sx; }
            if e2 < dy { err += dx; y0 += sy; }
        }

    }

    pub fn draw_screen(&self) {
        update_texture(self.screen, &self.canvas);

        let mut scrw = screen_width() as u32 / 320;
        let mut scrh = screen_height() as u32 / 240;
        let min:u32;
        if scrw < scrh {
            min = scrw;
        } else {
            min = scrh;
        }
        scrw = 320 * min;
        scrh = 240 * min;


        draw_texture_ex(
            self.screen,
            (screen_width()/2. - (scrw/2)as f32).floor(),
            (screen_height()/2. - (scrh/2)as f32).floor(),
            WHITE,
            DrawTextureParams {
                dest_size : Some(Vec2 {x:(scrw as f32).floor(), y:(scrh as f32).floor()}),
                ..Default::default()
            }
        );
    }
}