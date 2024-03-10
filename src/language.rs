pub struct Translations {
    //=====# main.rs translations #=====//
    //panic text
    pub media_name_len_panic_text: &'static str,
    pub media_name_is_empty_panic_text: &'static str,
    //misc text
    pub no_choice_misc_text: &'static str,

    //=====# temp_dir.rs translations #=====//
    //expect text
    pub create_temp_dir_expect: &'static str,
    pub remove_temp_dir_expect: &'static str,

    //=====# posters.rs translations #=====//
    //err text
    pub msg_err: &'static str,
    pub reading_err: &'static str,
    pub downloading_err: &'static str,

    //=====# choose_episode.rs translations #=====//
    //misc text
    pub total_episode_misc_text: &'static str,
    pub select_episode_misc_text: &'static str,
    //err text
    pub choose_episode_err: &'static str,

    //=====# choose_lang.rs translations #=====//
    //misc text
    pub select_lang_misc_text: &'static str,
    //err text
    pub choose_lang_err: &'static str,

    //=====# choose_media.rs translations #=====//
    //misc text
    pub total_media_misc_text: &'static str,
    pub select_media_misc_text: &'static str,
    //err text
    pub choose_media_err: &'static str,

    //=====# choose_season.rs translations #=====//
    //misc text
    pub total_season_misc_text: &'static str,
    pub select_season_misc_text: &'static str,
    //err text
    pub choose_season_err: &'static str,

    //=====# get_medias.rs translations #=====//
    //expect
    pub response_expect: &'static str,

    //=====# vlc.rs translations #=====//
    //misc text
    pub vlc_start_misc_text: &'static str,
    pub vlc_exit_misc_text: &'static str,
    // err text
    pub vlc_exit_with_err: &'static str,
    pub vlc_wait_err: &'static str,
    pub vlc_start_err: &'static str,

    //=====# watch_media.rs translations #=====//
    //misc text
    pub preparing_misc_text: &'static str,
    pub getting_episodes_misc_text: &'static str,
    pub getting_language_misc_text: &'static str,
    pub fetching_misc_text: &'static str,
    //err text
    pub click_season_err: &'static str,
    pub click_episode_err: &'static str,
    //expect text
    pub language_option_expect: &'static str,
}

pub const fn portuguese() -> Translations {
    let portuguese_translation = Translations {
        //=====# main.rs translations #=====//
        //panic text
        media_name_len_panic_text: "Desculpe, sua pesquisa precisa ter no mínimo 4 caracteres",
        media_name_is_empty_panic_text: "Não conseguimos achar nada com sua pesquisa",
        //misc text
        no_choice_misc_text: "Nenhuma Escolha?",

        //=====# temp_dir.rs translations #=====//
        //expect
        create_temp_dir_expect: "Não foi possível criar o diretório temporário!",
        remove_temp_dir_expect: "Não foi possível remover o diretório temporário!",

        //=====# posters.rs translations #=====//
        //err text
        msg_err: "Não foi possível criar a imagem em",
        reading_err: "Erro Lendo",
        downloading_err: "Erro Baixando",

        //=====# choose_episode.rs translations #=====//
        //misc text
        total_episode_misc_text: "Total de episódio para assistir:",
        select_episode_misc_text: "Selecione o episódio que você quer assistir:",
        //err text
        choose_episode_err: "Aconteceu um erro, por favor, tente novamente",

        //=====# choose_lang.rs translations #=====//
        //misc text
        select_lang_misc_text: "Selecione a opção de linguagem:",
        //err text
        choose_lang_err: "Aconteceu um erro, por favor, tente novamente",

        //=====# choose_media.rs translations #=====//
        //misc text
        total_media_misc_text: "Total de mídia(s) para assistir:",
        select_media_misc_text: "Selecione o que você quer assistir:",
        //err text
        choose_media_err: "Aconteceu um erro, por favor, tente novamente",

        //=====# choose_season.rs translations #=====//
        //misc text
        total_season_misc_text: "Total de temporadas para assistir:",
        select_season_misc_text: "Selecione a temporada que você quer assistir",
        //err text
        choose_season_err: "Aconteceu um erro, por favor, tente novamente",

        //=====# get_medias.rs translations #=====//
        //expect
        response_expect: "Não foi possível carregar a URL",

        //=====# vlc.rs translations #=====//
        //misc text
        vlc_start_misc_text: "Iniciando o reprodutor de mídia",
        vlc_exit_misc_text: "Tchau Tchau (^_^)/",
        // err text
        vlc_exit_with_err: "VLC fechou com um erro:",
        vlc_wait_err: "Falha ao esperar por VLC:",
        vlc_start_err: "Falha ao iniciar o VLC:",

        //=====# watch_media.rs translations #=====//
        //misc text
        preparing_misc_text: "Preparando tudo, pode demorar",
        getting_episodes_misc_text: "Adquirindo episódios",
        getting_language_misc_text: "Adquirindo opções de linguagens",
        fetching_misc_text: "Buscando serviço",
        //err text
        click_season_err: "Erro: Não é possível clicar na temporada",
        click_episode_err: "Erro: Não é possível clicar no episódio",
        //expect text
        language_option_expect: "Não foi possível adquirir opções de linguagens.",
    };

    portuguese_translation
}

