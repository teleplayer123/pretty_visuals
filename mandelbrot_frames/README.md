## Description

Rust project to experiment with cool visuals. Build and run the project, then run the following ffmpeg command:

```bash
ffmpeg -framerate 60 -i frames/frame_%03d.png -c:v libx264 -crf 18 -pix_fmt yuv420p mandelbrot.mp4
```