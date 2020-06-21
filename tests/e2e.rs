use assert_gui::Gui;
use assert_gui::GuiError;
use std::time::Duration;

const FIRST_ANSWER: (f64, f64) = (1234.0, 330.0);
const NEXT_BUTTON: (f64, f64) = (1657.0, 1037.0);

#[test]
fn test() -> Result<(), GuiError> {
    Gui::bin("quizers")?
        .open()
        .click(FIRST_ANSWER)?
        .click(NEXT_BUTTON)?
        .assert_text("Test");

    Ok(())
}
