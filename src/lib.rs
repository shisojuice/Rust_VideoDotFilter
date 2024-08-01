use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn pixel_filter(mut buffer: Vec<u8>,canvas_width :u32,canvas_height :u32,dot_size :u32) -> Vec<u8> {
    let width = canvas_width as usize;
    let height = canvas_height as usize;
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
                        buffer[i + 3] = 255; // アルファ値
                    }
                }
            }
        }
    }

    buffer
}


fn closest_color(r: u32, g: u32, b: u32) -> (u32, u32, u32) {
    let colors = [
        (171, 0, 19),
        (231, 0, 91),
        (255, 119, 183),
        (255, 199, 219),
        (167, 0, 0),
        (219, 43, 0),
        (255, 119, 99),
        (255, 191, 179),
        (127, 11, 0),
        (203, 79, 15),
        (255, 155, 59),
        (255, 219, 171),
        (67, 47, 0),
        (139, 115, 0),
        (243, 191, 63),
        (255, 231, 163),
        (0, 71, 0),
        (0, 151, 0),
        (131, 211, 19),
        (227, 255, 163),
        (0, 81, 0),
        (0, 171, 0),
        (79, 223, 75),
        (171, 243, 191),
        (0, 63, 23),
        (0, 147, 59),
        (88, 248, 152),
        (179, 255, 207),
        (27, 63, 95),
        (0, 131, 139),
        (0, 235, 219),
        (159, 255, 243),
        (39, 27, 143),
        (0, 115, 239),
        (63, 191, 255),
        (171, 231, 255),
        (0, 0, 171),
        (35, 59, 239),
        (95, 115, 255),
        (199, 215, 255),
        (71, 0, 159),
        (131, 0, 243),
        (167, 139, 253),
        (215, 203, 255),
        (143, 0, 119),
        (191, 0, 191),
        (247, 123, 255),
        (255, 199, 255),
        (0, 0, 0),
        (117, 117, 117),
        (188, 188, 188),
        (255, 255, 255)
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