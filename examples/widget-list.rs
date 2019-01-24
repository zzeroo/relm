/*
 * Copyright (c) 2019 Stefan Müller <co@zzeroo.com>
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

/// This example creates an application with many counters.
/// The user can add or remove these counters. The counter itselfs can each
/// increment or decrement a i32 value via buttons.
/// This example is a mix of the `button.rs` and `widget-list-attribute.rs`
/// examples. Primary to show how to combine relm widgets with gtk widgets.

extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
#[cfg_attr(test, macro_use)]
extern crate gtk_test;

use relm::{
    Component,
    ContainerWidget,
    Relm,
    Update,
    Widget,
    WidgetTest,
};
use gtk::prelude::*;
use gtk::{
    Button,
    Label,
    Orientation::{Horizontal, Vertical},
    Window,
    WindowType,
};



// # Models

// A model contains the data related to a `Widget`. It may be updated by
// the `Widget::update` function.
//
// We need two Models for this example.

// ## CounterModel
// The CounterModel is a simple i32 value
struct CounterModel {
    counter: i32,
}

// ## ApplicationModel
// The ApplicationModel, holds zero or more Counters
struct ApplicationModel {
    counters: Vec<Component<Counter>>,
}

// # Messages

// Messages are sent to `Widget::update` to indicate that an event happend.
// The model can be updated when an event is receive.
//
// This application needs two kind of messages.

// ## CounterMsg
// The CounterMsg are messages to decrement or increment the counter value
#[derive(Msg)]
enum CounterMsg {
    Decrement,
    Increment,
}

// ## ApplicationMsg
// The ApplicationMsg are messages used to add and remove counters
// plus a Quit message to quit the whole application
#[derive(Msg)]
enum ApplicationMsg {
    AddCounter,
    RemoveCounter,
    Quit,
}

// # Widgets

// Now we have to create the widgets. Relm widgets starts as plain rust structs.
// Later these structs implement `Update` and `Widget`. This implementations turn the
// structs into relm widgets.
// Another fact is that relm widgets can composed from gtk widgets or other relm widgets.

// ## Counter widget
// A simple counter widget, two gtk::Button ("+" and "-") plus a gtk::Label which shows
// the counter value.
struct Counter {
    counter_label: Label,
    model: CounterModel,
    vbox: gtk::Box,
}

// ## Win
// The Application is composed from the ApplicationModel and the widgets
struct Win {
    model: ApplicationModel,
    widgets: Widgets,
}

// You must keep your components as long as you want them to send/receive events.
// Common practice is to store Components in the Widget struct.
// See: https://github.com/antoyo/relm/blob/master/tests/communication.rs#L216-L220
#[derive(Clone)]
struct Widgets {
    window: Window,
    hbox: gtk::Box,
}

// ### `Update` Implementation for Counter
impl Update for Counter {
    type Model = CounterModel;
    type ModelParam = ();
    type Msg = CounterMsg;

    // Return the inital Model
    fn model(_: &Relm<Self>, _: ()) -> Self::Model {
        CounterModel {
            counter: 0,
        }
    }

    // how to update this counter model
    fn update(&mut self, event: CounterMsg) {
        let label = &self.counter_label;

        match event {
            CounterMsg::Decrement => {
                self.model.counter -= 1;
                label.set_text(&self.model.counter.to_string());
            },
            CounterMsg::Increment => {
                self.model.counter += 1;
                label.set_text(&self.model.counter.to_string());
            },
        }
    }
}

// ### `Update` Implementation for Win
impl Update for Win {
    type Model = ApplicationModel;
    type ModelParam = ();
    type Msg = ApplicationMsg;

    // Return the inital Model
    fn model(_: &Relm<Self>, _: ()) -> Self::Model {
        ApplicationModel {
            counters: vec![],
        }
    }

    // How to update the application model, add and remove counter
    // and quit the application
    fn update(&mut self, event: ApplicationMsg) {
        match event {
            ApplicationMsg::AddCounter => {
                let button = Button::new_with_label("FOo");
                self.widgets.hbox.add(&button);
                let widget = self.widgets.hbox.add_widget::<Counter>(());
                self.model.counters.push(widget);
            },
            ApplicationMsg::RemoveCounter => {
                if let Some(counter) = self.model.counters.pop() {
                    self.widgets.hbox.remove_widget(counter);
                }
            },
            ApplicationMsg::Quit => {
                gtk::main_quit()
            },
        }
    }

}


// ### `Widget` Implementation for Counter
impl Widget for Counter {
    type Root = gtk::Box;

    // return the root of the widget
    fn root(&self) -> Self::Root {
        self.vbox.clone()
    }

    // Create the widgets
    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let vbox = gtk::Box::new(Vertical, 0);

        let plus_button = Button::new_with_label("+");
        vbox.add(&plus_button);

        let counter_label = Label::new("0");
        vbox.add(&counter_label);

        let minus_button = Button::new_with_label("-");
        vbox.add(&minus_button);

        // Send the message Increment when the button is clicked.
        connect!(relm, plus_button, connect_clicked(_), CounterMsg::Increment);
        connect!(relm, minus_button, connect_clicked(_), CounterMsg::Decrement);

        Counter {
            counter_label: counter_label,
            model,
            vbox: vbox,
        }
    }
}

// ### `Widget` Implementation for Win
impl Widget for Win {
    type Root = Window;

    // return the root of the widget
    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    // Create the widgets
    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // GTK+ widgets are used normally within a relm `Widget`
        let window = Window::new(WindowType::Toplevel);

        let vbox = gtk::Box::new(Vertical, 0);

        // The horizontal gtk::Box hold the Counter widgets
        let hbox = gtk::Box::new(Horizontal, 0);
        vbox.add(&hbox);

        let add_button = Button::new_with_label("Add");
        vbox.add(&add_button);
        let remove_button = Button::new_with_label("Remove");
        vbox.add(&remove_button);

        window.add(&vbox);

        window.show_all();

        connect!(relm, add_button, connect_clicked(_), ApplicationMsg::AddCounter);
        connect!(relm, remove_button, connect_clicked(_), ApplicationMsg::RemoveCounter);
        connect!(relm, window, connect_delete_event(_, _), return (Some(ApplicationMsg::Quit), Inhibit(false)));

        Win {
            model,
            widgets: Widgets {
                window: window,
                hbox: hbox,
            }
        }
    }
}

impl WidgetTest for Win {
    type Widgets = Widgets;

    fn get_widgets(&self) -> Self::Widgets {
        self.widgets.clone()
    }
}


fn main() {
    Win::run(()).expect("Win::run failed");
}
