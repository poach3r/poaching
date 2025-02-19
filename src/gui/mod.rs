use crate::player::Player;
use crate::status::*;
use gtk::prelude::*;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use relm4::prelude::*;

#[derive(Debug)]
struct PlayerWrapper {
    player: Player<'static>,
}

#[relm4::factory]
impl FactoryComponent for PlayerWrapper {
    type Init = crate::player::Player<'static>;
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
    type Init = (usize, String, Vec<crate::player::Player<'static>>);
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::Box;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 4,

            gtk::Label {
                set_label: &self.number.to_string().as_str(),
            },

            #[name(label)]
            gtk::Label {
                set_justify: gtk::Justification::Center,
                set_label: &self.events,
            },

            self.players.widget() -> &gtk::FlowBox {
                set_orientation: gtk::Orientation::Horizontal,
                set_halign: gtk::Align::Fill
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
    players: Vec<crate::player::Player<'static>>,
    scenarios: Vec<Vec<crate::scenario::Scenario>>,
    rng: ThreadRng,
    rounds: FactoryVecDeque<Round>,
}

#[derive(Debug)]
enum AppMsg {
    AddRound,
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = (
        Vec<crate::player::Player<'static>>,
        Vec<Vec<crate::scenario::Scenario>>,
    );
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Hunger Games Simulator"),
            set_default_size: (300, 100),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                gtk::Button {
                    set_label: "Simulate Round",
                    connect_clicked => AppMsg::AddRound,
                },

                gtk::ScrolledWindow {
                    set_hscrollbar_policy: gtk::PolicyType::Never,
                    set_hexpand: true,
                    set_vexpand: true,

                    #[local_ref]
                    rounds_box -> gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 18,
                    }
                }
            }
        }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::AddRound => {
                let amt = self.rounds.len();
                self.players.shuffle(&mut self.rng);
                let events = crate::simulator::simulate_round(
                    &mut self.players,
                    &self.scenarios,
                    &mut self.rng,
                );
                self.rounds
                    .guard()
                    .push_back((amt + 1, events, self.players.clone()));
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
            rng: rand::rng(),
            rounds,
        };

        let rounds_box = model.rounds.widget();
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

pub fn run(
    players: Vec<crate::player::Player<'static>>,
    scenarios: Vec<Vec<crate::scenario::Scenario>>,
) {
    let app = RelmApp::new("relm4.example.factory");
    app.run::<App>((players, scenarios));
}
