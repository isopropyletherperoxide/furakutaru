# furakutaru
Fractal Generator (originally in Go) rewritten in Rust

## Examples
![Julia Example](


![Mandelbrot Example](https://github.com/isopropyletherperoxide/furakutaru/blob/main/example/mandelbrot.png?raw=true)

### Installation
Run ``cargo build --release`` in the project folder.

### Configuration
Edit the ``config.toml`` file to configure the fractal generator
#### Options
- ``width``, ``height`` (``int``): width and height of the output image
- ``scale_fac`` (float):  multiplier for scale of the fractal, the lower the more zoomed in the fractal
- ``fractal_type`` (string): type of fractal, either ``"Mandelbrot"`` or ``"Julia"`` 
- ``julia_r``, ``julia_i`` (float): real and imaginary parts of the constant part of the julia set equation



