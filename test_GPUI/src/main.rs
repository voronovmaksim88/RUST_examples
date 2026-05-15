use gpui::*;
use gpui_platform::application;

struct Counter {
    count: i32,
}

impl Render for Counter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .bg(rgb(0x1a1a2e))
            .child(
                div()
                    .mb_6()
                    .text_xl()
                    .text_color(rgb(0x00d4ff))
                    .child(format!("Count: {}", self.count)),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        div()
                            .id("increment")
                            .px_4()
                            .py_2()
                            .bg(rgb(0x0f3460))
                            .rounded_md()
                            .hover(|style| style.bg(rgb(0x16213e)))
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _click, _window, cx| {
                                this.count += 1;
                                cx.notify();
                            }))
                            .child("Increment"),
                    )
                    .child(
                        div()
                            .id("decrement")
                            .px_4()
                            .py_2()
                            .bg(rgb(0x533483))
                            .rounded_md()
                            .hover(|style| style.bg(rgb(0x6a3d99)))
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _click, _window, cx| {
                                this.count -= 1;
                                cx.notify();
                            }))
                            .child("Decrement"),
                    )
                    .child(
                        div()
                            .id("reset")
                            .px_4()
                            .py_2()
                            .bg(rgb(0xe94560))
                            .rounded_md()
                            .hover(|style| style.bg(rgb(0xff6b6b)))
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _click, _window, cx| {
                                this.count = 0;
                                cx.notify();
                            }))
                            .child("Reset"),
                    ),
            )
    }
}

fn main() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(400.), px(300.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("GPUI Counter".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| cx.new(|_| Counter { count: 0 }),
        )
        .unwrap();
        cx.activate(true);
    });
}
