use std::sync::Arc;

struct PanicWrapper {
    data: Vec<u8>
}

impl AsRef<[u8]> for PanicWrapper {
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl Drop for PanicWrapper {
    fn drop(&mut self) {
        panic!("food");
    }
}

fn main() {
    let p = Arc::new(PanicWrapper { data: vec![0, 3, 4] });
    let d = core_graphics::data_provider::CGDataProvider::from_buffer(p);
    drop(d);
        println!("Hello, world!");
}
