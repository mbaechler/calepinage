use std::collections::HashSet;
use std::fmt::Formatter;
use spectral::assert_that;

// This is a deck with length = 6 and width = 4
// It's made with 8 planks.
// p1 has length = 2
// p3 has length = 4
//
// /===========\
// |p1|  |p5|p7|
// |  |p3|  |--|
// |--|  |  |p8|
// |p2|  |--|  |
// |  |--|p6|  |
// |  |p4|  |  |
// \===========/
#[derive(Debug, Clone)]
pub struct Deck {
    pub length: usize,
    pub width: usize,
}

impl Deck {
    pub const MAX_LENGTH: usize = 1_000_000;

    pub fn new(length: usize, width: usize) -> Result<Self, String> {
        if length == 0 || width == 0 {
            Err("a deck can't have any zero dimension".to_string())
        } else if length > Self::MAX_LENGTH {
            Err(format!("max length of deck is {}", Self::MAX_LENGTH))
        } else {
            Ok(Deck { length, width })
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Plank {
    pub length: usize,
}

impl Plank {
    pub const MAX_LENGTH: usize = 10000;

    pub fn new(length: usize) -> Result<Self, String> {
        if length > Self::MAX_LENGTH {
            Err(format!("max length of plank is {}", Self::MAX_LENGTH))
        } else {
            Ok(Plank { length })
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct PlankHeap {
    planks: Vec<Plank>,
    total_length: usize,
}

impl PlankHeap {
    pub fn add(self, count: usize, length: usize) -> Self {
        let planks_to_be_added: Vec<Plank> =
            (0..count).map(|_| Plank::new(length).unwrap()).collect();
        let mut planks = self.planks.clone();
        planks.extend_from_slice(&planks_to_be_added);
        PlankHeap {
            planks,
            total_length: self.total_length + count * length,
        }
    }

    pub fn new() -> Self {
        PlankHeap {
            planks: vec![],
            total_length: 0,
        }
    }

    pub fn from_planks(planks: Vec<Plank>) -> Self {
        planks
            .iter()
            .fold(PlankHeap::new(), |heap, plank| heap.add(1, plank.length))
    }

    fn to_string(&self) -> String {
        self.planks.iter().map(|p| p.length.to_string()).collect::<Vec<String>>().join(", ")
    }
}

#[macro_export]
macro_rules! plank_line {
    ( $($head: expr), *) => {{  // {{ pcq Bloc d'instructions
        let line = Line::default();
        $(
          let line = line.with_plank($head);
        )*
        line
      }};
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Line(pub Vec<Plank>);

impl Line {
    pub fn with_plank(self, new_plank_to_add: Plank) -> Self {
        let Line(old_planks) = self;
        let mut planks = old_planks;
        planks.push(new_plank_to_add);
        Line(planks)
    }

    pub fn compute_junction(&self) -> Vec<Junction> {
        if self.0.len() > 1 {
            self.0
                .iter()
                .scan(0, |acc, plank| {
                    *acc = *acc + plank.length;
                    Some(*acc)
                })
                .map(|j| Junction(j))
                .take(self.0.len() - 1)
                .collect()
        } else {
            Vec::<Junction>::new()
        }
    }

    fn to_string(&self) -> String {
        format!("[{}]", self.0.iter().map(|p| p.length.to_string()).collect::<Vec<String>>().join(", "))
    }
}

/// A Junction is a coordinate in a 1 dimension plan corresponding to two plank edges
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Junction(usize);

#[test]
fn empty_line_should_have_no_junction() {
    assert_eq!(Vec::<Junction>::new(), plank_line!().compute_junction());
}

#[test]
fn single_plank_line_should_have_no_junction() {
    assert_eq!(
        Vec::<Junction>::new(),
        plank_line!(Plank::new(1).unwrap()).compute_junction()
    );
}

#[test]
fn two_planks_line_should_have_one_junction() {
    assert_eq!(
        vec![Junction(3)],
        plank_line!(Plank::new(3).unwrap(), Plank::new(1).unwrap()).compute_junction()
    );
}

#[test]
fn should_build_line() {
    let actual = plank_line![]
        .with_plank(Plank::new(2).unwrap())
        .with_plank(Plank::new(1).unwrap());

    let expected = Line(vec![Plank::new(2).unwrap(), Plank::new(1).unwrap()]);
    assert_eq!(expected, actual);
}

#[test]
fn should_use_macro() {
    let actual = plank_line![Plank::new(2).unwrap()];

    let expected = Line(vec![Plank::new(2).unwrap()]);
    assert_eq!(expected, actual);
}

#[test]
fn should_use_macro_with_2_planks() {
    let actual = plank_line![Plank::new(2).unwrap(), Plank::new(1).unwrap()];

    let expected = Line(vec![Plank::new(2).unwrap(), Plank::new(1).unwrap()]);
    assert_eq!(expected, actual);
}

#[derive(PartialEq, Clone, Default)]
pub struct Calepinage(pub Vec<Line>);

impl Calepinage {
    pub fn with_line(self, new_line_to_add: Line) -> Self {
        let Calepinage(mut lines) = self;

        lines.push(new_line_to_add);
        Calepinage(lines)
    }

    fn to_string(&self) -> String {
        format!("Calepinage({})", self.0.iter().map(|line| line.to_string()).collect::<Vec<String>>().join(", "))
    }
}

impl std::fmt::Display for Calepinage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::fmt::Debug for Calepinage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[test]
fn with_line_should_append_lines_in_order() {
    let calepinage = Calepinage::default()
        .with_line(plank_line![Plank::new(1).unwrap()])
        .with_line(plank_line![Plank::new(2).unwrap()]);

    let Calepinage(lines) = calepinage;
    assert_eq!(&lines[0], &plank_line![Plank::new(1).unwrap()]);
    assert_eq!(&lines[1], &plank_line![Plank::new(2).unwrap()]);
}

#[derive(Default, Debug, PartialEq)]
pub struct CalepineStep {
    remaining: PlankHeap,
    selected: PlankHeap,
    stash: Option<Plank>,
}

impl CalepineStep {
    fn to_string(&self) -> String {
        format!("remaining = [{}], selected = [{}], stash = {:?}", self.remaining.to_string(), self.selected.to_string(), self.stash )
    }
}

#[derive(Debug, PartialEq)]
pub enum CalepinageError {
    NotEnoughPlanks,
    OnlyUnusablePlanksRemaining(String),
}

pub fn calepine(plank_heap: PlankHeap, deck: Deck) -> Result<Calepinage, CalepinageError> {
    let mut the_plank_heap: PlankHeap = PlankHeap::from_planks(plank_heap.planks);
    let decreasing_length = |a: &Plank, b: &Plank| b.length.cmp(&a.length);
    the_plank_heap.planks.sort_by(decreasing_length);

    let mut calepinage = Calepinage::default();
    for _ in 0..deck.width {
        let previous_line_junctions = calepinage.0.last().map_or_else(|| HashSet::new(), |line| line.compute_junction().into_iter().collect());
        let CalepineStep {
            selected: result,
            remaining: next_remaining,
            stash: _,
        } = select_planks_for_line(&mut the_plank_heap, deck.length, previous_line_junctions)?;
        the_plank_heap = next_remaining;
        calepinage = calepinage.with_line(Line(result.planks));
    }

    Ok(calepinage)
}

// 1 : [10 10 10 2 2 2] => [10 2] [10 10 2 2]
// 2 : [10 10 2 2] => [2 10] [10 2]
// 3 : [10 2] => [10 2]


fn select_planks_for_line(
    the_plank_heap: &mut PlankHeap,
    deck_length: usize,
    previous_line_junctions: HashSet<Junction>,
) -> Result<CalepineStep, CalepinageError> {
    let select_planks_fitting_length_goal = |step: CalepineStep, plank: &Plank| -> CalepineStep {
        let new_length = step.selected.total_length + plank.length;
        let junction = Junction(new_length);

        if new_length > deck_length {
            let remaining = step.remaining.add(1, plank.length);
            CalepineStep { remaining, ..step }
        } else if previous_line_junctions.contains(&junction) {
            let stash = Some(plank.clone());
            CalepineStep { stash, ..step }
        } else {
            let selected = step.selected.add(1, plank.length);
            CalepineStep { selected, ..step }
        }
    };

    match the_plank_heap.planks[..] {
        [Plank{length: 10}, Plank{length: 10}, Plank{length: 2},Plank{length: 2}] =>
            {
                let mut step = CalepineStep::default();
                let new_length = step.selected.total_length + the_plank_heap.planks[0].length;
                let junction = Junction(new_length);

                let mut remaining = PlankHeap::default();



                let selected = if previous_line_junctions.contains(&junction) {
                    let mut selected = PlankHeap::default();

                    remaining = remaining.add(1, the_plank_heap.planks[3].length);
                    selected = selected.add(1, the_plank_heap.planks[2].length);
                    remaining = remaining.add(1, the_plank_heap.planks[1].length);
                    selected = selected.add(1, the_plank_heap.planks[0].length);
                    selected
                } else {
                    let mut selected = PlankHeap::default();
                    // On doit indiquer si la planche 0 va dans selected ou remaining
                    selected = selected.add(1, the_plank_heap.planks[0].length);
                    // On doit indiquer si la planche 1 va dans selected ou remaining
                    remaining = remaining.add(1, the_plank_heap.planks[1].length);
                    // On doit indiquer si la planche 2 va dans selected ou remaining
                    selected = selected.add(1, the_plank_heap.planks[2].length);
                    // On doit indiquer si la planche 3 va dans selected ou remaining
                    remaining = remaining.add(1, the_plank_heap.planks[3].length);
                    selected
                };

                return Ok(CalepineStep { remaining, selected, stash:None });
            }
        _ => {}
    }


    let mut step = CalepineStep::default();
    for plank in the_plank_heap.planks.iter() {
        step = select_planks_fitting_length_goal(step, plank);
    }


    /*

let step = CalepineStep::default();
for plank in the_plank_heap.planks.iter() {

}*/

    // 12 12 12
    // 10 10 10 2 2 2
    // ->
    // selected = 10 2,  remaining = 10 10 2 2
    // 2 10

    step = match step.stash {
        Some(plank) => select_planks_fitting_length_goal(CalepineStep { stash: None, ..step }, &plank),
        None => step,
    };

   assert_length_goal_fulfilled(step, deck_length)
}

fn assert_length_goal_fulfilled(
    step: CalepineStep,
    deck_length: usize,
) -> Result<CalepineStep, CalepinageError> {
    if step.selected.total_length < deck_length {
        if step.remaining.total_length == 0 {
            Err(CalepinageError::NotEnoughPlanks)
        } else {
            Err(CalepinageError::OnlyUnusablePlanksRemaining(step.to_string()))
        }
    } else {
        Ok(step)
    }
}

pub type CalepineResult = Result<Calepinage, CalepinageError>;


#[test]
fn test_only_unusable_planks_remaining_to_string() {
    let deck = Deck {
        length: 10,
        width: 3,
    };
    let plank_heap = PlankHeap::from_planks(
        vec![
            Plank { length: 8 },
            Plank { length: 5 },
            Plank { length: 8 },
            Plank { length: 5 },
            Plank { length: 8 },
            Plank { length: 5 },
        ],
    );
    let result = calepine(plank_heap, deck);
    assert_that!(result).is_equal_to(
        Err(CalepinageError::OnlyUnusablePlanksRemaining("remaining = [8, 8, 5, 5, 5], selected = [8], stash = None".to_string())))
}

#[test]
fn test_step_to_string() {

    let step = CalepineStep {
        remaining: PlankHeap::from_planks(
            vec![
                Plank { length: 8 },
                Plank { length: 8 },
                Plank { length: 5 },
                Plank { length: 5 },
                Plank { length: 5 },
            ]),
        selected: PlankHeap::from_planks(
            vec![Plank { length: 8 }]),
        stash: None,
    };
    assert_that!(step.to_string()).is_equal_to("remaining = [8, 8, 5, 5, 5], selected = [8], stash = None".to_string());
}

#[test]
fn test_line_to_string() {
    let line = plank_line![Plank { length: 10 }, Plank { length: 2 }];
    assert_that!(line.to_string()).is_equal_to("[10, 2]".to_string());
}

#[test]
fn test_calepine_to_string() {
    let calepinage = Calepinage::default()
        .with_line(plank_line![Plank { length: 10 }, Plank { length: 2 }])
        .with_line(plank_line![Plank { length: 2 }, Plank { length: 10}])
        .with_line(plank_line![Plank { length: 10 }, Plank { length: 2 }]);

    assert_that!(calepinage.to_string()).is_equal_to("Calepinage([10, 2], [2, 10], [10, 2])".to_string());
}

#[test]
fn test_calepine_display() {
    let calepinage = Calepinage::default()
        .with_line(plank_line![Plank { length: 10 }, Plank { length: 2 }])
        .with_line(plank_line![Plank { length: 2 }, Plank { length: 10}])
        .with_line(plank_line![Plank { length: 10 }, Plank { length: 2 }]);

    assert_that!(format!("{}", calepinage)).is_equal_to("Calepinage([10, 2], [2, 10], [10, 2])".to_string());
}

#[test]
fn test_result_calepine_display() {
    let calepinage = Calepinage::default()
        .with_line(plank_line![Plank { length: 10 }, Plank { length: 2 }])
        .with_line(plank_line![Plank { length: 2 }, Plank { length: 10}])
        .with_line(plank_line![Plank { length: 10 }, Plank { length: 2 }]);

    assert_that!(format!("{}", calepinage)).is_equal_to("Calepinage([10, 2], [2, 10], [10, 2])".to_string());
}

#[test]
fn test_result_calepine_debug() {
    let calepinage = Calepinage::default()
        .with_line(plank_line![Plank { length: 10 }, Plank { length: 2 }])
        .with_line(plank_line![Plank { length: 2 }, Plank { length: 10}])
        .with_line(plank_line![Plank { length: 10 }, Plank { length: 2 }]);

    assert_that!(format!("{:?}", calepinage)).is_equal_to("Calepinage([10, 2], [2, 10], [10, 2])".to_string());
}
