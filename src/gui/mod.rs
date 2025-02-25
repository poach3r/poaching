mod css;

use crate::player::*;
use crate::scenario::*;
use crate::simulator::*;
use crate::status::*;
use gtk::prelude::*;
use log::{info, warn};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use relm4::prelude::*;
use relm4::set_global_css;

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

            gtk::Label {
                set_text: &self.player.district.to_string()
            }
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
            add_css_class: "view",
            add_css_class: "container",
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
                add_css_class: "background",
                add_css_class: "container",
                set_orientation: gtk::Orientation::Horizontal,
                set_halign: gtk::Align::Fill,
            },
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

#[tracker::track]
struct App {
    #[tracker::do_not_track]
    players: Vec<Player<'static>>,
    #[tracker::do_not_track]
    scenarios: Vec<Vec<Scenario>>,
    #[tracker::do_not_track]
    start_scenarios: Vec<Vec<Scenario>>,
    #[tracker::do_not_track]
    rng: ThreadRng,
    #[tracker::do_not_track]
    rounds: FactoryVecDeque<Round>,
    game_over: bool,
}

#[derive(Debug)]
enum AppMsg {
    AddRound,
    NewGame,
    LoadPlayers,
    LoadPlayersActually(gtk::gio::File),
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = (Vec<Player<'static>>, Vec<Vec<Scenario>>, Vec<Vec<Scenario>>);
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Hunger Games Simulator"),
            set_default_size: (600, 700),

            #[wrap(Some)]
            set_titlebar = &gtk::HeaderBar {
                #[wrap(Some)]
                set_title_widget = &gtk::Box {
                    add_css_class: "devel",
                    add_css_class: "linked",

                    gtk::Button {
                        add_css_class: "pill",
                        set_label: "Simulate Round",
                        connect_clicked => AppMsg::AddRound,
                        #[track = "model.changed_game_over()"]
                        set_sensitive: !model.game_over,
                    },

                    gtk::Button {
                        add_css_class: "pill",
                        set_label: "New Game",
                        connect_clicked => AppMsg::NewGame,
                    },

                    gtk::Button {
                        add_css_class: "pill",
                        set_label: "Load Players",
                        connect_clicked => AppMsg::LoadPlayers,
                    },
                },
            },

            gtk::ScrolledWindow {
                set_hscrollbar_policy: gtk::PolicyType::Never,
                set_hexpand: true,
                set_vexpand: true,
                add_css_class: "main",

                #[local_ref]
                rounds_box -> gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 8,
                },
            }
        }
    }

    fn update(&mut self, msg: Self::Input, sender: ComponentSender<Self>) {
        self.reset();

        match msg {
            AppMsg::AddRound => {
                info!("Adding new round.");

                self.players.shuffle(&mut self.rng);
                let amt = self.rounds.len();
                let events = if amt == 0 {
                    simulate_round(&mut self.players, &self.start_scenarios, &mut self.rng)
                } else {
                    simulate_round(&mut self.players, &self.scenarios, &mut self.rng)
                };

                let mut players = self.players.clone();
                players.sort_by(|x, y| x.district.cmp(&y.district).reverse());

                self.set_game_over(events.1);
                self.rounds.guard().push_back((amt + 1, events.0, players));
            }
            AppMsg::NewGame => {
                info!("Starting new game.");

                for player in self.players.iter_mut() {
                    player.heal();
                    player.inventory.clear();
                }

                self.rounds.guard().clear();
                self.set_game_over(false);
            }
            AppMsg::LoadPlayers => {
                let f = gtk::FileChooserDialog::builder()
                    .title("Open File")
                    .action(gtk::FileChooserAction::Open)
                    .decorated(true)
                    .build();

                f.add_button("Open", gtk::ResponseType::Accept);
                f.connect_response(
                    move |chooser: &gtk::FileChooserDialog, response: gtk::ResponseType| {
                        if let None = chooser.file() {
                            warn!("User attempted to load players file, but none was found.");
                            chooser.destroy();
                            return;
                        }

                        if let gtk::ResponseType::Accept = response {
                            sender.input(AppMsg::LoadPlayersActually(chooser.file().unwrap()));
                            chooser.destroy();
                        }
                    },
                );
                f.show();
            }
            AppMsg::LoadPlayersActually(file) => {
                if let Some(players) = crate::load_players_path(file.path().unwrap()) {
                    self.players = players;
                    self.set_game_over(true); // prevents the user from continuing the current game with the new players
                } else {
                    let message = gtk::MessageDialog::builder()
                        .title("Error")
                        .text("Failed to load players file.")
                        .buttons(gtk::ButtonsType::Ok)
                        .build();

                    message.connect_response(|message, response| {
                        if let gtk::ResponseType::Ok = response {
                            message.destroy();
                        }
                    });
                    message.show();
                }
            }
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
            game_over: false,
            tracker: 0,
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
    set_global_css(&css::style());
    app.run::<App>((players, scenarios, start_scenarios));
}
