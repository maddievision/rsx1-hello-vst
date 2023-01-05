# hello_vst

Sample VST host that:
* Loads a plugin
* Loads an FXB preset into the plugin
* Constructs a sequence of MIDI events
* Sends MIDI events to the plugin while capturing its output
* Write the output to a WAV file

Included in the repo is an [MP3 file](/out/schala.mp3) of the output.

<audio src="/out/schala.mp3" controls>Your browser does not support the audio tag.</audio>

Sample console output:

```
$ cargo run
   Compiling hello_vst v0.1.0 (/Users/maddievision/code/rs/hello_vst)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
     Running `target/debug/hello_vst`
Loaded plugin: chipsynth SFC
Loading preset: data/schala.fxb
Initialized instance!
Setting up audio and event buffers
Starting MIDI event sequence and output capture
Writing output to file out/schala.wav:
Closing instance...
```
