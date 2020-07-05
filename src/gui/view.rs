// use crate::gui::helpers::{
//     build_view, button, controls, question_text, question_view, questions_list,
// };
// use crate::gui::page::View;
// use crate::gui::quizers::{Elem, Msg};
// use conv::prelude::*;
// use iced::{Column, Text};
// use md_questions::Questions;

// pub(crate) fn first_question<'a>(state: &'a mut State) -> Elem<'a> {
//     let back = button(&mut state.back_button, "Back");
//     let next = button(&mut state.next_button, "Next").on_press(Msg::NextPressed);
//     build_view(
//         questions_list(
//             &mut state.scroll,
//             &mut state.questions_labels,
//             state.page_idx,
//         ),
//         question_view(
//             question_text(
//                 &state.questions[state.page_idx],
//                 state.selected_answers[state.page_idx],
//             ),
//             controls(back, next),
//         ),
//     )
// }

// pub(crate) fn middle_question<'a>(state: &'a mut State) -> Elem<'a> {
//     let back = button(&mut state.back_button, "Back").on_press(Msg::BackPressed);
//     let next = button(&mut state.next_button, "Next").on_press(Msg::NextPressed);
//     build_view(
//         questions_list(
//             &mut state.scroll,
//             &mut state.questions_labels,
//             state.page_idx,
//         ),
//         question_view(
//             question_text(
//                 &state.questions[state.page_idx],
//                 state.selected_answers[state.page_idx],
//             ),
//             controls(back, next),
//         ),
//     )
// }

// pub(crate) fn last_question<'a>(state: &'a mut State) -> Elem<'a> {
//     let back = button(&mut state.back_button, "Back");
//     let finish = button(&mut state.finish_button, "Finish").on_press(Msg::ShowResults);
//     build_view(
//         questions_list(
//             &mut state.scroll,
//             &mut state.questions_labels,
//             state.page_idx,
//         ),
//         question_view(
//             question_text(
//                 &state.questions[state.page_idx],
//                 state.selected_answers[state.page_idx],
//             ),
//             controls(back, finish),
//         ),
//     )
// }

// pub(crate) fn results<'a>(state: &'a mut State) -> Elem<'a> {
//     let back = button(&mut state.back_button, "Back");
//     let restart = button(&mut state.restart_button, "Restart");
//     let points = count_points(&state.questions, &state.selected_answers);
//     let result = format_result_msg(points, state.questions.len());
//     let results_section = Column::new().spacing(20).push(Text::new(result));
//     build_view(
//         questions_list(
//             &mut state.scroll,
//             &mut state.questions_labels,
//             state.page_idx,
//         ),
//         question_view(results_section.into(), controls(back, restart)),
//     )
// }

// fn count_points(questions: &Questions, selected_answers: &[Option<usize>]) -> u32 {
//     let mut points = 0;
//     for i in 0..questions.len() {
//         if let Some(idx) = selected_answers[i] {
//             if questions[i].answer(idx).is_correct() {
//                 points += 1;
//             }
//         }
//     }
//     points
// }

// fn format_result_msg(points: u32, questions_count: usize) -> String {
//     format!(
//         "You've got {}/{} ({:.2}%) points",
//         points,
//         questions_count,
//         f64::value_from(points).expect("failed to convert from usize to f64")
//             / f64::value_from(questions_count).expect("failed to convert from usize to f64")
//     )
// }
