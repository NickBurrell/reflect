#![feature(const_fn)]
#![allow(dead_code)]
macro_rules! impl_reflect {
    (
        struct $s_name:ident {
           $($v_name:ident: $v_type:ty),*
        }
    ) => {
        struct $s_name {
            $($v_name: $v_type),*
        }

        impl $s_name {

            const fn get_members_impl() -> &'static str {
                concat!(
                    "struct ", stringify!($s_name), " {\n",
                    $("\t", stringify!($v_name), ": ", stringify!($v_type), ",\n"),*,
                    "}")
            }
            pub fn get_members(&self) -> String {
                $s_name::get_members_impl().to_owned()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    impl_reflect! {
        struct Point {
            x: i32,
            y: i32
        }
    }

    #[test]
    fn test_get_members() {
        let new_point = Point {x: 1, y: 4};
        println!("{}", new_point.get_members());
        assert_eq!(Point::get_members_impl().to_owned(), "struct Point {
	x: i32,
	y: i32,
}", "ERROR");
    }
}
