//! A Not working Window
/// This was only a test to see if I could use .build() with piston on this BullshitWindow

/// A useless Window
struct BulshitWindow {}

impl BuildFromWindowSettings for BulshitWindow {
    fn build_from_window_settings(settings: &WindowSettings) -> Result<Self, Box<dyn Error>> {
        println!("Nice!");
        let test: BulshitWindow = BulshitWindow {};
        return Ok(test);
    }
}
