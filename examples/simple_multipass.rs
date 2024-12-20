
fn main() {

}
// use std::num::NonZero;
//
// use bevy::prelude::*;
// use bevy_egui::{EguiContext, EguiFullOutput, EguiInput, EguiPlugin, EguiSettings, EguiStartupSet};
// use egui::{Align2, Color32, Frame};
// use egui_flex::{Flex, FlexAlign, FlexItem, FlexJustify};
//
// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_plugins(EguiPlugin)
//         .add_systems(
//             PreStartup,
//             configure_context.after(EguiStartupSet::InitContexts),
//         )
//         .add_systems(Update, ui_example_system)
//         .run();
// }
//
// fn configure_context(mut egui_settings: Query<&mut EguiSettings>) {
//     egui_settings.single_mut().run_manually = true;
// }
//
// fn ui_example_system(mut contexts: Query<(&mut EguiContext, &mut EguiInput, &mut EguiFullOutput)>) {
//     let (mut ctx, mut egui_input, mut egui_full_output) = contexts.single_mut();
//
//     let ui = |ctx: &egui::Context| {
//         egui::Window::new("Hello").show(ctx, |ui| {
//             // let passes = ui
//             //     .ctx()
//             //     .viewport(|viewport| viewport.output.num_completed_passes)
//             //     + 1;
//             // ui.label(format!("Passes: {}", passes));
//             // ui.ctx().request_discard("Trying to reach max limit");
//             let frame = Frame::none()
//                 .fill(Color32::from_rgb(245, 245, 245))
//                 // .stroke(Stroke::new(1.0, Color32::from_gray(200)))
//                 .outer_margin(0.0)
//                 .inner_margin(2.0);
//             Flex::new().show(ui, |flex| {
//                 flex.add_flex_frame(
//                     FlexItem::new().grow(0.0),
//                     Flex::vertical().align_items(egui_flex::FlexAlign::Stretch),
//                     frame,
//                     |flex| {
//                         // Title and close button
//                         flex.add_ui(FlexItem::new(), |ui| {
//                             Flex::new()
//                                 .justify(FlexJustify::SpaceBetween)
//                                 .show(ui, |flex| {
//                                     flex.add_ui(FlexItem::new().grow(1.0), |ui| {
//                                         ui.label("测量");
//                                     });
//                                     flex.add_ui(
//                                         FlexItem::new()
//                                             .grow(0.0)
//                                             .align_self_content(Align2::LEFT_BOTTOM),
//                                         |ui| {
//                                             if ui
//                                                 .add(
//                                                     egui::Button::new(
//                                                         egui::RichText::new("×").size(20.0),
//                                                     )
//                                                     .frame(false),
//                                                 )
//                                                 .clicked()
//                                             {
//                                                 // self.show_login = false;
//                                                 // Handle close action
//                                             }
//                                         },
//                                     );
//                                 });
//                         });
//
//                         flex.add_flex(
//                             FlexItem::new().align_self(FlexAlign::Start),
//                             Flex::horizontal(),
//                             |flex| {
//                                 flex.add_ui(FlexItem::new(), |ui| {
//                                     // egui::Button::new("Ruler").show(ui);
//                                     // egui::Button::new("Eraser").show(ui);
//                                     ui.button("Ruler");
//                                     ui.button("Eraser");
//                                     // if EguiButton::default().icon(&Icon::RULER).show(ui).clicked() {
//                                     //     current_line_measure.clear();
//                                     // }
//                                     // if EguiButton::default().icon(&Icon::ERASER).show(ui).clicked()
//                                     // {
//                                     //     current_line_measure.clear();
//                                     //     measure_points.selected_points.clear();
//                                     // }
//                                 });
//                             },
//                         );
//                     },
//                 );
//             });
//         });
//     };
//
//     let ctx = ctx.get_mut();
//     ctx.memory_mut(|memory| {
//         memory.options.max_passes = NonZero::new(4).unwrap();
//     });
//
//     **egui_full_output = Some(ctx.run(egui_input.take(), ui));
//     egui::CentralPanel::default().show(&ctx, |ui| {
//         ui.label("Hello egui!");
//     });
// }
