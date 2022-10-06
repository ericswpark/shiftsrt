# shiftsrt

A little program to shift .srt subtitle files by an offset

## Disclaimer

This is my first Rust project. While it should not touch your original subtitle file, always make a backup before running this tool.

If you have any suggestions or PRs to improve this project, please feel free to send them!

## Usage

```
./shiftsrt <full path to .srt file> <offset in milliseconds>
```

### Example 1

Shift the file `Blade.Runner.2049.srt` by 3 seconds (3000 milliseconds):

```
./shiftsrt ./Blade.Runner.2049.srt 3000
```

### Example 2

Shift the file `Up.srt` back by 7 seconds (7000 milliseconds):

```
./shiftsrt ./Up.srt -7000
```


