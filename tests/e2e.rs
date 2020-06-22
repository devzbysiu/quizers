use assert_gui::Gui;
use assert_gui::GuiError;

const FIRST_ANSWER: (f64, f64) = (757.0, 321.0);
const NEXT_BUTTON: (f64, f64) = (1151.0, 1032.0);

#[test]
fn test() -> Result<(), GuiError> {
    Gui::bin("quizers")?
        .open()
        .click(FIRST_ANSWER)?
        .click(NEXT_BUTTON)?
        .assert_text("Test");

    Ok(())
}
