pub fn is_desktop() -> bool {
    #[cfg(feature = "web")]
    {
        false
    }
    #[cfg(not(feature = "web"))]
    {
        true
    }
}
