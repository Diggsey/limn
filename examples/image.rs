extern crate limn;

mod util;

use limn::prelude::*;
use limn::widgets::image::ImageBuilder;

fn main() {
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Limn image demo")
        .with_min_dimensions(100, 100);
    let app = util::init(window_builder);
    let mut root = WidgetBuilder::new("root");

    let mut image_widget = ImageBuilder::new("rust.png");
    image_widget.layout().add(constraints![
        center(&root),
        bound_by(&root).padding(50.0),
    ]);
    root.add_child(image_widget);

    app.main_loop(root);
}
