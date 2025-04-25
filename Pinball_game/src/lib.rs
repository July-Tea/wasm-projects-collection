use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, Element};
use rand::Rng;

// 游戏常量
const CANVAS_WIDTH: f64 = 440.0; // 新的宽度
const CANVAS_HEIGHT: f64 = 330.0; // 新的高度
const BALL_RADIUS: f64 = 8.0; // 缩小球的尺寸以适应新画布
const PADDLE_WIDTH: f64 = 80.0; // 缩小挡板宽度
const PADDLE_HEIGHT: f64 = 12.0; // 缩小挡板高度
const PADDLE_Y: f64 = CANVAS_HEIGHT - 30.0;
const BRICK_WIDTH: f64 = 50.0; // 调整砖块大小以适应新的宽度
const BRICK_HEIGHT: f64 = 20.0;
const BRICK_ROWS: usize = 5;
const BRICK_COLS: usize = 7; // 调整列数以适应新的宽度
const BRICK_TOP_OFFSET: f64 = 40.0;
const BRICK_PADDING: f64 = 5.0;

// 游戏状态
struct GameState {
    ball_x: f64,
    ball_y: f64,
    ball_dx: f64,
    ball_dy: f64,
    paddle_x: f64,
    bricks: Vec<Brick>,
    game_over: bool,
    game_started: bool,
    last_time: f64,  // 添加上一帧的时间戳
}

struct Brick {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    active: bool,
}

// 初始化游戏状态
impl GameState {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let bricks = create_bricks();
        
        // 初始球的速度和方向，但不立即使用
        let angle = rng.gen_range((-60.0_f64).to_radians()..(60.0_f64).to_radians());
        let speed = 300.0; // 调整基础速度为更大的值，因为我们会乘以delta time(秒)
        
        // 挡板初始位置
        let paddle_x = (CANVAS_WIDTH - PADDLE_WIDTH) / 2.0;
        
        // 获取当前时间
        let win = web_sys::window().unwrap();
        let now = win.performance().unwrap().now();
        
