# Ray Tracer

**Yet another simple ray tracing implementation in Rust, based on Peter Shirley's ray tracing in one weekend book.**

![img](./output.jpg)

## Installation
> rust must be installed.
```
git clone https://github.com/metinUr/ray-tracer.git
cd ray-tracer
cargo run --release
```

## Examples
### Diffuse Material

![img](./examples/ex_lambertian.jpg)
```
cargo run --release --example ex_lambertian
```

### Metal Material

![img](./examples/ex_metal.jpg)
```
cargo run --release --example ex_metal
```

### Glass Material

![img](./examples/ex_dielectric.jpg)
```
cargo run --release --example ex_dielectric
```

### Random Spheres

![img](./examples/ex_random.jpg)
```
cargo run --release --example ex_random
```
or just run
```
cargo run --release
```
