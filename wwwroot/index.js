import init, { pixel_filter } from './Rust_VideoDotFilter.js';

const video = document.getElementById('myVideo');

const dot_canvas = document.getElementById('dotCanvas');
const dot_ctx = dot_canvas.getContext('2d',{willReadFrequently: true,});
const dot_size = document.getElementById("dot_size");

const none_filter = document.getElementById("none_filter");

navigator.mediaDevices.getUserMedia({ video: true, audio: false })
    .then(stream => {
        video.srcObject = stream;
        // 描画を開始
        video.addEventListener('loadeddata', () => {
            dot_canvas.width = video.videoWidth;
            dot_canvas.height = video.videoHeight;
            function draw() {
                dot_ctx.drawImage(video, 0, 0, dot_canvas.width, dot_canvas.height);
                requestAnimationFrame(draw);
            }
            draw();
        });
    })
    .catch(err => {
        console.error('エラー:', err);
    });

async function run() {
    await init();
    dot_size.addEventListener("change", () => {
        function draw() {
            const imageData = dot_ctx.getImageData(0, 0, dot_canvas.width, dot_canvas.height);
            const ret = pixel_filter(new Uint8Array(imageData.data.buffer),dot_canvas.width,dot_canvas.height,dot_size.value);
            dot_ctx.putImageData(new ImageData(new Uint8ClampedArray(ret.buffer), dot_canvas.width, dot_canvas.height), 0, 0);
            requestAnimationFrame(draw);
        }
        draw();
    });
    none_filter.addEventListener("click", () => {
        function draw() {
            dot_ctx.drawImage(video, 0, 0, dot_canvas.width, dot_canvas.height);
            requestAnimationFrame(draw);
        }
        draw();
    });
}
run();
