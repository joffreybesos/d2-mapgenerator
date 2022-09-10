# Diablo 2 map image generator

This tool is a rewrite of [d2-mapserver](https://github.com/joffreybesos/d2-mapserver) project.

It can be used in two modes:
    - as a CLI tool to generate images files
    - as a REST API server (like the old map server)

It still uses blacha's map generation tool to extract data from D2LoD.  

You MUST have your d2lod files in a subfolder called `d2lod`

For the CLI tool all image files are generated in your windows user temp folder.

## Usage

```bash
USAGE:
    d2-mapgenerator.exe <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    generate    Generate the images directly into your user temp folder
    help        Print this message or the help of the given subcommand(s)
    server      Launch a REST API server to request and generate images
```

### Generate images

```bash
USAGE:
    d2-mapgenerator.exe generate [OPTIONS] --seed <seed> --difficulty <difficulty>

OPTIONS:
    -b, --blachaexe <blachaexe>      Location of d2-mapgen.exe (optional) [default:
                                     ./mapgen/d2-mapgen.exe]
    -d, --difficulty <difficulty>    Game difficulty 0-2, 0 = normal, 1 = nightmare, 2 = hell
    -h, --help                       Print help information
    -l, --d2lod <d2lod>              Diablo 2 LoD 1.13c game files (optional) [default: ./game]
    -m, --map <mapid>                Map area 1-136, set to 0 or omit to generate for ALL maps
                                     [default: 0]
    -r, --rotate                     Rotate the image 45 degrees
    -s, --seed <seed>                Seed value as decimal
    -z, --scale <scale>              Pixel multiplier of the map image (optional) [default: 1]
```

### REST Server

```bash
USAGE:
    d2-mapgenerator.exe server [OPTIONS]

OPTIONS:
    -b, --blachaexe <blachaexe>    Location of d2-mapgen.exe (optional) [default:
                                   ./mapgen/d2-mapgen.exe]
    -h, --help                     Print help information
    -l, --d2lod <d2lod>            Diablo 2 LoD 1.13c game files (optional) [default: ./game]
    -p, --port <port>              Port to use for server [default: 3003]
    -z, --scale <scale>            Pixel multiplier of the map image (optional) [default: 1]
```