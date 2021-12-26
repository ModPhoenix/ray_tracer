# Ray tracer

Ray tracer written on Rust, based on the book ["The Ray Tracer Challenge" by Jamis Buck](http://www.raytracerchallenge.com/).

## What can the ray tracer do currently?

For now it can render scene using virtual "world" and camera into .ppm image format. World can hold one light source and any number of graphic primitives.

You can apply various transformations and material settings to the object. All of these factors will affect the final pixel color in the final picture.

### Available graphic primitives

- Sphere
- Plane
- Cube
- Cylinder

### Examples of rendered scenes

![Two mirrors in front of each other](progress/chapter_12_02.png)
![Reflection and Refraction and Cubes](progress/chapter_12_01.png)
![Reflection and Refraction](progress/chapter_11_02.png)
![Three spheres on a plane with patterns](progress/chapter_10_03.png)
![Three spheres on a plane](progress/chapter_09_01.png)
![Three spheres on a plane from afar](progress/chapter_09_02.png)

## Getting started

You can customize the scene in `src/main.rs` file and then render an image using `cargo run`, image file will be named `scene.png`.

```sh
cargo run --release

open scene.png
```

## Progress in the book

- [x] Chapter 01: Tuples, Points, and Vectors
- [x] Chapter 02: Drawing on a Canvas
- [x] Chapter 03: Matrices
- [x] Chapter 04: Matrix Transformations
- [x] Chapter 05: Ray-Sphere Intersections
- [x] Chapter 06: Light and Shading
- [x] Chapter 07: Making a Scene
- [x] Chapter 08: Shadows
- [x] Chapter 09: Planes
- [x] Chapter 10: Patterns
- [x] Chapter 11: Reflection and Refraction
- [x] Chapter 12: Cubes
- [ ] Chapter 13: Cylinders
- [ ] Chapter 14: Groups
- [ ] Chapter 15: Triangles
- [ ] Chapter 16: Constructive Solid Geometry (CSG)
