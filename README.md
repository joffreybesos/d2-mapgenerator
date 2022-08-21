# Diablo 2 map image generator

I've written the [d2-mapserver](https://github.com/joffreybesos/d2-mapserver) project as a CLI tool.

It still uses blacha's map generation tool to extract data from D2LoD.  
You also need the d2lod files in a subfolder called `d2lod`

## Usage

    d2-mapserver-rust.exe [OPTIONS] --seed <seed> --difficulty <difficulty>

    -s, --seed <seed>                Seed value as decimal
    -d, --difficulty <difficulty>    Game difficulty 0-2, 0 = normal, 1 = nightmare, 2 = hell
    -m, --map <mapid>                Map area 1-136, set to 0 or omit to generate for ALL maps
                                     
    -b, --blachaexe <blachaexe>      Location of d2-mapgen.exe (optional) [default: ./mapgen/d2-mapgen.exe]
    -h, --help                       Print help information
    -l, --d2lod <d2lod>              Diablo 2 LoD 1.13c game files (optional) [default: ./d2lod]
    -z, --scale <scale>              Pixel multiplier of the map image (optional) [default: 1]