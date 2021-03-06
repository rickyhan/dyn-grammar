use crate::prelude::*;

/// S -> top N bottom .
/// N -> beef S beef .
/// N -> chicken .
/// S -> .
pub struct Two;

impl Level for Two {

    fn name() -> &'static str {
        "El McGangbang"
    }

    fn description() -> &'static str {
        "The McChicken is placed directly inside The Big Mac."
    }

    fn orders() -> [Burger; 10] {
        [
            burger![TopBun, Chicken, BottomBun],
            burger![TopBun, Beef, Beef, BottomBun],
            burger![TopBun, Beef, TopBun, Beef, TopBun, Beef, Beef, BottomBun, Beef, BottomBun, Beef, BottomBun],
            burger![TopBun, Beef, TopBun, Chicken, BottomBun, Beef, BottomBun],
            burger![TopBun, Beef, TopBun, Beef, TopBun, Beef, Beef, BottomBun, Beef, BottomBun, Beef, BottomBun],
            burger![TopBun, Beef, TopBun, Beef, Beef, BottomBun, Beef, BottomBun],
            burger![TopBun, Beef, TopBun, Beef, TopBun, Beef, Beef, BottomBun, Beef, BottomBun, Beef, BottomBun],
            burger![TopBun, Beef, TopBun, Beef, TopBun, Chicken, BottomBun, Beef, BottomBun, Beef, BottomBun],
            burger![TopBun, Beef, TopBun, Beef, TopBun, Beef, TopBun, Chicken, BottomBun, Beef, BottomBun, Beef, BottomBun, Beef, BottomBun],
            burger![TopBun, Beef, TopBun, Beef, TopBun, Beef, TopBun, Beef, Beef, BottomBun, Beef, BottomBun, Beef, BottomBun, Beef, BottomBun],

        ]
    }

}
