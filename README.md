# Bevy_split_canvas

An alternative for rendering to [multiple canvases](https://github.com/bevyengine/bevy/issues/20453) while the feature is not available in bevy.

<p align="center">
    <img src="assets/crate_demo.gif">
</p>

```bash
# To compile the example
wasm-pack build "examples/" --target web

# To run a simple webserver
cd examples && python3 -m http.server 8000
```