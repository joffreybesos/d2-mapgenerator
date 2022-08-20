use clap::{Arg, ArgMatches, Command, builder::ValueParser};


pub fn command_line_interface() -> ArgMatches {
    Command::new("generate")
        .about("Generate d2 map images")
        .version("1.0.0")
        .arg_required_else_help(true)
        .author("joffreybesos")
        .disable_version_flag(true)
        .arg(
            Arg::new("seed")
                .short('s')
                .long("seed")
                .help("Seed value as decimal")
                .takes_value(true)
                .required(true)
                // .allow_invalid_utf8(true)
                .value_parser(ValueParser::os_string())
        )
        .arg(
            Arg::new("difficulty")
                .long("difficulty")
                .short('d')
                .help("Game difficulty 0-2, 0 = normal, 1 = nightmare, 2 = hell")
                .takes_value(true)
                .required(true)
                .value_parser(ValueParser::os_string())
        )
        .arg(
            Arg::new("mapid")
                .long("map")
                .short('m')
                .help("Map area 1-136")
                .takes_value(true)
                .required(true)
                .value_parser(ValueParser::os_string())
        )
        .arg(
            Arg::new("d2lod")
                .long("d2lod")
                .short('l')
                .help("Diablo 2 LoD 1.13c game files (optional)")
                .takes_value(true)
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .default_value("./d2lod")
        )
        .arg(
            Arg::new("blachaexe")
                .long("blachaexe")
                .short('b')
                .help("Location of d2-mapgen.exe (optional)")
                .takes_value(true)
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .default_value("./mapgen/d2-mapgen.exe")
        )
        .get_matches()
}