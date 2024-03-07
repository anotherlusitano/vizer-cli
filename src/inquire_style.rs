use inquire::ui::{Color, RenderConfig, StyleSheet, Styled};

pub fn set_inquire_style() {
    let default: RenderConfig = RenderConfig::default();
    let selected = StyleSheet::default();

    let prompt_prefix = Styled::new("?").with_fg(Color::LightYellow);

    let something = default.with_prompt_prefix(prompt_prefix);

    let selected_option =
        something.with_selected_option(Some(selected.with_fg(Color::LightYellow)));

    let option_prefix = selected_option
        .with_highlighted_option_prefix(Styled::new(">").with_fg(Color::LightYellow));

    let help_message = option_prefix.with_help_message(selected.with_fg(Color::LightYellow));

    inquire::set_global_render_config(help_message);
}
