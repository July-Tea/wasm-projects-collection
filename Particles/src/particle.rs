use nalgebra::Vector2;
use rand::Rng;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

// 粒子结构体
pub struct Particle {
    pub position: Vector2<f64>,    // 位置
    pub velocity: Vector2<f64>,    // 速度
    pub acceleration: Vector2<f64>, // 加速度
    pub radius: f64,               // 半径
    pub color: String,             // 颜色
    pub alpha: f64,                // 透明度
    pub life: f64,                 // 生命值
}

impl Particle {
    // 创建新粒子
    pub fn new(x: f64, y: f64, is_burst: bool) -> Self {
        let mut rng = rand::thread_rng();
        
        // 生成随机速度
        let vx = if is_burst {
            (rng.gen::<f64>() - 0.5) * 4.0  // 爆发粒子速度更快
        } else {
            (rng.gen::<f64>() - 0.5) * 0.5
        };
        
        let vy = if is_burst {
            (rng.gen::<f64>() - 0.5) * 4.0
        } else {
            (rng.gen::<f64>() - 0.5) * 0.5
        };
        
        // 生成颜色 - 使用柔和的蓝色调
        let r = (150.0 + rng.gen::<f64>() * 50.0) as u8;
        let g = (180.0 + rng.gen::<f64>() * 75.0) as u8;
        let b = (200.0 + rng.gen::<f64>() * 55.0) as u8;
        let color = format!("rgb({}, {}, {})", r, g, b);
        
        Particle {
            position: Vector2::new(x, y),
            velocity: Vector2::new(vx, vy),
            acceleration: Vector2::new(0.0, 0.0),
            radius: 2.0 + rng.gen::<f64>() * 2.0,
            color,
            alpha: 0.7 + rng.gen::<f64>() * 0.3,
            life: if is_burst { 0.7 } else { 1.0 },
        }
    }
    
    // 更新粒子状态
    pub fn update(&mut self, width: f64, height: f64, mouse_x: f64, mouse_y: f64) {
        // 应用加速度
        self.velocity += self.acceleration;
        
        // 限制最大速度，防止粒子运动过快
        let speed = self.velocity.magnitude();
        if speed > 3.0 {
            self.velocity = self.velocity.normalize() * 3.0;
        }
        
        // 更新位置
        self.position += self.velocity;
        
        // 重置加速度
        self.acceleration.x = 0.0;
        self.acceleration.y = 0.0;
        
        // 检查边界碰撞并反弹
        if self.position.x < self.radius {
            self.position.x = self.radius;
            self.velocity.x = -self.velocity.x * 0.6; // 减少反弹能量
        } else if self.position.x > width - self.radius {
            self.position.x = width - self.radius;
            self.velocity.x = -self.velocity.x * 0.6;
        }
        
        if self.position.y < self.radius {
            self.position.y = self.radius;
            self.velocity.y = -self.velocity.y * 0.6;
        } else if self.position.y > height - self.radius {
            self.position.y = height - self.radius;
            self.velocity.y = -self.velocity.y * 0.6;
        }
        
        // 鼠标互动 - 计算鼠标与粒子间的距离
        let dx = mouse_x - self.position.x;
        let dy = mouse_y - self.position.y;
        let distance = (dx * dx + dy * dy).sqrt();
        
        // 当鼠标靠近时施加力
        if distance > 0.0 && distance < 80.0 {
            let force = 0.5 / distance; // 力的大小随距离增大而减小
            let angle = dy.atan2(dx);
            
            // 对粒子施加基于距离的推力（远离鼠标）
            self.acceleration.x -= force * angle.cos();
            self.acceleration.y -= force * angle.sin();
        }
        
        // 爆发粒子的生命周期减少
        if self.life < 1.0 {
            self.life -= 0.01;
            self.alpha = self.life;
            
            // 生命结束后重置
            if self.life <= 0.0 {
                self.life = 1.0;
                self.alpha = 0.7 + rand::thread_rng().gen::<f64>() * 0.3;
            }
        }
        
        // 缓慢减速，模拟空气阻力
        self.velocity *= 0.99;
    }
    
    // 绘制粒子
    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        // 保存当前绘图状态
        context.save();
        
        // 设置全局透明度
        context.set_global_alpha(self.alpha);
        
        // 设置填充颜色
        context.set_fill_style(&JsValue::from_str(&self.color));
        
        // 绘制圆形
        context.begin_path();
        context.arc(
            self.position.x,
            self.position.y,
            self.radius,
            0.0,
            std::f64::consts::PI * 2.0
        ).unwrap();
        context.fill();
        
        // 添加发光效果 - 绘制一个更大的半透明圆
        context.set_fill_style(&JsValue::from_str(&format!(
            "rgba({}, {}, {}, 0.2)",
            // 提取颜色分量 - 这是一个简化方法
            200, 220, 255
        )));
        
