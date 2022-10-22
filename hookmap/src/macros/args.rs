#[macro_export]
macro_rules! each {
    [ $($button:expr),* ] => {
        $crate::hotkey::args::ButtonArgs::Each(vec![$($button),*])
    };
}

#[macro_export]
macro_rules! not {
    [ $($button:expr),* ] => {
        $crate::hotkey::args::ButtonArgs::Not(
            std::sync::Arc::new( vec![ $( $button ),* ] )
        )
    };
}

#[macro_export]
macro_rules! modifiers {
    (@inner [ $( $acc:expr ),* ] !$arg:expr $(, $( $rest:tt )* )? ) => {
        modifiers!(
            @inner
            [
                $( $acc, )*
                $crate::hotkey::args::ModifierArg {
                    arg: $arg.into(),
                    invert: true,
                }
            ]
            $( $( $rest )* )?
        )
    };

    (@inner [ $( $acc:expr ),* ] $arg:expr $(, $( $rest:tt )* )? ) => {
        modifiers!(
            @inner
            [
                $( $acc, )*
                $crate::hotkey::args::ModifierArg {
                    arg: $arg.into(),
                    invert: false,
                }
            ]
            $( $( $rest )* )?
        )
    };

    (@inner $acc:tt ) => {
        $crate::hotkey::args::ModifierArgs { args: vec!$acc }
    };

    [ $($args:tt)* ] => {
        $crate::modifiers!(@inner [] $($args)* )
    };
}

// #[cfg(test)]
// mod tests {
//     use hookmap_core::button::Button;
//
//     use crate::device::Button::*;
//     use crate::hotkey::args::{ModifierArg, ModifierArgs};
//
//     fn arg(invertion: bool, button: Button) -> ModifierArg {
//         ModifierArg {
//             arg: button.into(),
//             invert: invertion,
//         }
//     }
//
//     #[test]
//     fn empty() {
//         assert_eq!(modifiers![], ModifierArgs { args: vec![] });
//     }
//
//     #[test]
//     fn single_pressed() {
//         assert_eq!(
//             modifiers![A],
//             ModifierArgs {
//                 args: vec![arg(false, A)]
//             }
//         );
//     }
//
//     #[test]
//     fn single_released() {
//         assert_eq!(
//             modifiers![!A],
//             ModifierArgs {
//                 args: vec![arg(true, A)]
//             }
//         );
//     }
//
//     #[test]
//     fn multi_pressed() {
//         assert_eq!(
//             modifiers![A, B, C],
//             ModifierArgs {
//                 args: vec![arg(false, A), arg(false, B), arg(false, C)]
//             }
//         )
//     }
//
//     #[test]
//     fn multi_released() {
//         assert_eq!(
//             modifiers![!A, !B, !C],
//             ModifierArgs {
//                 args: vec![arg(true, A), arg(true, B), arg(true, C)]
//             }
//         );
//     }
//
//     #[test]
//     fn mixed() {
//         assert_eq!(
//             modifiers!(!A, B, !C, D, !E, !F, G, H),
//             ModifierArgs {
//                 args: vec![
//                     arg(true, A),
//                     arg(false, B),
//                     arg(true, C),
//                     arg(false, D),
//                     arg(true, E),
//                     arg(true, F),
//                     arg(false, G),
//                     arg(false, H)
//                 ]
//             }
//         );
//     }
//
//     #[test]
//     fn expr() {
//         fn f() -> Button {
//             Button::C
//         }
//         let modifiers = modifiers![!if true { Button::A } else { Button::B }, !f()];
//         assert_eq!(
//             modifiers,
//             ModifierArgs {
//                 args: vec![arg(true, A), arg(true, C)]
//             }
//         );
//     }
// }
