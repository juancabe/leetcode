#[allow(dead_code)]
struct Solution;

//  Category: algorithms
//  Level: Hard
//  Percent: 23.20749%

//  You are given a 2D integer array squares. Each squares[i] = [xi, yi, li] represents the coordinates of the bottom-left point and the side length of a square parallel to the x-axis.
//
//  Find the minimum y-coordinate value of a horizontal line such that the total area covered by squares above the line equals the total area covered by squares below the line.
//
//  Answers within 10-5 of the actual answer will be accepted.
//
//  Note: Squares may overlap. Overlapping areas should be counted only once in this version.
//
//
//  Example 1:
//
//
//  Input: squares = [[0,0,1],[2,2,1]]
//
//  Output: 1.00000
//
//  Explanation:
//
//
//
//  Any horizontal line between y = 1 and y = 2 results in an equal split, with 1 square unit above and 1 square unit below. The minimum y-value is 1.
//
//
//  Example 2:
//
//
//  Input: squares = [[0,0,2],[1,1,1]]
//
//  Output: 1.00000
//
//  Explanation:
//
//
//
//  Since the blue square overlaps with the red square, it will not be counted again. Thus, the line y = 1 splits the squares into two equal parts.
//
//
//
//  Constraints:
//
//
//  	1 <= squares.length <= 5 * 10⁴
//  	squares[i] = [xi, yi, li]
//  	squares[i].length == 3
//  	0 <= xi, yi <= 10⁹
//  	1 <= li <= 10⁹
//  	The total area of all the squares will not exceed 10¹⁵.
//

//  // @submit start
#[derive(Clone, Copy)]
enum EventKind {
    On,
    Off,
}

#[derive(Clone, Copy)]
struct Event {
    y: i32,
    kind: EventKind,
    xs: [i32; 2],
}

impl Event {
    fn from_sq(sq: &[i32]) -> [Self; 2] {
        let xs = [sq[0], sq[0] + sq[2]];
        let on = Self {
            y: sq[1],
            kind: EventKind::On,
            xs,
        };
        let off = Self {
            y: sq[1] + sq[2],
            kind: EventKind::Off,
            xs,
        };

        [on, off]
    }
}

struct SegmentTree {
    counts: Vec<i32>,
    covered_len: Vec<i64>,
    coords: Vec<i32>,
    n: usize,
}

impl SegmentTree {
    fn new(xs: Vec<i32>) -> Self {
        let n = xs.len();
        let size = 4 * n;
        Self {
            counts: vec![0; size],
            covered_len: vec![0; size],
            coords: xs,
            n,
        }
    }

    fn update(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize, val: i32) {
        if l <= start && end <= r {
            self.counts[node] += val;
        } else {
            let mid = (start + end) / 2;
            if l < mid {
                self.update(node * 2, start, mid, l, r, val);
            }
            if r > mid {
                self.update(node * 2 + 1, mid, end, l, r, val);
            }
        }
        self.push_up(node, start, end);
    }

    fn push_up(&mut self, node: usize, start: usize, end: usize) {
        if self.counts[node] > 0 {
            self.covered_len[node] = (self.coords[end] - self.coords[start]) as i64;
        } else if end - start > 1 {
            self.covered_len[node] = self.covered_len[node * 2] + self.covered_len[node * 2 + 1];
        } else {
            self.covered_len[node] = 0;
        }
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let mut xs: Vec<i32> = squares
            .iter()
            .flat_map(|sq| [sq[0], sq[0] + sq[2]])
            .collect();
        xs.sort_unstable();
        xs.dedup();

        let mut events: Vec<Event> = squares.iter().flat_map(|sq| Event::from_sq(sq)).collect();
        events.sort_unstable_by_key(|ev| ev.y);

        let mut st = SegmentTree::new(xs.clone());
        let mut total_area: f64 = 0.0;

        // First pass: Calculate total area
        for i in 0..events.len() - 1 {
            let event = events[i];
            let idx_l = xs.binary_search(&event.xs[0]).unwrap();
            let idx_r = xs.binary_search(&event.xs[1]).unwrap();

            let val = match event.kind {
                EventKind::On => 1,
                EventKind::Off => -1,
            };

            st.update(1, 0, st.n - 1, idx_l, idx_r, val);
            let height = (events[i + 1].y - event.y) as f64;
            total_area += height * st.covered_len[1] as f64;
        }

        // Reset Tree for Second Pass
        st = SegmentTree::new(xs.clone());
        let mut current_area: f64 = 0.0;
        let target = total_area / 2.0;

        for i in 0..events.len() - 1 {
            let event = events[i];
            let idx_l = xs.binary_search(&event.xs[0]).unwrap();
            let idx_r = xs.binary_search(&event.xs[1]).unwrap();

            let val = match event.kind {
                EventKind::On => 1,
                EventKind::Off => -1,
            };

            st.update(1, 0, st.n - 1, idx_l, idx_r, val);

            let width = st.covered_len[1] as f64;
            let height = (events[i + 1].y - event.y) as f64;
            let area_segment = width * height;

            if current_area + area_segment >= target - 1e-11 {
                return event.y as f64 + (target - current_area) / width;
            }
            current_area += area_segment;
        }

        events.last().unwrap().y as f64
    }
}
//  // @submit end

fn main() {}
