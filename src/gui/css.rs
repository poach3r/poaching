use log::info;

pub struct Style<'a> {
    pub bg1: &'a str,
    pub bg2: &'a str,
    pub bg3: &'a str,
    pub fg: &'a str,
}

pub const DARK: Style = Style {
    bg1: "161616",
    bg2: "191919",
    bg3: "262626",
    fg: "f2f4f8",
};

pub const LIGHT: Style = Style {
    bg1: "ffffff",
    bg2: "f2f2f2",
    bg3: "f2f4f8",
    fg: "393939",
};

pub fn enable_light_mode() {
    info!("Enabling light mode.");
    relm4::set_global_css(LIGHT.get_css().as_str());
}

pub fn enable_dark_mode() {
    info!("Enabling dark mode.");
    relm4::set_global_css(DARK.get_css().as_str());
}

impl<'a> Style<'a> {
    pub fn get_css(&self) -> String {
        format!(
            "
        label {{
            color: #{};
        }}

        button {{
            background-color: #{};
            border: 0 solid transparent;
            background-image: none;
        }}

        .main {{
            background-color: #{};
            padding: 8px;
        }}

        .round {{
            padding: 8px;
            background-color: #{};
            border-radius: 8px;
        }}

        .statuses {{
            padding: 8px;
            background-color: #{};
            border-radius: 8px;
        }}
        ",
            self.fg, self.bg3, self.bg1, self.bg3, self.bg2
        )
    }
}