        context.begin_path();
        context.arc(
            self.position.x,
            self.position.y,
            self.radius * 2.0,
            0.0,
            std::f64::consts::PI * 2.0
        ).unwrap();
        context.fill();
        
        // 恢复绘图状态
        context.restore();
    }
}

// 粒子系统
pub struct ParticleSystem {
    particles: Vec<Particle>,
    width: f64,
    height: f64,
    max_connection_dist: f64,
}

impl ParticleSystem {
    // 创建新的粒子系统
    pub fn new(count: usize, width: f64, height: f64, max_connection_dist: f64) -> Self {
        let mut particles = Vec::with_capacity(count);
        let mut rng = rand::thread_rng();
        
        // 创建初始粒子
        for _ in 0..count {
            let x = rng.gen::<f64>() * width;
            let y = rng.gen::<f64>() * height;
            particles.push(Particle::new(x, y, false));
        }
        
        ParticleSystem {
            particles,
            width,
            height,
            max_connection_dist,
        }
    }
    
    // 更新粒子系统
    pub fn update(&mut self, mouse_x: f64, mouse_y: f64) {
        // 遍历并更新所有粒子
        for particle in &mut self.particles {
            particle.update(self.width, self.height, mouse_x, mouse_y);
        }
        
        // 计算粒子间相互作用力
        self.apply_particle_forces();
    }
    
    // 绘制粒子系统
    pub fn draw(&self, context: &CanvasRenderingContext2d) {
        // 首先绘制粒子间的连线
        self.draw_connections(context);
        
        // 然后绘制所有粒子
        for particle in &self.particles {
            particle.draw(context);
        }
    }
    
    // 绘制粒子间的连线
    fn draw_connections(&self, context: &CanvasRenderingContext2d) {
        context.save();
        
        // 遍历所有粒子对
        for i in 0..self.particles.len() {
            for j in i+1..self.particles.len() {
                let p1 = &self.particles[i];
                let p2 = &self.particles[j];
                
                // 计算两粒子间距离
                let dx = p1.position.x - p2.position.x;
                let dy = p1.position.y - p2.position.y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                // 如果距离小于阈值，绘制连线
                if distance < self.max_connection_dist {
                    // 透明度随距离变化
                    let opacity = 1.0 - distance / self.max_connection_dist;
                    let alpha = opacity * 0.5 * p1.alpha.min(p2.alpha);
                    
                    // 设置线宽随距离变化
                    let line_width = (1.0 - distance / self.max_connection_dist) * 0.8;
                    context.set_line_width(line_width);
                    
                    // 设置线条颜色和透明度
                    context.set_stroke_style(&JsValue::from_str(&format!(
                        "rgba(180, 220, 255, {})", alpha
                    )));
                    
                    // 绘制线条
                    context.begin_path();
                    context.move_to(p1.position.x, p1.position.y);
                    context.line_to(p2.position.x, p2.position.y);
                    context.stroke();
                }
            }
        }
        
        context.restore();
    }
    
    // 计算粒子间相互作用力
    fn apply_particle_forces(&mut self) {
        // 从粒子向量中创建可变引用的临时向量，用于借用检查
        let positions: Vec<Vector2<f64>> = self.particles.iter().map(|p| p.position).collect();
        
        // 计算粒子间引力
        for i in 0..self.particles.len() {
            let mut force_x = 0.0;
            let mut force_y = 0.0;
            
            for j in 0..positions.len() {
                if i != j {
                    let dx = positions[j].x - positions[i].x;
                    let dy = positions[j].y - positions[i].y;
                    let distance = (dx * dx + dy * dy).sqrt();
                    
                    // 只计算一定距离内的力
                    if distance > 0.0 && distance < self.max_connection_dist * 0.8 {
                        let strength = 0.02 / (distance * distance);
                        force_x += dx * strength;
                        force_y += dy * strength;
                    }
                }
            }
            
            // 应用计算出的力
            self.particles[i].acceleration.x += force_x;
            self.particles[i].acceleration.y += force_y;
        }
    }
    
    // 创建粒子爆发效果
    pub fn create_burst(&mut self, x: f64, y: f64) {
        // 创建爆发粒子的数量
        let burst_count = 8;
        
        // 保存当前粒子数量
        let current_count = self.particles.len();
        
        // 计算最终粒子数，防止粒子过多影响性能
        let max_particles = 120;
        
        // 如果需要，移除一些旧粒子以保持性能
        if current_count + burst_count > max_particles {
            self.particles.drain(0..(current_count + burst_count - max_particles));
        }
        
        // 创建新的爆发粒子
        for _ in 0..burst_count {
            self.particles.push(Particle::new(x, y, true));
        }
    }
} 