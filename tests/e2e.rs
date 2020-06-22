use assert_gui::Gui;
use assert_gui::GuiError;

const FIRST_ANSWER: (f64, f64) = (757.0, 321.0);
const NEXT_BUTTON: (f64, f64) = (1151.0, 1032.0);

const QUESTION_TXT: &str = r#"A developer is working on a complex project with multiple bundles. One bundle provides an OSGi service for other bundles. Which two options are necessary to ensure that the other bundles can reference that OSGi service? (Choose two.)

The bundles consuming the service need to import the fully qualified name of the service interface.
The service needs to correctly declare metatype information.
The bundle providing the service needs to contain a whitelist of allowed consumer bundles.
The bundle providing the service needs to contain an adequate SCR descriptor file.
The bundle providing the service needs to export the java package of the service interface.
"#;

#[test]
fn test() -> Result<(), GuiError> {
    Gui::bin("quizers")?
        .open()
        .click(FIRST_ANSWER)?
        .click(NEXT_BUTTON)?
        .assert()
        .with_similarity(0.75)
        .text(QUESTION_TXT)?;

    Ok(())
}
