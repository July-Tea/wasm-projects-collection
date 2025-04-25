use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};
use std::cell::RefCell;
use std::rc::Rc;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

// 全局粒子系统配置
struct ParticleConfig {
    // 初始粒子数量
    initial_count: usize,
    // 点击时添加的粒子数量
    click_add_count: usize,
    // 粒子最小尺寸
    min_size: f64,
    // 粒子最大尺寸
    max_size: f64,
}

// 全局配置实例
static mut PARTICLE_CONFIG: ParticleConfig = ParticleConfig {
    initial_count: 400,      // 默认初始粒子数量
    click_add_count: 10,      // 默认点击添加粒子数量
    min_size: 3.0,          // 默认最小尺寸
    max_size: 6.0,          // 默认最大尺寸
};

// 设置全局配置的函数
#[wasm_bindgen]
pub fn set_particle_config(initial_count: usize, click_add_count: usize, min_size: f64, max_size: f64) {
    unsafe {
        PARTICLE_CONFIG = ParticleConfig {
            initial_count,
            click_add_count,
            min_size,
            max_size,
        };
    }
}

// 粒子系统状态
struct ParticleSystem {
    particles: Vec<Particle>,
    mouse_x: f64,
    mouse_y: f64,
    is_mouse_down: bool,
    canvas_width: f64,
    canvas_height: f64,
    rng: SmallRng,
}

// 粒子结构
struct Particle {
    x: f64,
    y: f64,
    z: f64,  // 增加深度信息
    size: f64,
    speed_x: f64,
    speed_y: f64,
    color: String,
    opacity: f64,
}

impl Particle {
    fn new(x: f64, y: f64, rng: &mut SmallRng, _canvas_width: f64, _canvas_height: f64) -> Self {
        let z = rng.gen_range(0.1..1.0);  // 深度信息
        
        // 随机选择一个颜色方案
        let color = match rng.gen_range(0..5) {
            0 => {
                // 蓝色系
                let blue = rng.gen_range(180..255);
                let green = rng.gen_range(120..200);
                let red = rng.gen_range(50..150);
                format!("rgb({}, {}, {})", red, green, blue)
            },
            1 => {
                // 紫色系
                let red = rng.gen_range(120..200);
                let blue = rng.gen_range(180..255);
                let green = rng.gen_range(50..150);
                format!("rgb({}, {}, {})", red, green, blue)
            },
            2 => {
                // 绿色系
                let green = rng.gen_range(180..255);
                let blue = rng.gen_range(100..200);
                let red = rng.gen_range(50..150);
                format!("rgb({}, {}, {})", red, green, blue)
            },
            3 => {
                // 橙黄色系
                let red = rng.gen_range(200..255);
                let green = rng.gen_range(150..230);
                let blue = rng.gen_range(50..120);
                format!("rgb({}, {}, {})", red, green, blue)
            },
            _ => {
                // 粉色系
                let red = rng.gen_range(200..255);
                let green = rng.gen_range(100..180);
                let blue = rng.gen_range(150..230);
                format!("rgb({}, {}, {})", red, green, blue)
            }
        };
        
        // 从全局配置获取大小范围
        let size_range = unsafe {
            PARTICLE_CONFIG.min_size..PARTICLE_CONFIG.max_size
        };
        
        Particle {
            x,
            y,
            z,
            size: rng.gen_range(size_range) * z, // 基于深度调整大小
            speed_x: rng.gen_range(-0.3..0.3),
            speed_y: rng.gen_range(-0.3..0.3),
            color,
            opacity: rng.gen_range(0.3..0.8) * z, // 基于深度调整透明度
        }
    }

