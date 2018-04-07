/*
 * Copyright (c) 2017 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use gtk::prelude::*;
use relm::Widget;

use self::Msg::*;

#[derive(Msg)]
pub enum Msg {
    Quit,
}

relm_widget! {
    impl ::relm::Widget for Win {
        fn model() -> () {
            ()
        }

        fn update(&mut self, event: Msg) {
            match event {
                Quit => gtk::main_quit(),
            }
        }

        view! {
            gtk::Window {
                gtk::Notebook {
                    gtk::Button {
                        child: {
                            tab_label: Some("First Button"),
                        },
                        label: "+",
                    },
                    gtk::Label {
                        tab: {
                            label: &gtk::Label::new("Second page"),
                        },
                        text: "0",
                    },
                    gtk::Button {
                        label: "-",
                    },
                },
                delete_event(_, _) => (Quit, Inhibit(false)),
            }
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}
