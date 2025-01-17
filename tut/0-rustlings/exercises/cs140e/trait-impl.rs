// FIXME: Make me pass! Diff budget: 25 lines.

#[derive(Debug)]
pub enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

// What traits does `Duration` need to implement?
impl PartialEq for Duration {
     fn eq(&self, other: &Duration) -> bool {
     	match (self, other) {
	      (&Duration::MilliSeconds(A), &Duration::MilliSeconds(B)) => A == B,

	      (&Duration::Seconds(A), &Duration::Seconds(B)) => A == B,

	      (&Duration::Minutes(A), &Duration::Minutes(B)) => A == B,
	
	      (&Duration::MilliSeconds(A), &Duration::Seconds(B)) | (&Duration::Seconds(B), &Duration::MilliSeconds(A)) => A == (B as u64) * 1000,

	      (&Duration::MilliSeconds(A), &Duration::Minutes(B)) | (&Duration::Minutes(B), &Duration::MilliSeconds(A)) => A == (B as u64) * 60 * 1000,

	      (&Duration::Seconds(A), &Duration::Minutes(B)) | (&Duration::Minutes(B), &Duration::Seconds(A)) => A == (B as u32) * 60,
	}
     }
}
#[test]
fn traits() {
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}