    fn update(&mut self, mouse_x: f64, mouse_y: f64, is_mouse_down: bool, canvas_width: f64, canvas_height: f64) {
        // 添加布朗运动 - 随机小幅度改变速度
        let mut rng = rand::thread_rng();
        let brownian_x = rng.gen_range(-0.08..0.08);
        let brownian_y = rng.gen_range(-0.08..0.08);
        
        self.speed_x += brownian_x;
        self.speed_y += brownian_y;
        
        // 更新位置
        self.x += self.speed_x;
        self.y += self.speed_y;

        // 边界检查
        if self.x < 0.0 || self.x > canvas_width {
            self.speed_x *= -1.0;
        }
        if self.y < 0.0 || self.y > canvas_height {
            self.speed_y *= -1.0;
        }

        // 轻微受到鼠标位置影响
        let dx = mouse_x - self.x;
        let dy = mouse_y - self.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        if distance < 80.0 {
            let force = 0.8 / (distance + 1.0);
            if is_mouse_down {
                // 鼠标按下时，粒子会被吸引
                self.speed_x += dx * force * 0.01;
                self.speed_y += dy * force * 0.01;
            } else {
                // 鼠标悬停时，粒子会轻微排斥
                self.speed_x -= dx * force * 0.005;
                self.speed_y -= dy * force * 0.005;
            }
        }
        
        // 轻微阻尼，避免速度过大
        self.speed_x *= 0.99;
        self.speed_y *= 0.99;
        
        // 限制最大速度
        let max_speed = 1.5;
        let speed = (self.speed_x * self.speed_x + self.speed_y * self.speed_y).sqrt();
        if speed > max_speed {
            self.speed_x = (self.speed_x / speed) * max_speed;
            self.speed_y = (self.speed_y / speed) * max_speed;
        }
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.set_global_alpha(self.opacity);
        ctx.set_fill_style(&JsValue::from_str(&self.color));
        ctx.begin_path();
        ctx.arc(self.x, self.y, self.size, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        ctx.fill();
    }
}

impl ParticleSystem {
    fn new(width: f64, height: f64) -> Self {
        let mut rng = SmallRng::seed_from_u64(42);
        let mut particles = Vec::new();
        
        // 从全局配置获取初始粒子数量
        let initial_count = unsafe { PARTICLE_CONFIG.initial_count };
        
        // 创建粒子
        for _ in 0..initial_count {
            let x = rng.gen_range(0.0..width);
            let y = rng.gen_range(0.0..height);
            particles.push(Particle::new(x, y, &mut rng, width, height));
        }
        
        ParticleSystem {
            particles,
            mouse_x: width / 2.0,
            mouse_y: height / 2.0,
            is_mouse_down: false,
            canvas_width: width,
            canvas_height: height,
            rng,
        }
    }

    fn update(&mut self) {
        // 保存当前状态以避免多次借用
        let mouse_x = self.mouse_x;
        let mouse_y = self.mouse_y;
        let is_mouse_down = self.is_mouse_down;
        let canvas_width = self.canvas_width;
        let canvas_height = self.canvas_height;
        
        for particle in &mut self.particles {
            particle.update(mouse_x, mouse_y, is_mouse_down, canvas_width, canvas_height);
        }
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        // 清空画布
        ctx.set_global_alpha(1.0);
        ctx.set_fill_style(&JsValue::from_str("white"));
        ctx.fill_rect(0.0, 0.0, self.canvas_width, self.canvas_height);
        
        // 绘制粒子之间的连线
        self.draw_connections(ctx);
        
        // 绘制粒子
        for particle in &self.particles {
            particle.draw(ctx);
        }
    }
    
    fn draw_connections(&self, ctx: &CanvasRenderingContext2d) {
        for i in 0..self.particles.len() {
            for j in i+1..self.particles.len() {
                let p1 = &self.particles[i];
                let p2 = &self.particles[j];
                
                let dx = p1.x - p2.x;
                let dy = p1.y - p2.y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                // 只在一定距离内的粒子之间绘制连线
                if distance < 60.0 {
                    let opacity = (1.0 - distance / 60.0) * 0.2 * p1.z * p2.z;
                    ctx.set_global_alpha(opacity);
                    
                    // 使用粒子的颜色为连线着色
                    let color_mix = mix_colors(&p1.color, &p2.color);
                    ctx.set_stroke_style(&JsValue::from_str(&color_mix));
                    ctx.set_line_width(0.5);
                    
                    ctx.begin_path();
                    ctx.move_to(p1.x, p1.y);
                    ctx.line_to(p2.x, p2.y);
                    ctx.stroke();
                }
            }
        }
    }
    
    fn add_particles_at(&mut self, x: f64, y: f64, count: usize) {
        // 从全局配置获取点击添加的粒子数量
        let add_count = unsafe { PARTICLE_CONFIG.click_add_count };
        for _ in 0..add_count {
            self.particles.push(Particle::new(x, y, &mut self.rng, self.canvas_width, self.canvas_height));
        }
    }
}

#[wasm_bindgen]
pub struct ParticleAnimation {
    system: Rc<RefCell<ParticleSystem>>,
    ctx: CanvasRenderingContext2d,
    animation_id: Option<i32>,
}

#[wasm_bindgen]
impl ParticleAnimation {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<ParticleAnimation, JsValue> {
        // 获取window对象
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        // 获取canvas元素
        let canvas = document.get_element_by_id(canvas_id)
            .ok_or_else(|| JsValue::from_str("找不到指定ID的canvas元素"))?;
        
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| JsValue::from_str("所提供的元素不是canvas"))?;
        
        // 设置canvas大小
        canvas.set_width(440);
        canvas.set_height(330);
        
