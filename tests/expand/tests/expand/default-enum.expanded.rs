use pin_project_lite::pin_project;
enum Enum<T, U> {
    Struct { pinned: T, unpinned: U },
    Unit,
}
#[allow(dead_code)]
#[allow(single_use_lifetimes)]
#[allow(clippy::unknown_clippy_lints)]
#[allow(clippy::mut_mut)]
#[allow(clippy::redundant_pub_crate)]
#[allow(clippy::ref_option_ref)]
#[allow(clippy::type_repetition_in_bounds)]
enum EnumProj<'__pin, T, U>
where
    Enum<T, U>: '__pin,
{
    Struct {
        pinned: ::pin_project_lite::__private::Pin<&'__pin mut (T)>,
        unpinned: &'__pin mut (U),
    },
    Unit,
}
#[allow(dead_code)]
#[allow(single_use_lifetimes)]
#[allow(clippy::unknown_clippy_lints)]
#[allow(clippy::mut_mut)]
#[allow(clippy::redundant_pub_crate)]
#[allow(clippy::ref_option_ref)]
#[allow(clippy::type_repetition_in_bounds)]
enum EnumProjRef<'__pin, T, U>
where
    Enum<T, U>: '__pin,
{
    Struct {
        pinned: ::pin_project_lite::__private::Pin<&'__pin (T)>,
        unpinned: &'__pin (U),
    },
    Unit,
}
#[allow(dead_code)]
#[allow(single_use_lifetimes)]
#[allow(clippy::mut_mut)]
#[allow(clippy::redundant_pub_crate)]
#[allow(clippy::type_repetition_in_bounds)]
enum EnumProjReplace<T, U> {
    Struct {
        pinned: ::pin_project_lite::__private::PhantomData<T>,
        unpinned: U,
    },
    Unit,
}
#[allow(single_use_lifetimes)]
#[allow(clippy::unknown_clippy_lints)]
#[allow(clippy::used_underscore_binding)]
const _: () = {
    impl<T, U> Enum<T, U> {
        fn project<'__pin>(
            self: ::pin_project_lite::__private::Pin<&'__pin mut Self>,
        ) -> EnumProj<'__pin, T, U> {
            unsafe {
                match self.get_unchecked_mut() {
                    Self::Struct { pinned, unpinned } => EnumProj::Struct {
                        pinned: ::pin_project_lite::__private::Pin::new_unchecked(pinned),
                        unpinned: unpinned,
                    },
                    Self::Unit => EnumProj::Unit,
                }
            }
        }
        fn project_ref<'__pin>(
            self: ::pin_project_lite::__private::Pin<&'__pin Self>,
        ) -> EnumProjRef<'__pin, T, U> {
            unsafe {
                match self.get_ref() {
                    Self::Struct { pinned, unpinned } => EnumProjRef::Struct {
                        pinned: ::pin_project_lite::__private::Pin::new_unchecked(pinned),
                        unpinned: unpinned,
                    },
                    Self::Unit => EnumProjRef::Unit,
                }
            }
        }
        fn project_replace(
            self: ::pin_project_lite::__private::Pin<&mut Self>,
            replacement: Self,
        ) -> EnumProjReplace<T, U> {
            unsafe {
                let __replacement = replacement;
                let __self_ptr: *mut Self = self.get_unchecked_mut();
                match &mut *__self_ptr {
                    Self::Struct { pinned, unpinned } => {
                        let result = EnumProjReplace::Struct {
                            pinned: ::pin_project_lite::__private::PhantomData,
                            unpinned: ::pin_project_lite::__private::ptr::read(unpinned),
                        };
                        let __guard = ::pin_project_lite::__private::UnsafeOverwriteGuard {
                            target: __self_ptr,
                            value: ::pin_project_lite::__private::ManuallyDrop::new(__replacement),
                        };
                        {
                            let __guard =
                                ::pin_project_lite::__private::UnsafeDropInPlaceGuard(pinned);
                        }
                        result
                    }
                    Self::Unit => {
                        let result = EnumProjReplace::Unit;
                        let __guard = ::pin_project_lite::__private::UnsafeOverwriteGuard {
                            target: __self_ptr,
                            value: ::pin_project_lite::__private::ManuallyDrop::new(__replacement),
                        };
                        result
                    }
                }
            }
        }
    }
    #[allow(non_snake_case)]
    struct __Origin<'__pin, T, U> {
        __dummy_lifetime: ::pin_project_lite::__private::PhantomData<&'__pin ()>,
        Struct: (T, ::pin_project_lite::__private::AlwaysUnpin<U>),
        Unit: (),
    }
    impl<'__pin, T, U> ::pin_project_lite::__private::Unpin for Enum<T, U> where
        __Origin<'__pin, T, U>: ::pin_project_lite::__private::Unpin
    {
    }
    trait MustNotImplDrop {}
    #[allow(clippy::drop_bounds, drop_bounds)]
    impl<T: ::pin_project_lite::__private::Drop> MustNotImplDrop for T {}
    impl<T, U> MustNotImplDrop for Enum<T, U> {}
};
fn main() {}
