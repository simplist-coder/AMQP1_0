/// # Handle
/// The handle of a Link.
/// ```xml
/// <type name="handle" class="restricted" source="uint"/>
/// ```
/// An alias established by the attach frame and subsequently used by endpoints as a shorthand to refer
/// to the Link in all outgoing frames. The two endpoints may potentially use different handles to refer
/// to the same Link. Link handles may be reused once a Link is closed for both send and receive.
pub type Handle = u32;