        GameState {
            // 球的初始位置在挡板上方中央
            ball_x: paddle_x + PADDLE_WIDTH / 2.0,
            ball_y: PADDLE_Y - BALL_RADIUS,
            ball_dx: speed * angle.sin(), // 使用sin来计算水平速度
            ball_dy: -speed * angle.cos(), // 使用cos来确保主要是垂直运动
            paddle_x,
            bricks,
            game_over: false,
            game_started: false,
            last_time: now,
        }
    }
    
    // 重置游戏状态
    fn reset(&mut self) {
        let mut rng = rand::thread_rng();
        let angle = rng.gen_range((-60.0_f64).to_radians()..(60.0_f64).to_radians());
        let speed = 300.0; // 调整基础速度
        
        // 球位于挡板上方
        self.ball_x = self.paddle_x + PADDLE_WIDTH / 2.0;
        self.ball_y = PADDLE_Y - BALL_RADIUS;
        self.ball_dx = speed * angle.sin(); // 使用sin来计算水平速度
        self.ball_dy = -speed * angle.cos(); // 使用cos来确保主要是垂直运动
        self.game_over = false;
        self.game_started = false;
        self.bricks = create_bricks();
        
        // 更新时间戳
        let win = web_sys::window().unwrap();
        self.last_time = win.performance().unwrap().now();
    }
    
    // 更新游戏状态
    fn update(&mut self) {
        if self.game_over {
            return;
        }

        // 如果游戏尚未开始，球跟随挡板移动
        if !self.game_started {
            self.ball_x = self.paddle_x + PADDLE_WIDTH / 2.0;
            self.ball_y = PADDLE_Y - BALL_RADIUS;
            
            // 即使在游戏未开始时也更新上一帧时间
            let win = web_sys::window().unwrap();
            self.last_time = win.performance().unwrap().now();
            return;
        }

        // 计算时间差（delta time）
        let win = web_sys::window().unwrap();
        let now = win.performance().unwrap().now();
        let delta_time = (now - self.last_time) / 1000.0; // 转换为秒
        self.last_time = now;
        
        // 限制delta_time，防止在切换标签页等情况下delta_time过大导致球穿模
        let delta_time = f64::min(delta_time, 0.1);

        // 更新球的位置，使用delta_time使运动与帧率无关
        self.ball_x += self.ball_dx * delta_time;
        self.ball_y += self.ball_dy * delta_time;
        
        // 检测边界碰撞
        if self.ball_x - BALL_RADIUS <= 0.0 || self.ball_x + BALL_RADIUS >= CANVAS_WIDTH {
            self.ball_dx = -self.ball_dx;
        }
        
        if self.ball_y - BALL_RADIUS <= 0.0 {
            self.ball_dy = -self.ball_dy;
        }
        
        // 检测游戏结束
        if self.ball_y + BALL_RADIUS >= CANVAS_HEIGHT {
            self.game_over = true;
        }
        
        // 检测挡板碰撞
        if self.ball_y + BALL_RADIUS >= PADDLE_Y && 
           self.ball_y + BALL_RADIUS <= PADDLE_Y + PADDLE_HEIGHT &&
           self.ball_x >= self.paddle_x && 
           self.ball_x <= self.paddle_x + PADDLE_WIDTH 
        {
            // 根据击中挡板的位置计算反弹角度
            let relative_intersect_x = (self.paddle_x + PADDLE_WIDTH / 2.0) - self.ball_x;
            let normalized_intersect_x = relative_intersect_x / (PADDLE_WIDTH / 2.0);
            let bounce_angle = normalized_intersect_x * 60.0_f64.to_radians();
            
            // 保持相同的速度大小，只改变方向
            let current_speed = (self.ball_dx.powi(2) + self.ball_dy.powi(2)).sqrt();
            // 根据当前速度重新计算dx和dy分量
            self.ball_dx = current_speed * bounce_angle.sin();
            self.ball_dy = -current_speed * bounce_angle.cos().abs(); // 向上反弹
        }
        
        // 检测砖块碰撞
        for brick in &mut self.bricks {
            if !brick.active {
                continue;
            }
            
            // 使用更精确的碰撞检测
            // 计算球心与砖块边缘的最近点
            let closest_x = f64::max(brick.x, f64::min(self.ball_x, brick.x + brick.width));
            let closest_y = f64::max(brick.y, f64::min(self.ball_y, brick.y + brick.height));
            
            // 计算球心与最近点之间的距离
            let distance_x = self.ball_x - closest_x;
            let distance_y = self.ball_y - closest_y;
            let distance_squared = distance_x * distance_x + distance_y * distance_y;
            
            // 如果距离小于球半径，则发生碰撞
            if distance_squared < BALL_RADIUS * BALL_RADIUS {
                brick.active = false;
                
                // 确定应该反弹的方向
                // 从哪个方向碰撞更多，就从那个方向反弹
                if distance_x.abs() > distance_y.abs() {
                    self.ball_dx = -self.ball_dx;
                } else {
                    self.ball_dy = -self.ball_dy;
                }
                
                break; // 每次只处理一个碰撞
            }
        }
        
        // 检查是否清空所有砖块
        if !self.bricks.iter().any(|brick| brick.active) {
            self.reset();
        }
    }
    
    // 渲染游戏
    #[allow(deprecated)]
    fn render(&self, ctx: &CanvasRenderingContext2d) {
        // 清空画布
        ctx.clear_rect(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT);
        
        // 绘制球
        ctx.begin_path();
        ctx.arc(self.ball_x, self.ball_y, BALL_RADIUS, 0.0, f64::consts::PI * 2.0).unwrap();
        ctx.set_fill_style(&JsValue::from_str("black"));
        ctx.fill();
        ctx.close_path();
        
        // 绘制挡板
        ctx.begin_path();
        ctx.rect(self.paddle_x, PADDLE_Y, PADDLE_WIDTH, PADDLE_HEIGHT);
        ctx.set_fill_style(&JsValue::from_str("black"));
        ctx.fill();
        ctx.close_path();
        
        // 绘制砖块
        for brick in &self.bricks {
            if brick.active {
                ctx.begin_path();
                ctx.rect(brick.x, brick.y, brick.width, brick.height);
                ctx.set_fill_style(&JsValue::from_str("black"));
                ctx.fill();
                ctx.set_stroke_style(&JsValue::from_str("white"));
                ctx.stroke();
                ctx.close_path();
            }
        }
        
        // 显示游戏提示
        if self.game_over {
            // 设置文本对齐方式为居中
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            
            // 游戏结束文字 - 居中并向下移动
            ctx.set_font("36px Arial");
            ctx.set_fill_style(&JsValue::from_str("black"));
            ctx.fill_text("游戏结束", CANVAS_WIDTH / 2.0, CANVAS_HEIGHT / 2.0 + 20.0).unwrap();
            
            // 重新开始提示文字 - 居中并向下移动
            ctx.set_font("20px Arial");
            ctx.fill_text("点击屏幕重新开始", CANVAS_WIDTH / 2.0, CANVAS_HEIGHT / 2.0 + 70.0).unwrap();
        } else if !self.game_started {
            // 游戏未开始时显示提示 - 居中并向下移动
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            ctx.set_font("20px Arial");
            ctx.set_fill_style(&JsValue::from_str("black"));
            ctx.fill_text("点击屏幕开始游戏", CANVAS_WIDTH / 2.0, CANVAS_HEIGHT / 2.0 + 50.0).unwrap();
        }
    }
}

