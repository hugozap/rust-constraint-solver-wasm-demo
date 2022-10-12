mod utils;

use cassowary::strength::{MEDIUM, REQUIRED, STRONG, WEAK};
use cassowary::WeightedRelation::*;
use cassowary::{Solver, Variable};
use js_sys::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, rust-contraint-solver-wasm!");
}
#[wasm_bindgen]
struct Point {
    x: Variable,
    y: Variable,
}

#[wasm_bindgen]
impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point {
            x: Variable::new(),
            y: Variable::new(),
        }
    }
}

#[wasm_bindgen]
struct PointLocation {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
struct App {
    points: Vec<PointLocation>,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> App {
        App { points: Vec::new() }
    }

    //return a pointer to the points
    #[wasm_bindgen]
    pub fn get_points(&self) -> *const PointLocation {
        self.points.as_ptr()
    }
    #[wasm_bindgen]
    pub fn update_locations(&mut self) {
        //call calculateLayout and save return value
        let new_locations = self.calculateLayout();

        //add first point using p1.x and p1.y
        self.points.push(PointLocation {
            x: new_locations["p1.x"],
            y: new_locations["p1.y"],
        });

        //add second point using p2.x and p2.y
        self.points.push(PointLocation {
            x: new_locations["p2.x"],
            y: new_locations["p2.y"],
        });

    }

    fn calculateLayout(&self) -> HashMap<String, f64> {
        //simple solver for 2 constraints using cassowary 0.3.0
        let mut names = HashMap::new();
        let window_width = Variable::new();
        names.insert(window_width, "window_width");

        let p1 = Point {
            x: Variable::new(),
            y: Variable::new(),
        };

        let p2 = Point {
            x: Variable::new(),
            y: Variable::new(),
        };

        names.insert(p1.x, "p1.x");
        names.insert(p1.y, "p1.y");
        names.insert(p2.x, "p2.x");
        names.insert(p2.y, "p2.y");

        let mut solver = Solver::new();
        solver
            .add_constraints(&[
                p1.x | EQ(REQUIRED) | 0.0,
                p1.y | EQ(REQUIRED) | 0.0,
                p2.x | EQ(REQUIRED) | window_width,
                p2.y | EQ(REQUIRED) | 0.0,
            ])
            .unwrap();

        solver.add_edit_variable(window_width, STRONG).unwrap();
        solver.suggest_value(window_width, 100.0).unwrap();

        //return hash with values
        let mut result = HashMap::new();
        for (var, name) in names {
            result.insert(String::from(name), solver.get_value(var));
        }
        result
    }
}

//add test to calculateLayout
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculateLayout() {
        let result = calculateLayout();
        assert_eq!(result.get("p1.x"), Some(&0.0));
        assert_eq!(result.get("p1.y"), Some(&0.0));
        assert_eq!(result.get("p2.x"), Some(&100.0));
        assert_eq!(result.get("p2.y"), Some(&0.0));
    }
}
