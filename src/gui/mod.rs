mod css;

use crate::player::*;
use crate::scenario::*;
use crate::simulator::*;
use crate::status::*;
use gtk::prelude::*;
use log::{error, info};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use relm4::prelude::*;

#[derive(Debug)]
struct PlayerWrapper {
    player: Player<'static>,
}

#[relm4::factory]
impl FactoryComponent for PlayerWrapper {
    type Init = Player<'static>;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::FlowBox;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            gtk::Label {
                set_text: self.player.name.as_str(),
            },

            gtk::Label {
                set_text: match self.player.status {
                    Status::Alive(AliveStatus::Injured) => "Injured",
                    Status::Alive(AliveStatus::Healthy) => "Healthy",
                    _ => "Dead"
                }
            },
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            _ => (),
        }
    }

    fn init_model(player: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        PlayerWrapper { player }
    }
}

#[derive(Debug)]
struct Round {
    number: usize,
    events: String,
    players: FactoryVecDeque<PlayerWrapper>,
}

#[relm4::factory]
impl FactoryComponent for Round {
    type Init = (usize, String, Vec<Player<'static>>);
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            add_css_class: "round",
            set_spacing: 8,

            gtk::Box {
                set_spacing: 2,
                set_orientation: gtk::Orientation::Horizontal,
                set_halign: gtk::Align::Fill,
                set_valign: gtk::Align::Fill,

                gtk::Box {
                    set_halign: gtk::Align::Start,
                    set_valign: gtk::Align::Start,
                    gtk::Label {
                        set_label: &self.number.to_string().as_str(),
                    },
                },

                gtk::Box {
                    set_halign: gtk::Align::Center,
                    set_hexpand: true,
                    gtk::Label {
                        set_justify: gtk::Justification::Center,
                        set_label: &self.events,
                    },
                },
            },
            self.players.widget() -> &gtk::FlowBox {
                add_css_class: "statuses",
                set_orientation: gtk::Orientation::Horizontal,
                set_halign: gtk::Align::Fill,
            },
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            _ => (),
        }
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        let mut players = FactoryVecDeque::builder()
            .launch(gtk::FlowBox::default())
            .detach();

        for player in value.2.clone() {
            players.guard().push_front(player);
        }

        Self {
            number: value.0,
            events: value.1,
            players,
        }
    }
}

struct App {
    players: Vec<Player<'static>>,
    scenarios: Vec<Vec<Scenario>>,
    start_scenarios: Vec<Vec<Scenario>>,
    rng: ThreadRng,
    rounds: FactoryVecDeque<Round>,
}

#[derive(Debug)]
enum AppMsg {
    AddRound,
    NewGame,
    LoadPlayers,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = (Vec<Player<'static>>, Vec<Vec<Scenario>>, Vec<Vec<Scenario>>);
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Hunger Games Simulator"),
            set_default_size: (300, 100),

            gtk::Box {
                set_spacing: 8,
                add_css_class: "main",
                set_orientation: gtk::Orientation::Vertical,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 8,
                    set_halign: gtk::Align::Fill,
                    set_hexpand: true,

                    gtk::Button {
                        set_label: "Simulate Round",
                        connect_clicked => AppMsg::AddRound,
                        set_hexpand: true,
                    },

                    gtk::Button {
                        set_label: "New Game",
                        connect_clicked => AppMsg::NewGame,
                        set_hexpand: true,
                    },

                    gtk::Button {
                        set_label: "Load Players", // TODO
                        connect_clicked => AppMsg::LoadPlayers,
                        set_hexpand: true,
                    },
                },

                gtk::ScrolledWindow {
                    set_hscrollbar_policy: gtk::PolicyType::Never,
                    set_hexpand: true,
                    set_vexpand: true,

                    #[local_ref]
                    rounds_box -> gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 8,
                    },
                }
            }
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::AddRound => {
                info!("Adding new round.");
                let amt = self.rounds.len();
                self.players.shuffle(&mut self.rng);
                let events = if amt == 0 {
                    simulate_round(&mut self.players, &self.start_scenarios, &mut self.rng)
                } else {
                    simulate_round(&mut self.players, &self.scenarios, &mut self.rng)
                };
                self.rounds
                    .guard()
                    .push_back((amt + 1, events, self.players.clone()));
            }
            AppMsg::NewGame => {
                info!("Starting new game.");
                for player in self.players.iter_mut() {
                    player.heal();
                }
                self.rounds.guard().clear();
            }
            _ => (),
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let rounds = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .detach();

        let model = App {
            players: init.0,
            scenarios: init.1,
            start_scenarios: init.2,
            rng: rand::rng(),
            rounds,
        };

        let rounds_box = model.rounds.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

pub fn run(
    players: Vec<Player<'static>>,
    scenarios: Vec<Vec<Scenario>>,
    start_scenarios: Vec<Vec<Scenario>>,
    gtk_options: Vec<String>,
) {
    let app = RelmApp::new("org.poach3r.hunger_games").with_args(gtk_options);
    relm4::set_global_css(css::STYLE);
    app.run::<App>((players, scenarios, start_scenarios));
}
