# Reverser
Gadget to reverse binary files before you upload them to Github. Easy to use.

## Usage

To encode file in a folder: (recursively reverse all binary files in the current folder and subfolders)
```
reverser --enc
```

To decode them:
```
reverser --dec
```
Or just double-click to run, the program is considered to be in decoding mode by default.

## Notes

This program was originally designed to reverse images to be uploaded to Github, there are special optimizations for image files, which is evident in the source code.

These parts of design are:

1. It will only monitor files ending with a specific extension.
2. It will read the header of the file, and when a file is considered to be already reversed, it will not flip this file again if it is executed in `--enc` mode, and vice versa.
3. If you need to monitor other types of files, please modify the source code yourself.
