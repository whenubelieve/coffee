use coffee::graphics::{Color, Window, WindowSettings};
use coffee::ui::{button, renderer, Button, Column, Root, UserInterface};
use coffee::{Game, Result, Timer};

fn main() -> Result<()> {
    Menu::run(WindowSettings {
        title: String::from("Examples menu - Coffee"),
        size: (1280, 1024),
        resizable: false,
    })
}

struct Menu {}

impl Game for Menu {
    type Input = ();
    type View = ();
    type UserInterface = Ui;

    const TICKS_PER_SECOND: u16 = 10;

    fn new(_window: &mut Window) -> Result<(Menu, Self::Input, Self::View)> {
        Ok((Menu {}, (), ()))
    }

    fn update(&mut self, _view: &Self::View, _window: &Window) {}

    fn draw(
        &self,
        _view: &mut Self::View,
        window: &mut Window,
        _timer: &Timer,
    ) {
        let mut frame = window.frame();
        frame.clear(Color::BLACK);
    }
}

struct Ui {
    particles_button: button::State,
    input_button: button::State,
    color_button: button::State,
}

impl UserInterface for Ui {
    type Msg = Msg;
    type Renderer = renderer::Basic;

    fn new(window: &mut Window) -> (Ui, Self::Renderer) {
        (
            Ui {
                particles_button: button::State::new(),
                input_button: button::State::new(),
                color_button: button::State::new(),
            },
            renderer::Basic::new(window.gpu()),
        )
    }

    fn layout(&mut self, window: &Window) -> Root<Msg, Self::Renderer> {
        Root::new(
            Column::new()
                .width(window.width())
                .height(window.height())
                .center_children()
                .push(
                    Column::new()
                        .width(300.0)
                        .spacing(30)
                        .push(
                            Button::new(
                                &mut self.particles_button,
                                "Particles",
                            )
                            .on_click(Msg::ParticlesPressed),
                        )
                        .push(
                            Button::new(&mut self.input_button, "Input")
                                .on_click(Msg::InputPressed),
                        )
                        .push(
                            Button::new(&mut self.color_button, "Color")
                                .on_click(Msg::ColorPressed),
                        ),
                ),
        )
    }

    fn update(&mut self, msg: Msg) {}
}

#[derive(Debug)]
enum Msg {
    ParticlesPressed,
    InputPressed,
    ColorPressed,
}
