# furakutaru
Fractal Generator (originally in Go) rewritten in Rust

## Examples
<img alt="Julia Example" src="https://github.com/isopropyletherperoxide/furakutaru/blob/main/examples/julia.png?raw=true" width=256>

<img alt="Mandelbrot Example" src="https://github.com/isopropyletherperoxide/furakutaru/blob/main/examples/mandelbrot.png?raw=true" width=256>

### Installation
Run ``cargo build --release`` in the project folder.

### Configuration
Edit the ``config.toml`` file to configure the fractal generator
#### Options
- ``width``, ``height`` (``int``): width and height of the output image
- ``scale_fac`` (float):  multiplier for scale of the fractal, the lower the more zoomed in the fractal
- ``fractal_type`` (string): type of fractal, either ``"Mandelbrot"`` or ``"Julia"`` 
- ``julia_r``, ``julia_i`` (float): real and imaginary parts of the constant part of the julia set equation



