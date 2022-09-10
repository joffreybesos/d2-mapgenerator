use clap::{Arg, ArgMatches, Command, value_parser};

pub fn command_line_interface() -> ArgMatches {
    Command::new("d2-mapgenerator")
        .about("Generate d2 map images")
        .version("0.3.0")
        .arg_required_else_help(true)
        .author("joffreybesos")
        .disable_version_flag(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("generate")
            .about("Generate the images directly into your user temp folder")
            .arg(
                Arg::new("seed")
                    .short('s')
                    .long("seed")
                    .help("Seed value as decimal")
                    .takes_value(true)
                    .required(true)
                    // .allow_invalid_utf8(true)
                    // .value_parser(ValueParser::os_string())
                    .value_parser(value_parser!(u32))
            )
            .arg(
                Arg::new("difficulty")
                    .long("difficulty")
                    .short('d')
                    .help("Game difficulty 0-2, 0 = normal, 1 = nightmare, 2 = hell")
                    .takes_value(true)
                    .required(true)
                    .value_parser(value_parser!(u32).range(0..=2))
            )
            .arg(
                Arg::new("mapid")
                    .long("map")
                    .short('m')
                    .help("Map area 1-136, set to 0 or omit to generate for ALL maps")
                    .takes_value(true)
                    .default_value("0")
                    .value_parser(value_parser!(u32).range(0..=136))
            )
            .arg(
                Arg::new("pathstart")
                    .long("pathstart")
                    .help("Draw a path using pathfinding, start with this location")
                    .takes_value(true)
                    .default_value("0")
            )
            .arg(
                Arg::new("pathend")
                    .long("pathend")
                    .help("End with this location (e.g. '102' for exit 102, or '5424,5246' for world location")
                    .takes_value(true)
                    .default_value("0")
            )
            .arg(
                Arg::new("d2lod")
                    .long("d2lod")
                    .short('l')
                    .help("Diablo 2 LoD 1.13c game files (optional)")
                    .takes_value(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .default_value("./game")
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
            .arg(
                Arg::new("scale")
                    .long("scale")
                    .short('z')
                    .help("Pixel multiplier of the map image (optional)")
                    .takes_value(true)
                    .default_value("1")
                    .value_parser(value_parser!(f32))
            )
            .arg(
                Arg::new("rotate")
                    .long("rotate")
                    .short('r')
                    .help("Rotate the image 45 degrees")
                    .takes_value(false)
                    .required(false)
            )
            .arg(
                Arg::new("pathonly")
                    .long("pathonly")
                    .short('p')
                    .help("Just give me the path finding data")
                    .takes_value(false)
                    .required(false)
            )
        )
        .subcommand(Command::new("server")
            .about("Launch a REST API server to request and generate images")
            .arg(
                Arg::new("port")
                    .short('p')
                    .long("port")
                    .help("Port to use for server")
                    .takes_value(true)
                    .required(false)
                    .value_parser(value_parser!(u16).range(1..=65535))
                    .default_value("3003")
            )
            .arg(
                Arg::new("d2lod")
                    .long("d2lod")
                    .short('l')
                    .help("Diablo 2 LoD 1.13c game files (optional)")
                    .takes_value(true)
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .default_value("./game")
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
            .arg(
                Arg::new("scale")
                    .long("scale")
                    .short('z')
                    .help("Pixel multiplier of the map image (optional)")
                    .takes_value(true)
                    .default_value("1")
                    .value_parser(value_parser!(f32))
            )
        )
        .get_matches()
}