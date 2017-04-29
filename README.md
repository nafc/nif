# nif
Nafc image format

# Specifications
First 3 bytes are an identifier, these bytes are in hex
`6e 69 66`

1 byte width then 1 byte height, the width/height is equal the the value of the byte plus one.

The following n bytes, n being defined as width*height, are pixels referring to the palette.

Then the palette is the rest of the file.

| byte  | meaning         | comment                                                                                                              |
|-------|-----------------|----------------------------------------------------------------------------------------------------------------------|
| 0-2   | File identifier | always `6e 69 66`                                                                                                    |
| 3     | Width           | width = value + 1                                                                                                    |
| 4     | Height          | height = value +1                                                                                                    |
| 5-?   | Pixels          | Each bytes refers to a palette value. e.g. the first color in the palette is value 0.                                |
| ?-end | Palette         | The length of this palette has to be a multiple of 3 because one color is 3 bytes. e. g. every 3 bytes is one color. |
