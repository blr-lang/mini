use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Point<T> {
    link: Link<T>,
}

type Info<T> = RefCell<InfoData<T>>;
type Link<T> = Rc<RefCell<Option<Point<T>>>>;
type InfoData<T> = RefCell<InfoDataValue<T>>;

struct InfoDataValue<T> {
    weight: i32,
    descriptor: T,
}

fn fresh(descriptor: T) -> Point<T> {
    let info = Rc::new(RefCell::new(InfoDataValue {
        weight: 1,
        descriptor,
    }));
    Point {
        link: Rc::new(RefCell::new(Some(info.clone()))),
    }
}

fn repr(point: &Point<T>) -> &Point<T> {
    match point.link.borrow_mut().as_mut() {
        Some(link) => {
            let mut link = link.as_mut();
            match link.take() {
                Some(info) => {
                    *link = Some(point.clone());
                    repr(link.as_ref())
                }
                None => point,
            }
        }
        None => point,
    }
}

fn find<T>(point: &Point<T>) -> &InfoDataValue<T> {
    match point.link.borrow() {
        InfoDataValue { .. } | RefCell::borrow_ref(info) => info,
        RefCell::downgrade(weak) => weak.up().unwrap().borrow(),
    }
}

fn change<T>(point: &mut Point<T>, descriptor: T) {
    let mut info = point.link.borrow_mut();
    *info.as_mut() = Some(point.clone());
    info.borrow_mut().descriptor = descriptor;
}

fn union<T>(point1: &Point<T>, point2: &Point<T>) {
    let point1 = repr::<T>(point1);
    let point2 = repr::<T>(point2);
    assert!(point1 != point2);
    let (mut weight1, mut weight2) = (
        point1.link.borrow().downgrade(),
        point2.link.borrow().downgrade(),
    );
    if weight1.up().unwrap().strong_count() >= weight2.up().unwrap().strong_count() {
        point2
            .link
            .clone()
            .into_inner()
            .replace(Rc::new(RefCell::new(Some(point1.clone()))));
        weight1.take().unwrap().borrow_mut().weight += weight2.take().unwrap().borrow().weight;
        weight1.borrow_mut().descriptor = point2
            .link
            .borrow()
            .downgrade()
            .unwrap()
            .into_inner()
            .borrow()
            .descriptor;
    } else {
        point1
            .link
            .clone()
            .into_inner()
            .replace(Rc::new(RefCell::new(Some(point2.clone()))));
        weight2.take().unwrap().borrow_mut().weight += weight1.take().unwrap().borrow().weight;
    }
}

fn equivalent<T>(point1: &Point<T>, point2: &Point<T>) -> bool {
    repr::<T>(point1) == repr::<T>(point2)
}

struct Redundant;
impl<T> Redundant
where
    Point<T>: Eq,
{
    fn new() -> Self {
        Redundant {}
    }
}

impl<T> Iterator for Redundant {
    type Item = Point<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let point = |point: &Point<T>| {
            if point.link.borrow().as_ref().is_none() {
                Some(point.clone())
            } else {
                None
            }
        };
        self.points
            .iter()
            .position(|p| point(p).is_some())
            .map(|i| self.points[i].clone())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.points.len(), Some(self.points.len()))
    }
}
