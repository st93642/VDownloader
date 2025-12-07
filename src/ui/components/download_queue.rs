/*****************************************************************************/
/*                                                                           */
/*  download_queue.rs                                    TTTTTTTT SSSSSSS II */
/*                                                          TT    SS      II */
/*  By: st93642@students.tsi.lv                             TT    SSSSSSS II */
/*                                                          TT         SS II */
/*  Created: Dec 07 2025 13:36 st93642                      TT    SSSSSSS II */
/*  Updated: Dec 07 2025 13:37 st93642                                       */
/*                                                                           */
/*   Transport and Telecommunication Institute - Riga, Latvia                */
/*                       https://tsi.lv                                      */
/*****************************************************************************/

use gtk4::{prelude::*, Box, Frame, Label, Orientation, ScrolledWindow};

pub fn create_queue_placeholder() -> Frame {
    let frame = Frame::builder()
        .label("Download Queue")
        .margin_top(12)
        .build();

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vscrollbar_policy(gtk4::PolicyType::Automatic)
        .min_content_height(200)
        .build();

    let queue_box = Box::new(Orientation::Vertical, 6);
    queue_box.set_margin_top(12);
    queue_box.set_margin_bottom(12);
    queue_box.set_margin_start(12);
    queue_box.set_margin_end(12);

    let placeholder_label = Label::new(Some("No downloads in queue"));
    placeholder_label.add_css_class("dim-label");

    queue_box.append(&placeholder_label);
    scrolled_window.set_child(Some(&queue_box));
    frame.set_child(Some(&scrolled_window));

    frame
}
