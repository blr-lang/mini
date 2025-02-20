use std::{cell::RefCell, marker::PhantomData, rc::Rc};

use tracing::debug;

#[derive(Debug)]
pub struct Point<T> {
    link: Rc<RefCell<Link<T>>>,
}

impl<T> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.link, &other.link)
    }
}

impl<T> Clone for Point<T> {
    fn clone(&self) -> Self {
        Self {
            link: Rc::clone(&self.link),
        }
    }
}

#[derive(Debug)]
struct Info<T> {
    weight: usize,
    descriptor: T,
}

#[derive(Debug)]
enum Link<T> {
    Info(Rc<RefCell<Info<T>>>),
    Point(Point<T>),
}

impl<T> Clone for Link<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Info(info) => Self::Info(Rc::clone(&info)),
            Self::Point(point) => Self::Point(point.clone()),
        }
    }
}

pub struct UnionFind<T> {
    marker: PhantomData<T>,
}

impl<T: std::fmt::Debug> UnionFind<T> {
    pub fn fresh(descriptor: T) -> Point<T> {
        Point {
            link: Rc::new(RefCell::new(Link::Info(Rc::new(RefCell::new(Info {
                weight: 1,
                descriptor,
            }))))),
        }
    }
    pub fn repr(point: &Point<T>) -> Point<T> {
        let mut link = point.link.borrow_mut();
        match &*link {
            Link::Point(pointed) => {
                let pointeded = Self::repr(pointed);
                debug!(?point, ?pointed, ?pointeded, "depth recursion");
                if pointeded != *pointed {
                    let compressed_link = pointed.link.borrow().clone();
                    *link = compressed_link;
                    pointeded
                } else {
                    drop(link);
                    pointeded
                }
            }
            Link::Info(_) => {
                drop(link);
                point.clone()
            }
        }
    }
    pub fn find_map<U>(point: &Point<T>, f: impl FnOnce(&mut T) -> U) -> U {
        let link = point.link.borrow();
        match &*link {
            Link::Info(info) => f(&mut info.borrow_mut().descriptor),
            Link::Point(point) => match &*point.link.borrow() {
                Link::Info(info) => f(&mut info.borrow_mut().descriptor),
                Link::Point(point) => Self::find_map(&Self::repr(&point), f),
            },
        }
    }
    pub fn change(point: &Point<T>, v: T) {
        let link = point.link.borrow();
        match &*link {
            Link::Info(info) => {
                info.borrow_mut().descriptor = v;
            }
            Link::Point(point) => match &*point.link.borrow() {
                Link::Info(info) => {
                    info.borrow_mut().descriptor = v;
                }
                Link::Point(point) => Self::change(&Self::repr(&point), v),
            },
        }
    }
    pub fn union(point1: &Point<T>, point2: &Point<T>) {
        let point1 = Self::repr(point1);
        let point2 = Self::repr(point2);
        debug_assert_ne!(point1, point2);
        let mut l1 = point1.link.borrow_mut();
        let mut l2 = point2.link.borrow_mut();
        match (&*l1, &*l2) {
            (Link::Info(info1), Link::Info(info2)) => {
                let i1 = info1.borrow_mut();
                // We always keep 2
                let mut i2 = info2.borrow_mut();
                let w = i1.weight + i2.weight;
                if i1.weight >= i2.weight {
                    i2.weight = w;
                    drop(i1);
                    drop(i2);
                    info1.swap(&info2);
                    drop(l1);
                    *l2 = Link::Point(point1.into());
                } else {
                    i2.weight = w;
                    drop(i1);
                    drop(i2);
                    drop(l2);
                    *l1 = Link::Point(point2.into());
                }
            }
            _ => unreachable!("repr should return info links. l1:{l1:?} l2:{l2:?}"),
        }
    }
    pub fn equivalent(point1: &Point<T>, point2: &Point<T>) -> bool {
        Self::repr(point1) == Self::repr(point2)
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;
    use test_pretty_log::test;

    use crate::hmx::union_find::UnionFind;

    #[test]
    fn union_find() {
        let one = UnionFind::fresh(1);
        let two = UnionFind::fresh(2);
        let three = UnionFind::fresh(3);
        let four = UnionFind::fresh(4);
        UnionFind::union(&one, &two);
        UnionFind::union(&two, &four);

        assert!(UnionFind::equivalent(&one, &two));
        assert!(!UnionFind::equivalent(&one, &three));
        assert!(UnionFind::equivalent(&one, &four));

        assert!(!UnionFind::equivalent(&two, &three));
        assert!(UnionFind::equivalent(&two, &four));

        assert!(!UnionFind::equivalent(&three, &four));

        // Assert expectations over path compression
        expect![[r#"
            Point {
                link: RefCell {
                    value: Info(
                        RefCell {
                            value: Info {
                                weight: 3,
                                descriptor: 4,
                            },
                        },
                    ),
                },
            }
        "#]]
        .assert_debug_eq(&one);
        expect![[r#"
            Point {
                link: RefCell {
                    value: Point(
                        Point {
                            link: RefCell {
                                value: Info(
                                    RefCell {
                                        value: Info {
                                            weight: 3,
                                            descriptor: 4,
                                        },
                                    },
                                ),
                            },
                        },
                    ),
                },
            }
        "#]]
        .assert_debug_eq(&two);
        expect![[r#"
            Point {
                link: RefCell {
                    value: Info(
                        RefCell {
                            value: Info {
                                weight: 1,
                                descriptor: 3,
                            },
                        },
                    ),
                },
            }
        "#]]
        .assert_debug_eq(&three);
        expect![[r#"
            Point {
                link: RefCell {
                    value: Point(
                        Point {
                            link: RefCell {
                                value: Info(
                                    RefCell {
                                        value: Info {
                                            weight: 3,
                                            descriptor: 4,
                                        },
                                    },
                                ),
                            },
                        },
                    ),
                },
            }
        "#]]
        .assert_debug_eq(&four);
    }
}
