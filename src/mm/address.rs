use core::fmt::Debug;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct PhyAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct VirtAddr(pub usize);

trait NextStep {
    fn next(&mut self);
}

impl NextStep for PhyAddr {
    fn next(&mut self) {
        self.0 += 1;
    }
}

impl NextStep for VirtAddr {
    fn next(&mut self) {
        self.0 += 1;
    }
}

struct MemRange<T>
where
    T: Copy + PartialEq + PartialOrd + Debug + NextStep,
{
    start: T,
    end: T,
}

impl<T> MemRange<T>
where
    T: Copy + PartialEq + PartialOrd + Debug + NextStep,
{
    pub fn new(start: T, end: T) -> Self {
        assert!(start <= end);
        MemRange { start, end }
    }

    pub fn get_start(&self) -> T {
        self.start
    }

    pub fn get_end(&self) -> T {
        self.end
    }
}

impl<T> IntoIterator for MemRange<T>
where
    T: Copy + PartialEq + PartialOrd + Debug + NextStep,
{
    type Item = T;
    type IntoIter = MemRangeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        MemRangeIterator {
            current: self.start,
            end: self.end,
        }
    }
}

struct MemRangeIterator<T>
where
    T: Copy + PartialEq + PartialOrd + Debug + NextStep,
{
    current: T,
    end: T,
}

impl<T> Iterator for MemRangeIterator<T>
where
    T: Copy + PartialEq + PartialOrd + Debug + NextStep,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let tmp = self.current;
            self.current.next();
            Some(tmp)
        }
    }
}
