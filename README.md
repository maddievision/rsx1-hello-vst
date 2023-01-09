# hello_vst

Sample VST instrument host that:
* Loads two instances of a chipsynth SFC with separate FXP preset files 
* Displays the editors for the instances
* Loads a MIDI file
* Routes the MIDI file events to the appropriate instances
* Mixes the audio from both instances and plays in realtime

Note that [chipsynth SFC](https://www.plogue.com/products/chipsynth-sfc.html) is required to run this as-is, but you can stub out the path for a different VST instrument.

Video example: https://streamable.com/f98brn

The MIDI file is a sequence of Schala's Theme from Chrono Trigger, and the chipsynth SFC instance presets are loaded with samples from the original song.

![Preview](/preview.png)
