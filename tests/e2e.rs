use assert_gui::GuiError;
use assert_gui::{Gui, OpenedGui};

const NEXT_BUTTON: (f64, f64) = (1644.0, 1043.0);
const START_POINT: (f64, f64) = (154.0, 2.0);
const END_POINT: (f64, f64) = (1919.0, 1079.0);

const QUESTION_TXT: &str = r#"The structure section of an editable template has a locked component. What happens to the content of that component when a developer unlocks it?

The content stays in the same place but it ignored on pages using the template.
The content is moved to the initial section of the editable template.
The content is deleted after confirmation from the template author.
The content is copied to the initial section of the editable template.
"#;

#[test]
fn test() -> Result<(), GuiError> {
    QuizersGui::open()?
        .go_to_question(3)?
        .assert_question(QUESTION_TXT)?
        .go_to_question(5)?
        .close()?;
    Ok(())
}

struct QuizersGui {
    gui: OpenedGui,
    current_question: usize,
}

impl QuizersGui {
    fn open() -> Result<Self, GuiError> {
        Ok(Self {
            gui: Gui::bin("quizers").open()?,
            current_question: 0,
        })
    }

    fn go_to_question(mut self, idx: usize) -> Result<Self, GuiError> {
        let idx = idx - 1 - self.current_question;
        let mut gui = self.gui.clone();
        for _ in 0..idx {
            gui = gui.click(NEXT_BUTTON).unwrap();
        }
        self.current_question = idx;
        Ok(self)
    }

    fn assert_question<S: Into<String>>(self, txt: S) -> Result<Self, GuiError> {
        self.gui
            .clone()
            .assert()
            .with_similarity(0.7)
            .text_from_portion(txt, START_POINT, END_POINT)?;
        Ok(self)
    }

    fn close(self) -> Result<(), GuiError> {
        self.gui.kill()?;
        Ok(())
    }
}