        // 获取2D上下文
        let ctx = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
        
        // 创建粒子系统
        let system = Rc::new(RefCell::new(ParticleSystem::new(440.0, 330.0)));
        
        // 设置鼠标移动事件
        let system_clone = system.clone();
        let mousemove_callback = Closure::wrap(Box::new(move |event: MouseEvent| {
            // 获取鼠标相对于canvas的坐标
            let x = event.offset_x() as f64;
            let y = event.offset_y() as f64;
            
            let mut system = system_clone.borrow_mut();
            system.mouse_x = x;
            system.mouse_y = y;
        }) as Box<dyn FnMut(MouseEvent)>);
        
        canvas.add_event_listener_with_callback(
            "mousemove",
            mousemove_callback.as_ref().unchecked_ref(),
        )?;
        mousemove_callback.forget();
        
        // 设置鼠标按下事件
        let system_clone = system.clone();
        let mousedown_callback = Closure::wrap(Box::new(move |event: MouseEvent| {
            // 获取鼠标相对于canvas的坐标
            let x = event.offset_x() as f64;
            let y = event.offset_y() as f64;
            
            let mut system = system_clone.borrow_mut();
            system.is_mouse_down = true;
            system.mouse_x = x;
            system.mouse_y = y;
            
            // 鼠标点击时，在点击位置添加更多粒子
            system.add_particles_at(x, y, 5);
        }) as Box<dyn FnMut(MouseEvent)>);
        
        canvas.add_event_listener_with_callback(
            "mousedown",
            mousedown_callback.as_ref().unchecked_ref(),
        )?;
        mousedown_callback.forget();
        
        // 设置鼠标松开事件
        let system_clone = system.clone();
        let mouseup_callback = Closure::wrap(Box::new(move |_event: MouseEvent| {
            let mut system = system_clone.borrow_mut();
            system.is_mouse_down = false;
        }) as Box<dyn FnMut(MouseEvent)>);
        
        canvas.add_event_listener_with_callback(
            "mouseup",
            mouseup_callback.as_ref().unchecked_ref(),
        )?;
        mouseup_callback.forget();
        
        Ok(ParticleAnimation {
            system,
            ctx,
            animation_id: None,
        })
    }
    
    #[wasm_bindgen]
    pub fn start(&mut self) -> Result<(), JsValue> {
        let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
        let g = f.clone();
        
        let system = self.system.clone();
        let ctx = self.ctx.clone();
        
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let mut system = system.borrow_mut();
            system.update();
            system.draw(&ctx);
            
            // 请求下一帧动画
            web_sys::window()
                .unwrap()
                .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .unwrap();
        }) as Box<dyn FnMut()>));
        
        let window = web_sys::window().unwrap();
        let id = window.request_animation_frame(
            g.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
        )?;
        
        self.animation_id = Some(id);
        Ok(())
    }
    
    #[wasm_bindgen]
    pub fn stop(&mut self) {
        if let Some(id) = self.animation_id {
            let window = web_sys::window().unwrap();
            window.cancel_animation_frame(id).unwrap();
            self.animation_id = None;
        }
    }
}

// 混合两种颜色的辅助函数
fn mix_colors(color1: &str, color2: &str) -> String {
    // 从RGB字符串中提取RGB值
    let c1 = parse_rgb(color1);
    let c2 = parse_rgb(color2);
    
    if let (Some((r1, g1, b1)), Some((r2, g2, b2))) = (c1, c2) {
        // 混合两种颜色（简单平均）
        let r = (r1 + r2) / 2;
        let g = (g1 + g2) / 2;
        let b = (b1 + b2) / 2;
        
        format!("rgb({}, {}, {})", r, g, b)
    } else {
        // 如果解析失败，返回默认颜色
        "rgba(150, 150, 220, 0.5)".to_string()
    }
}

// 从RGB字符串中解析RGB值的辅助函数
fn parse_rgb(color: &str) -> Option<(u8, u8, u8)> {
    // 匹配格式为"rgb(r, g, b)"的字符串
    let color = color.trim();
    if !color.starts_with("rgb(") || !color.ends_with(")") {
        return None;
    }
    
    // 提取RGB部分
    let rgb_part = &color[4..color.len()-1];
    let parts: Vec<&str> = rgb_part.split(',').collect();
    
    if parts.len() != 3 {
        return None;
    }
    
    // 解析RGB值
    let r = parts[0].trim().parse::<u8>().ok()?;
    let g = parts[1].trim().parse::<u8>().ok()?;
    let b = parts[2].trim().parse::<u8>().ok()?;
    
    Some((r, g, b))
}
