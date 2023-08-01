mod font_cache;
mod to_path;

use crate::application_io::EditorApi;
pub use font_cache::*;
use node_macro::node_fn;
pub use to_path::*;

use crate::Node;

pub struct TextGenerator<Text, FontName, Size> {
	text: Text,
	font_name: FontName,
	font_size: Size,
}

#[node_fn(TextGenerator)]
fn generate_text<'a: 'input, T>(editor: EditorApi<'a, T>, text: String, font_name: Font, font_size: f32) -> crate::vector::VectorData {
	let buzz_face = editor.font_cache.get(&font_name).map(|data| load_face(data));
	crate::vector::VectorData::from_subpaths(to_path(&text, buzz_face, font_size as f64, None))
}