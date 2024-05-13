// From https://github.com/danielway/micrograd-rs/blob/master/src/value.rs

// https://rufflewind.com/2016-12-30/reverse-mode-automatic-differentiation
// https://tiberiusferreira.github.io/blog/posts/designing_autograd_system_rust_first_steps/

use std::{
    cell::{Ref, RefCell},
    collections::HashSet,
    rc::Rc,
};

#[derive(Clone, Eq, PartialEq)]
pub struct Tensor(Rc<RefCell<TensorInternal>>);

impl std::fmt::Display for Tensor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.borrow().data)
    }
}

// debug print
impl std::fmt::Debug for Tensor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.borrow().data)
    }
}

impl Tensor {
    pub fn from<T>(t: T) -> Tensor
    where
        T: Into<Tensor>,
    {
        t.into()
    }

    pub fn new(tensor: TensorInternal) -> Self {
        Tensor(Rc::new(RefCell::new(tensor)))
    }

    pub fn adjust(&self, factor: f64) {
        let mut value = self.borrow_mut();
        value.data += factor * value.gradient;
    }

    pub fn pow(&self, other: &Tensor) -> Tensor {
        let result = self.borrow().data.powf(other.borrow().data);

        let prop_fn: PropagateFn = |value| {
            let mut base = value.previous[0].borrow_mut();
            let power = value.previous[1].borrow();
            base.gradient += power.data * (base.data.powf(power.data - 1.0)) * value.gradient;
        };

        Tensor::new(TensorInternal::new(
            result,
            None,
            Some("^".to_string()),
            vec![self.clone(), other.clone()],
            Some(prop_fn),
        ))
    }

    pub fn tanh(&self) -> Tensor {
        let result = self.borrow().data.tanh();

        let prop_fn: PropagateFn = |value| {
            let mut previous = value.previous[0].borrow_mut();
            previous.gradient += (1.0 - value.data.powf(2.0)) * value.gradient;
        };

        Tensor::new(TensorInternal::new(
            result,
            None,
            Some("tanh".to_string()),
            vec![self.clone()],
            Some(prop_fn),
        ))
    }

    pub fn relu(&self) -> Tensor {
        let result = self.borrow().data.max(0.0);

        let prop_fn: PropagateFn = |value| {
            let mut previous = value.previous[0].borrow_mut();
            previous.gradient += (value.data > 0.0) as i32 as f64 * value.gradient;
        };

        Tensor::new(TensorInternal::new(
            result,
            None,
            Some("relu".to_string()),
            vec![self.clone()],
            Some(prop_fn),
        ))
    }

    pub fn gradient(&self) -> f64 {
        self.borrow().gradient
    }

    pub fn clear_gradient(&self) {
        self.borrow_mut().gradient = 0.0;
    }

    pub fn backward(&self) {
        let mut visited: HashSet<Tensor> = HashSet::new();

        self.borrow_mut().gradient = 1.0;
        self.backward_internal(&mut visited, self);
    }

    fn backward_internal(&self, visited: &mut HashSet<Tensor>, tensor: &Tensor) {
        if !visited.contains(&tensor) {
            visited.insert(tensor.clone());

            let borrowed_value = tensor.borrow();
            if let Some(prop_fn) = borrowed_value.propagate {
                prop_fn(&borrowed_value);
            }

            for child_id in &tensor.borrow().previous {
                self.backward_internal(visited, child_id);
            }
        }
    }
}

fn add(a: &Tensor, b: &Tensor) -> Tensor {
    let result = a.borrow().data + b.borrow().data;

    let prop_fn: PropagateFn = |value| {
        let mut first = value.previous[0].borrow_mut();
        let mut second = value.previous[1].borrow_mut();

        first.gradient += value.gradient;
        second.gradient += value.gradient;
    };

    Tensor::new(TensorInternal::new(
        result,
        None,
        Some("+".to_string()),
        vec![a.clone(), b.clone()],
        Some(prop_fn),
    ))
}

fn mul(a: &Tensor, b: &Tensor) -> Tensor {
    let result = a.borrow().data * b.borrow().data;

    let prop_fn: PropagateFn = |value| {
        let mut first = value.previous[0].borrow_mut();
        let mut second = value.previous[1].borrow_mut();

        first.gradient += second.data * value.gradient;
        second.gradient += first.data * value.gradient;
    };

    Tensor::new(TensorInternal::new(
        result,
        None,
        Some("*".to_string()),
        vec![a.clone(), b.clone()],
        Some(prop_fn),
    ))
}

impl std::ops::Add for Tensor {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        add(&self, &other)
    }
}

impl std::ops::Sub for Tensor {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        add(&self, &(-other))
    }
}

impl std::ops::Mul for Tensor {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        mul(&self, &other)
    }
}

impl std::ops::Div for Tensor {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        // TODO: does this work?
        mul(&self, &other.pow(&Tensor::from(-1)))
    }
}

impl std::ops::Neg for Tensor {
    type Output = Self;

    fn neg(self) -> Self {
        mul(&self, &Tensor::from(-1))
    }
}

// impl PartialEq for Tensor {
//     fn eq(&self, other: &Self) -> bool {
//         self.data == other.data
//     }
// }

// impl PartialOrd for Tensor {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         self.data.partial_cmp(&other.data)
//     }
// }

impl std::hash::Hash for Tensor {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state);
    }
}

impl std::ops::Deref for Tensor {
    type Target = Rc<RefCell<TensorInternal>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Into<f64>> From<T> for Tensor {
    fn from(t: T) -> Tensor {
        Tensor::new(TensorInternal::new(t.into(), None, None, Vec::new(), None))
    }
}

////////////////////////////////////////////////////
/// ////////////////////////////////////////////////

type PropagateFn = fn(value: &Ref<TensorInternal>);

pub struct TensorInternal {
    data: f64,
    gradient: f64,
    label: Option<String>,
    operation: Option<String>,
    previous: Vec<Tensor>,
    propagate: Option<PropagateFn>,
}

impl TensorInternal {
    fn new(
        data: f64,
        label: Option<String>,
        op: Option<String>,
        prev: Vec<Tensor>,
        propagate: Option<PropagateFn>,
    ) -> TensorInternal {
        TensorInternal {
            data,
            gradient: 0.0,
            label,
            operation: op,
            previous: prev,
            propagate,
        }
    }
}

impl PartialEq for TensorInternal {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
            && self.gradient == other.gradient
            && self.label == other.label
            && self.operation == other.operation
            && self.previous == other.previous
    }
}

impl Eq for TensorInternal {}

impl std::hash::Hash for TensorInternal {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.to_bits().hash(state);
        self.gradient.to_bits().hash(state);
        self.label.hash(state);
        self.operation.hash(state);
        self.previous.hash(state);
    }
}

impl std::fmt::Debug for TensorInternal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValueInternal")
            .field("data", &self.data)
            .field("gradient", &self.gradient)
            .field("label", &self.label)
            .field("operation", &self.operation)
            .field("previous", &self.previous)
            .finish()
    }
}
