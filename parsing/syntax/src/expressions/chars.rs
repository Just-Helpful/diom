#[derive(Clone, Debug)]
pub struct Char(pub char);

// impl DisplayAs<Spans> for Char<Range<usize>> {
//   fn write<W: Write>(&self, w: &mut SpanWriter<W>) -> std::fmt::Result {
//     w.bracket("char", &self.info)
//   }
// }
