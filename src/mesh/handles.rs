use std::convert::From;

const UNSET_VALUE: i32 = -1;

/// BaseHandle struct
pub struct Handle {
    index: i32,
}

impl Handle {
    pub const fn index(&self) -> i32 {
        self.index
    }

    pub const fn is_valid(&self) -> bool {
        self.index >= 0
    }

    pub fn reset(&mut self) {
        self.index = UNSET_VALUE
    }

    pub fn invalidate(&mut self) {
        self.index = UNSET_VALUE
    }

    pub fn increment(&mut self) {
        self.index += 1
    }

    pub fn decrement(&mut self) {
        self.index -= 1
    }

    pub fn increment_by(&mut self, amount: i32) {
        self.index += amount
    }

    pub fn decrement_by(&mut self, amount: i32) {
        self.index -= amount
    }
}

impl Default for Handle {
    fn default() -> Self {Handle{index: UNSET_VALUE}}
}

// Individual Handle types constructed via NewType pattern
pub struct VertexHandle(Handle);
pub struct HalfedgeHandle(Handle);
pub struct EdgeHandle(Handle);
pub struct FaceHandle(Handle);
pub struct MeshHandle(Handle);

#[cfg(test)]
mod tests {
    use crate::mesh::handles::*;

    #[test]
    fn handle_creation() {
        // Arrange

        // Act
        // let handle = TypedHandle::VertexHandle(Handle::default());
        let handle = VertexHandle(Handle::default());

        assert_eq!(handle.0.index(), -1);
    }
}