# nif
Nafc image format

# Specifications
First 4 bytes are an identifier, these bytes are in hex
`6e 69 66 01`

1 byte width then 1 byte height, the width/height is equal the the value of the byte plus one.

1 byte for the amount of color in the palette, the amount is equal the the value plus one.
if that byte is 0 then there's (0+1)\*3, so 3 colors, if it is 255 the palette is (255+1)\*3 so 768 bytes long.

from 6th byte to 6th byte + palette length is the pal

| byte  | meaning                     | comment                                                                                              |
|-------|-----------------------------|------------------------------------------------------------------------------------------------------|
| 0-3   | File identifier             | Is always `6e 69 66 01`                                                                              |
| 4     | Width                       | width = byte + 1                                                                                     |
| 5     | Height                      | height = byte +1                                                                                     |
| 6     | Amount of colors in palette | amount of bytes = (byte + 1) * 3                                                                     |
| 6-?   | RGB colors                  | Bytes rotate between meaning R, G or B.                                                              |
| ?-End | Pixels                      | Each bytes refers to a palette value. e.g. the first color of the palette is number 0, know as 0x00. |
