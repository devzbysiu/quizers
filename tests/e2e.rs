use assert_gui::GuiError;
use assert_gui::{Gui, OpenedGui};
use std::time::Duration;

const NEXT_BUTTON: (f64, f64) = (1833.0, 1035.0);
const START_POINT: (f64, f64) = (154.0, 2.0);
const END_POINT: (f64, f64) = (1919.0, 1079.0);

const QUESTION_3_TXT: &str = r#"The structure section of an editable template has a locked component. What happens to the content of that component when a developer unlocks it?

The content stays in the same place but it ignored on pages using the template.
The content is moved to the initial section of the editable template.
The content is deleted after confirmation from the template author.
The content is copied to the initial section of the editable template.
"#;

const QUESTION_58_TXT: &str = r#"A developer needs to upgrade existing components (Proxy Components) based on Core Components Version 1(v1) to Core Components Version 2(v2). How should the developer upgrade to V2 Core Components?

Modify the sling:resourceSuperType property on the proxy component to point to V2 Component.
Modify the sling:resourceSuperType property on the proxy component to point to V1 Component.
Create a new Proxy Component and set sling:resourceType property to V2 Core Component.
Proxy Components will be automatically upgraded to the V2 Core Component on AEM Restart.
"#;

#[test]
fn test() -> Result<(), GuiError> {
    QuizersGui::open()?
        .go_to_question(3)?
        .assert_txt(QUESTION_3_TXT)?
        .go_to_question(58)?
        .assert_txt(QUESTION_58_TXT)?
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
            gui: Gui::bin("quizers")
                .with_pauses(Duration::from_millis(100))
                .open()?,
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

    fn assert_txt<S: Into<String>>(self, txt: S) -> Result<Self, GuiError> {
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
