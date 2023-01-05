use vst::host::Host;

pub struct SampleHost;

impl Host for SampleHost {
    fn automate(&self, _index: i32, _value: f32) {
        // no op
    }
}
