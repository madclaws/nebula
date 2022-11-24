# Notes

- Vertex data - array of floats in Normalized Device Coordinates (NDC).
    - Vertex data (3d) will be processed by vertex shader
- Vertex Buffer Objects (VBO) -> Managing GPU memory to store vertex data in
    GPU

## 2022-02-08 03:05:22

- Refactor in a sense, core stuff should be in their own modules
- All the rendering exercises will be their own module
- Want to render some module, add it to main and run it..
- May be we can have a trait implementation as `App`, that have `run` and all.

## 2022-02-11 21:31:13

- We are passing the color for wach vertex also a 2 floats in VBO + VAO

## 2022-02-14 02:43:02

- Restructure and refactor.
- Shader module is done, now we have to use that.

- When we use sub-folders inside `src`, Rust see it as a module, but
if try to import it it , then Rust expects a mod.rs.
ex foo/mod.rs.
So we can make a file mod.rs, and them import other modules inside foo in mod.rs.

##  2022-11-24 16:56:58

- femo commit test 1
- femo commit test 2
- femo commit test 3