use assert_gui::Gui;
use assert_gui::GuiError;

const FIRST_ANSWER: (f64, f64) = (480.0, 200.0);
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
    Gui::bin("quizers")
        .open()?
        .click(FIRST_ANSWER)?
        .click(NEXT_BUTTON)?
        .click(NEXT_BUTTON)?
        .assert()
        .text_from_portion(QUESTION_TXT, START_POINT, END_POINT)?
        .gui()
        .kill()?;

    Ok(())
}
