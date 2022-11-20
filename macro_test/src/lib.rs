use cloneem::clone;

#[derive(Clone)]
struct N1 { pub x: u32 }
#[derive(Clone)]
struct N2 { pub a: N1 }
#[derive(Clone)]
struct N3 { pub b: N2 }

fn basic_clone() {
    struct A { field: i32 }

    impl A {
        pub fn operate(&self) {
            let stuff = N3 { b: N2 { a: N1 { x: 321 } } };
            let local = std::rc::Rc::new(42);
            let pair = (5,5);
            clone!(self.field, stuff.b.a.x, something = local, elem = pair.1);
            assert_eq!(field, 5);
            assert_eq!(x, 321);
            assert_eq!(*something, 42);
            assert_eq!(elem, 5);
        }
    }
}