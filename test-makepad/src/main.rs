use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                body = <View> {
                    flow: Down,
                    spacing: 24,
                    align: { x: 0.5, y: 0.5 },
                    count_label = <Label> {
                        text: "0"
                        draw_text: {
                            text_style: { font_size: 48 }
                            color: #fff
                        }
                    }
                    plus_btn = <Button> {
                        text: "+1"
                        draw_text: { text_style: { font_size: 18 } }
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    counter: i32,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(plus_btn)).clicked(actions) {
            self.counter += 1;
            self.ui
                .label(id!(count_label))
                .set_text(cx, &self.counter.to_string());
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

fn main() {
    app_main();
}
