// macros
//@bytes_macro
#[macro_export]
macro_rules! bytes {
    ($name: ident) => {
        impl $name {
            pub fn to_bytes(&self) -> Vec<u8> {
                serialize(&self).unwrap()
            }

            pub fn from_bytes(data: Vec<u8>) -> Self {
                deserialize(&data[..]).unwrap()
            }
        }
    }
}

//@deref_macro
#[macro_export]
macro_rules! deref {
    ($name: ident, $target: ident) => {
        impl Deref for $name {
            type Target = $target;
    
            fn deref(&self) -> &Self::Target { &self.0 }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
        }
    }
}