use serde::Deserialize;
use serde::Serialize;
use too::backend::Key;
use too::layout::Anchor2;
use too::math::Size;
use too::math::Space;
use too::renderer::Pixel;
use too::renderer::Rgba;
use too::renderer::TextShape;
use too::view::Builder;
use too::view::EventCtx;
use too::view::Handled;
use too::view::Interest;
use too::view::Layout;
use too::view::Ui;
use too::view::View;
use too::view::ViewEvent;

#[derive(Debug)]
struct TypingBox {
    paragraphs: Vec<Paragraph>,
    pub written: String,
    paragraph_index: usize,
    style: Style,
}

impl Default for TypingBox {
    fn default() -> Self {
        Self {
            paragraphs: vec![],
            paragraph_index: 0,
            written: String::new(),
            style: Style::default(),
        }
    }
}

impl TypingBox {
    fn config(mut self, config: Config) -> Self {
        for word in config.words {
            if word.finished {
                continue;
            }
            self.paragraphs.push(word);
        }

        self.style = config.style;

        self
    }
}

impl View for TypingBox {
    type Args<'v> = Self;

    type Response = ();

    fn create(args: Self::Args<'_>) -> Self {
        args
    }

    fn layout(&mut self, _layout: Layout, space: Space) -> Size {
        space.fit(space.max)
    }

    fn update(&mut self, _: Self::Args<'_>, _: &Ui) -> Self::Response {
        Self::Response::default()
    }

    fn interactive(&self) -> bool {
        true
    }

    fn draw(&mut self, mut render: too::view::Render) {
        let style = self.style.bg;
        let lighten = self.style.lighten;
        let style = Rgba::new(style.0, style.1, style.2, style.3).lighten(lighten);
        render.fill_bg(style);
        let mut others = self.written.chars().peekable();
        let top_margin = 10;

        if let Some(paragraph) = self.paragraphs.get(self.paragraph_index) {
            let style = self.style.word_color;
            let style = Rgba::new(style.0, style.1, style.2, style.3);
            render.text(TextShape::new(&paragraph.word).fg(style));

            for (line_index, line) in paragraph.paragraph.lines().enumerate() {
                for (char_index, c) in line.char_indices() {
                    match others.next() {
                        Some(written) => {
                            let pixel = if written == c {
                                let style = self.style.spell_correct;
                                let style = Rgba::new(style.0, style.1, style.2, style.3);
                                Pixel::new(written).fg(style)
                            } else {
                                let style = self.style.spell_erro;
                                let style = Rgba::new(style.0, style.1, style.2, style.3);
                                Pixel::new(if c != ' ' { c } else { '_' }).fg(style)
                            };

                            render.set((char_index as i32, line_index as i32 + top_margin), pixel);
                        }
                        None => {
                            let style = self.style.shadow_text;
                            let style = Rgba::new(style.0, style.1, style.2, style.3);
                            let pixel = Pixel::new(c).fg(style);

                            render.set((char_index as i32, line_index as i32 + top_margin), pixel);
                        }
                    }
                }
            }
        }
    }

    fn interests(&self) -> too::view::Interest {
        Interest::ALL
    }

    fn event(&mut self, event: ViewEvent, _ctx: EventCtx) -> Handled {
        if let ViewEvent::KeyInput { key, modifiers, .. } = event {
            match key {
                Key::Char(c) => {
                    if self.written.len()
                        < self
                            .paragraphs
                            .get(self.paragraph_index)
                            .unwrap()
                            .paragraph
                            .len()
                    {
                        self.written.push(c);
                    }
                    return Handled::Sink;
                }
                Key::Delete | Key::Backspace => {
                    self.written.pop();
                    return Handled::Sink;
                }
                Key::Tab => {
                    if modifiers.is_shift() {
                        self.written = String::new();
                        self.paragraph_index = self.paragraph_index.checked_sub(1).unwrap_or(0);
                    } else {
                        self.written = String::new();
                        if self.paragraph_index < self.paragraphs.len() {
                            self.paragraph_index += 1;
                        }
                    }
                }
                _ => {}
            }
        };
        Handled::Bubble
    }
}

impl Builder<'_> for TypingBox {
    type View = TypingBox;

    type Style = ();
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Paragraph {
    pub word: String,
    pub paragraph: String,
    #[serde(default)]
    pub finished: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Style {
    #[serde(default = "Style::default_spell_error")]
    spell_erro: (u8, u8, u8, u8),
    #[serde(default = "Style::default_spell_correct")]
    spell_correct: (u8, u8, u8, u8),
    #[serde(default = "Style::default_shadow_text")]
    shadow_text: (u8, u8, u8, u8),
    #[serde(default = "Style::default_bg")]
    bg: (u8, u8, u8, u8),
    #[serde(default = "Style::default_lighten")]
    lighten: f32,
    #[serde(default = "Style::default_word_color")]
    word_color: (u8, u8, u8, u8),
}

impl Style {
    fn default_spell_error() -> (u8, u8, u8, u8) {
        (200, 30, 20, 90)
    }

    fn default_spell_correct() -> (u8, u8, u8, u8) {
        (40, 200, 50, 90)
    }

    fn default_shadow_text() -> (u8, u8, u8, u8) {
        (169, 169, 169, 100)
    }

    fn default_bg() -> (u8, u8, u8, u8) {
        (65, 105, 225, 100)
    }

    fn default_lighten() -> f32 {
        0.9
    }
    fn default_word_color() -> (u8, u8, u8, u8) {
        (40, 200, 50, 90)
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            spell_erro: Self::default_spell_error(),
            spell_correct: Self::default_spell_correct(),
            shadow_text: Self::default_shadow_text(),
            bg: Self::default_bg(),
            lighten: Self::default_lighten(),
            word_color: Self::default_word_color(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Config {
    #[serde(default)]
    style: Style,
    words: Vec<Paragraph>,
}

fn main() -> std::io::Result<()> {
    let app = |ui: &Ui| {
        let filepath = std::env::args().nth(1).unwrap_or("./config.toml".into());
        let content = std::fs::read_to_string(filepath).unwrap();
        let config: Config = toml::from_str(&content).unwrap();

        ui.show(TypingBox::default().config(config));
    };

    too::application(
        too::RunConfig {
            debug: too::view::DebugMode::Rolling,
            debug_anchor: Anchor2::LEFT_TOP,
            fps: 30.0,
            ctrl_z_switches: true,
            ..Default::default()
        },
        app,
    )
}
