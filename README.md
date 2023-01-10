# hello_vst

Sample VST instrument host that can load a project file defining VST instruments and a MIDI file:
* Specified VST instruments are loaded and instantiated.
* Specified presets for the VST instruments are lodaded 
* All VST instrument editor windows are shown
* Load MIDI file and routes events to the appropriate VST instruments
* Mixes the audio across all VST instances and plays in realtime

Note that [chipsynth SFC](https://www.plogue.com/products/chipsynth-sfc.html) is required to run this as-is, but you can stub out the path for a different VST instrument.

Video example: https://streamable.com/f98brn

![Preview](/preview.png)