pub const fn english() -> Translations {
    let english_translation = Translations {
        //=====# main.rs translations #=====//
        //panic text
        media_name_len_panic_text: "Sorry, your query needs to be at least 4 characters",
        media_name_is_empty_panic_text: "Couldn't find anything with your query",
        //misc text
        no_choice_misc_text: "No Choice?",

        //=====# temp_dir.rs translations #=====//
        //expect text
        create_temp_dir_expect: "Couldn't create the temporary directory!",
        remove_temp_dir_expect: "Couldn't remove the temporary directory!",

        //=====# posters.rs translations #=====//
        //err text
        msg_err: "Couldn't create image in",
        reading_err: "Error Reading",
        downloading_err: "Error Downloading",

        //=====# choose_episode.rs translations #=====//
        //misc text
        total_episode_misc_text: "Total of episodes to watch:",
        select_episode_misc_text: "Select the episode you want watch:",
        //err text
        choose_episode_err: "There was an error, please try again",

        //=====# choose_lang.rs translations #=====//
        //misc text
        select_lang_misc_text: "Select the language option:",
        //err text
        choose_lang_err: "There was an error, please try again",

        //=====# choose_media.rs translations #=====//
        //misc text
        total_media_misc_text: "Total of media to watch:",
        select_media_misc_text: "Select what you want to watch:",
        //err text
        choose_media_err: "There was an error, please try again",

        //=====# choose_season.rs translations #=====//
        //misc text
        total_season_misc_text: "Total of seasons to watch:",
        select_season_misc_text: "Select the season you want watch:",
        //err text
        choose_season_err: "There was an error, please try again",

        //=====# get_medias.rs translations #=====//
        //expect
        response_expect: "Could not load url.",

        //=====# vlc.rs translations #=====//
        //misc text
        vlc_start_misc_text: "Starting the player",
        vlc_exit_misc_text: "Bye bye (^_^)/",
        // err text
        vlc_exit_with_err: "VLC exited with an error:",
        vlc_wait_err: "Failed to wait for VLC:",
        vlc_start_err: "Failed to start VLC:",

        //=====# watch_media.rs translations #=====//
        //misc text
        preparing_misc_text: "Preparing everything, which can take a while",
        getting_episodes_misc_text: "Getting episodes",
        getting_language_misc_text: "Getting languages options",
        fetching_misc_text: "Fetching service",
        //err text
        click_season_err: "Error: Can't click on the season",
        click_episode_err: "Error: Can't click on the episode",
        //expect text
        language_option_expect: "Couldn't retrieve languages options.",
    };

    english_translation
}