// 创建砖块
fn create_bricks() -> Vec<Brick> {
    let mut bricks = Vec::new();
    let mut rng = rand::thread_rng();
    
    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
            // 随机确定是否创建砖块
            if rng.gen_bool(0.8) { // 80%概率创建砖块
                let brick = Brick {
                    x: col as f64 * (BRICK_WIDTH + BRICK_PADDING) + BRICK_PADDING,
                    y: row as f64 * (BRICK_HEIGHT + BRICK_PADDING) + BRICK_TOP_OFFSET,
                    width: BRICK_WIDTH,
                    height: BRICK_HEIGHT,
                    active: true,
                };
                bricks.push(brick);
            }
        }
    }
    
    bricks
}

// 初始化游戏
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // 设置panic hook
    console_error_panic_hook::set_once();
    
    // 获取Canvas元素和上下文
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
    
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    
    // 创建游戏状态
    let game_state = Rc::new(RefCell::new(GameState::new()));
    
    // 处理鼠标移动事件
    {
        let game_state = game_state.clone();
        let canvas_element = canvas.clone().dyn_into::<Element>().unwrap();
        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            let rect = canvas_element.get_bounding_client_rect();
            
            // 计算缩放比例 - canvas实际渲染尺寸与游戏内部逻辑尺寸的比例
            let scale_x = rect.width() / CANVAS_WIDTH;
            
            // 计算鼠标在canvas中的相对位置，并应用缩放比例
            let mouse_x = (event.client_x() as f64 - rect.left()) / scale_x;
            
            // 更新挡板位置，确保不超出边界
            let mut state = game_state.borrow_mut();
            state.paddle_x = mouse_x - PADDLE_WIDTH / 2.0;
            
            if state.paddle_x < 0.0 {
                state.paddle_x = 0.0;
            } else if state.paddle_x + PADDLE_WIDTH > CANVAS_WIDTH {
                state.paddle_x = CANVAS_WIDTH - PADDLE_WIDTH;
            }
        }) as Box<dyn FnMut(_)>);
        
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    
    // 处理点击事件（开始游戏或重新开始游戏）
    {
        let game_state = game_state.clone();
        let canvas_element = canvas.clone().dyn_into::<Element>().unwrap();
        let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
            let rect = canvas_element.get_bounding_client_rect();
            
            // 计算缩放比例
            let scale_x = rect.width() / CANVAS_WIDTH;
            let scale_y = rect.height() / CANVAS_HEIGHT;
            
            // 计算点击在canvas中的相对位置
            let click_x = (event.client_x() as f64 - rect.left()) / scale_x;
            let click_y = (event.client_y() as f64 - rect.top()) / scale_y;
            
            // 确保点击在canvas内部
            if click_x >= 0.0 && click_x <= CANVAS_WIDTH && click_y >= 0.0 && click_y <= CANVAS_HEIGHT {
                let mut state = game_state.borrow_mut();
                if state.game_over {
                    *state = GameState::new();
                } else if !state.game_started {
                    // 如果游戏尚未开始，点击时给球一个随机方向
                    let mut rng = rand::thread_rng();
                    let angle = rng.gen_range((-60.0_f64).to_radians()..(60.0_f64).to_radians());
                    let speed = 300.0; // 调整为更大的基础速度，因为我们会乘以delta time
                    
                    state.ball_dx = speed * angle.sin(); // 使用sin来计算水平速度
                    state.ball_dy = -speed * angle.cos(); // 使用cos来确保主要是垂直运动
                    state.game_started = true;
                    
                    // 重置时间戳，确保第一帧的delta time合理
                    let win = web_sys::window().unwrap();
                    state.last_time = win.performance().unwrap().now();
                }
            }
        }) as Box<dyn FnMut(_)>);
        
        canvas.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    
    // 设置游戏循环
    {
        let game_state = game_state.clone();
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            // 更新游戏状态
            game_state.borrow_mut().update();
            
            // 渲染游戏
            game_state.borrow().render(&context);
            
            // 请求下一帧
            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        
        request_animation_frame(g.borrow().as_ref().unwrap());
    }
    
    Ok(())
}

// 辅助函数：请求动画帧
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

// 设置panic hook以便于调试
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// 添加console_error_panic_hook依赖
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

// console_error_panic_hook模块
mod console_error_panic_hook {
    use std::panic;
    use wasm_bindgen::prelude::*;
    
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        fn error(msg: &str);
    }
    
    pub fn set_once() {
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            panic::set_hook(Box::new(|info| {
                error(&format!("panic: {:?}", info));
            }));
        });
    }
}
