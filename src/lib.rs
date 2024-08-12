use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn pixel_filter(mut buffer: Vec<u8>,canvas_width :u32,canvas_height :u32,dot_size :u32) -> Vec<u8> {
    let width = canvas_width as usize;
    let height = canvas_height as usize;

    for i in 0..width * height {
        let index = i * 4; // RGBAなので4倍

        let r = buffer[index] as f32;
        let g = buffer[index + 1] as f32;
        let b = buffer[index + 2] as f32;
        let (h, s, l) = rgb_to_hsl(r, g, b);

        let new_s = s * 2.0;
        let new_s = new_s.clamp(0.0, 1.0); // 彩度が1.0まで

        // RGBに変換
        let (new_r, new_g, new_b) = hsl_to_rgb(h, new_s, l);
        buffer[index] = new_r as u8;
        buffer[index + 1] = new_g as u8;
        buffer[index + 2] = new_b as u8;
    }

    let dot_size = dot_size as usize;
    for y in (0..height).step_by(dot_size) {
        for x in (0..width).step_by(dot_size) {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;

            for dy in 0..dot_size {
                for dx in 0..dot_size {
                    let i = ((y + dy) * width + (x + dx)) * 4; // RGBAなので4倍
                    if i + 3 < buffer.len() {
                        r += buffer[i] as u32;
                        g += buffer[i + 1] as u32;
                        b += buffer[i + 2] as u32;
                    }
                }
            }

            // ドット内のすべてのピクセルに平均色を設定
            r /= (dot_size * dot_size) as u32;
            g /= (dot_size * dot_size) as u32;
            b /= (dot_size * dot_size) as u32;
            (r,g,b)=closest_color(r,g,b);
            for dy in 0..dot_size {
                for dx in 0..dot_size {
                    let i = ((y + dy) * width + (x + dx)) * 4;
                    if i + 3 < buffer.len() {
                        buffer[i] = r as u8;
                        buffer[i + 1] = g as u8;
                        buffer[i + 2] = b as u8;
                        buffer[i + 3] = 255;
                    }
                }
            }
        }
    }

    buffer
}

fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let r = r / 255.0;
    let g = g / 255.0;
    let b = b / 255.0;

    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let c = max - min;

    let mut h = 0.0;
    if c != 0.0 {
        if max == r {
            h = 60.0 * ((g - b) / c % 6.0);
        } else if max == g {
            h = 60.0 * ((b - r) / c + 2.0);
        } else if max == b {
            h = 60.0 * ((r - g) / c + 4.0);
        }
    }

    let l = (max + min) / 2.0;

    let s = if c == 0.0 {
        0.0
    } else {
        c / (1.0 - (2.0 * l - 1.0).abs())
    };

    (h, s, l)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h >= 0.0 && h < 60.0 {
        (c, x, 0.0)
    } else if h >= 60.0 && h < 120.0 {
        (x, c, 0.0)
    } else if h >= 120.0 && h < 180.0 {
        (0.0, c, x)
    } else if h >= 180.0 && h < 240.0 {
        (0.0, x, c)
    } else if h >= 240.0 && h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    ((r + m) * 255.0, (g + m) * 255.0, (b + m) * 255.0)
}

fn closest_color(r: u32, g: u32, b: u32) -> (u32, u32, u32) {
    let colors = [
        (171, 0, 19),  // 赤みがかった濃いピンク
        (231, 0, 91),  // 鮮やかなピンク
        (255, 119, 183),  // 薄いピンク
        (255, 199, 219),  // とても薄いピンク
        (167, 0, 0),  // 暗い赤
        (219, 43, 0),  // やや暗い赤
        (255, 119, 99),  // 明るい赤
        (255, 191, 179),  // 薄い赤
        (127, 11, 0),  // 暗いオレンジ
        (203, 79, 15),  // やや暗いオレンジ
        (255, 155, 59),  // 明るいオレンジ
        (255, 219, 171),  // 薄いオレンジ
        (67, 47, 0),  // 暗い黄色
        (139, 115, 0),  // やや暗い黄色
        (243, 191, 63),  // 明るい黄色
        (255, 231, 163),  // 薄い黄色
        (0, 71, 0),  // 暗い緑
        (0, 151, 0),  // やや暗い緑
        (131, 211, 19),  // 明るい緑
        (227, 255, 163),  // 薄い緑
        (0, 81, 0),  // 暗い青緑
        (0, 171, 0),  // やや暗い青緑
        (79, 223, 75),  // 明るい青緑
        (171, 243, 191),  // 薄い青緑
        (0, 63, 23),  // 暗いシアン
        (0, 147, 59),  // やや暗いシアン
        (88, 248, 152),  // 明るいシアン
        (179, 255, 207),  // 薄いシアン
        (27, 63, 95),  // 暗い青
        (0, 131, 139),  // やや暗い青
        (0, 235, 219),  // 明るい青
        (159, 255, 243),  // 薄い青
        (39, 27, 143),  // 暗い紫
        (0, 115, 239),  // やや暗い紫
        (63, 191, 255),  // 明るい紫
        (171, 231, 255),  // 薄い紫
        (0, 0, 171),  // 暗い藍色
        (35, 59, 239),  // やや暗い藍色
        (95, 115, 255),  // 明るい藍色
        (199, 215, 255),  // 薄い藍色
        (71, 0, 159),  // 暗いマゼンタ
        (131, 0, 243),  // やや暗いマゼンタ
        (167, 139, 253),  // 明るいマゼンタ
        (215, 203, 255),  // 薄いマゼンタ
        (143, 0, 119),  // 暗いピンク
        (191, 0, 191),  // やや暗いピンク
        (247, 123, 255),  // 明るいピンク
        (255, 199, 255),  // 薄いピンク
        (0, 0, 0),  // 黒
        (117, 117, 117),  // 灰色
        (188, 188, 188),  // 薄い灰色
        (255, 255, 255)  // 白
    ];
    *colors
        .iter()
        .min_by_key(|&&(cr, cg, cb)| color_distance(r, g, b, cr, cg, cb))
        .unwrap()
}

fn color_distance(r1: u32, g1: u32, b1: u32, r2: u32, g2: u32, b2: u32) -> u32 {
    let r_diff = r1 as i32 - r2 as i32;
    let g_diff = g1 as i32 - g2 as i32;
    let b_diff = b1 as i32 - b2 as i32;
    // 3次元空間上の距離を計算
    (r_diff * r_diff + g_diff * g_diff + b_diff * b_diff) as u32
}
