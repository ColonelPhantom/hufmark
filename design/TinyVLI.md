# TinyVLI
TinyVLI is a way to store variable-length integers from 0-255. (or maybe more, but the design is 0-255).

## Huffman trees
Just look at likely probablities and create some sort of Huffman tree based on it

## Smart encoding
### Two bytes
Use the first two bytes as an indicator
- 00 means 0
- 01 means a small number (1-5) 0100 up to 0111
- 10 means a larger number (6-70) 10......
- 11 means a full number (71-255) 11........
### Full nibble
- 0x0 - 0x7 -> literal
- 0x8 - 0xE -> full byte, signal&0x7 << 4 + data_nibble
- 0xF -> next nibble indicates length in 
#### example encodings
##### 0x0 - 0x8 -> literal
- 0x5 -> 0x5
#### 0x08 - 0x6F -> add 0x80
- 0x8 -> 0x88
- 0x10 -> 0x90
- 0x25 -> 0x95
- 0x6F -> 0xEF
###### 0x70 - 0x806F (32879) -> substract 0x70, read length, store
- 0x70 -> 0xF0
- 0x71 -> 0xF1(0b1)
- 
