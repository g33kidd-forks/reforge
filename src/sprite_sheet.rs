use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use num::Float;

use graphics::{Context, ImageSize};
use opengl_graphics::{GlGraphics, Texture};

use asset_store::SpriteInfo;
use vec::{Vec2, Vec2f};

pub enum SpriteAnimation {
    PlayOnce(f64, f64, u32, u32),
    Loop(f64, f64, u32, u32, f64),
    Stay(f64, f64, u32),
}

pub struct SpriteSheet {
    // Texture
    texture: Rc<Texture>,
    
    // Sprite sheet info
    columns: u8,
    frame_width: u32,
    frame_height: u32,
    
    // Sprite sheet state
    current_frame: u32,
    
    // Time stuff
    animations: VecDeque<SpriteAnimation>,
    
    // Animation name -> frames map
    anim_map: HashMap<String, (u32, u32)>,
    
    // Whether or not to center the texture
    pub center: Vec2f,
}

impl SpriteSheet {
    pub fn new(sprite_info: &SpriteInfo) -> SpriteSheet {
        let (texture_width, texture_height) = sprite_info.texture.get_size();
        
        let columns = sprite_info.columns;
        let rows = sprite_info.rows;
        
        SpriteSheet {
            texture: sprite_info.texture.clone(),
            columns: columns,
            frame_width: texture_width/(columns as u32),
            frame_height: texture_height/(rows as u32),
            current_frame: 0,
            animations: VecDeque::new(),
            anim_map: sprite_info.animations.clone(),
            center: Vec2::new(0.0, 0.0),
        }
    }
    
    pub fn get_frame_size(&self) -> (u32, u32) {
        (self.frame_width, self.frame_height)
    }
    
    pub fn center(&mut self) {
        self.center = Vec2::new(self.frame_width as f64 / 2.0, self.frame_height as f64 / 2.0);
    }
    
    pub fn add_animation(&mut self, animation: SpriteAnimation) {
        self.animations.push_back(animation);
    }
    
    pub fn add_named_once(&mut self, name: &String, start: f64, end: f64) {
        let ref frame_interval = self.anim_map[name];
        self.animations.push_back(SpriteAnimation::PlayOnce(start, end, frame_interval.0, frame_interval.1));
    }
    
    pub fn add_named_loop(&mut self, name: &String, start: f64, end: f64, interval: f64) {
        let ref frame_interval = self.anim_map[name];
        self.animations.push_back(SpriteAnimation::Loop(start, end, frame_interval.0, frame_interval.1, interval));
    }
    
    pub fn add_named_stay(&mut self, name: &String, start: f64, end: f64) {
        let ref frame_interval = self.anim_map[name];
        if frame_interval.0 != frame_interval.1 {
            println!("WARNING: stay animation {} doesn't have equal frame start and end values", name);
        }
        self.animations.push_back(SpriteAnimation::Stay(start, end, frame_interval.0));
    }
    
    pub fn draw(&mut self, context: &Context, gl: &mut GlGraphics, x: f64, y: f64, rotation: f64, time: f64) {
        let mut anim_done = false;
        match self.animations.front() {
            Some(animation) =>
                match *animation {
                    SpriteAnimation::PlayOnce(start_time, end_time, start_frame, end_frame) => {
                        if time >= start_time {
                            if time <= end_time {
                                let frame = (((time-start_time)/(end_time-start_time) * ((end_frame - start_frame) as f64)).floor() as u32) + start_frame;
                                self.current_frame = frame;
                            } else {
                                anim_done = true;
                            }
                            self.draw_current_frame(context, gl, x, y, rotation);
                        }
                    },
                    SpriteAnimation::Loop(start_time, end_time, start_frame, end_frame, interval) => {
                        if time >= start_time {
                            if time <= end_time {
                                let mut frame = ((time-start_time) / interval).floor() as u32;
                                frame = frame % (end_frame - start_frame + 1);
                                frame += start_frame;
                                self.current_frame = frame;
                            } else {
                                anim_done = true;
                            }
                            self.draw_current_frame(context, gl, x, y, rotation);
                        }
                    },
                    SpriteAnimation::Stay(start_time, end_time, frame) => {
                        if time >= start_time {
                            if time <= end_time {
                                self.current_frame = frame;
                            } else {
                                anim_done = true;
                            }
                            self.draw_current_frame(context, gl, x, y, rotation);
                        }
                    },
                },
            None => {}
        }
        
        if anim_done {
            self.animations.pop_front();
        }
    }
    
    fn draw_current_frame(&self, context: &Context, gl: &mut GlGraphics, x: f64, y: f64, rotation: f64) {
        use graphics::*;
    
        let source_x = ((self.current_frame % (self.columns as u32)) as f64) * (self.frame_width as f64);
        let source_y = ((self.current_frame / (self.columns as u32)) as f64) * (self.frame_height as f64);
        
        let source_end_x = source_x + (self.frame_width as f64);
        let source_end_y = source_y + (self.frame_height as f64);
        
        let mut context = context.trans(x, y)
                                 .rot_rad(rotation)
                                 .trans(-self.center.x, -self.center.y);
        
        Image::new()
            .src_rect([source_x as i32, source_y as i32, self.frame_width as i32, self.frame_height as i32])
            .draw(&*self.texture, &context.draw_state, context.transform, gl);
    }
    
    pub fn set_frame(&mut self, frame: u32) {
        self.current_frame = frame;
    }
}
