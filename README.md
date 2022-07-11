# rust-renderer

## Motivation & Goals

tl;dr
> 1. Learn Rust as I've never used it before
>2. Mess around with rendering and Ray/Path Tracing

<br/>

Rust interested me as it promised a 'C like' experience while also being strict about memory management, multi-threading and other similar common pitfalls. I thought it best to dive head first in and try something that I knew would allow me to encounter these problems.

I'd recently heard about the [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) project and thought it would be interesting to tackle. I thought this would be a great combination as this would eventually rely on needing multi-threading and shared memory/states.

This project will look something akin to the code given in [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)/[RayTracing in Rust](https://misterdanb.github.io/raytracinginrust/). This will be my own interpretation though so do not use this as gospel if you are following along with that project and are trying to see what others have done to fix a bug you might be experiencing.

As stated the primary goal of this project is to learn, though it is likely that later in this project I might look to introduce an animation and physics engine to produce short videos rather than just still frames. I might also look to implement GPU acceleration to further improve the performance.

## How to use

Assuming you have Rust and Cargo set up correctly, you should be able to do `cargo run --release` and see the current output I've got it set up to do. Right now if you want to modify scenes etc you're going to have to modify the code. 

Note: You are going to want to run this in `release` mode, doing this resulted in a near 7.5x improvement in performance.

## References

- [Ray Tracing](https://raytracing.github.io/)
- [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
- [Rendering the Julia set in Rust to an image](https://github.com/image-rs/image)
- [RayTracing in Rust](https://misterdanb.github.io/raytracinginrust/)
