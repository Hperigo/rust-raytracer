## Ray Tracing in One Weekend but in rust.

A raytracer written in rust as a project to practice rust and graphics programming. 



#### Notes: 

1. Why do we need this type of MaterialClone trait? And also why cant we call `clone` on `geometry.rs` and can only call `clone_box`? 

```
pub trait  MaterialClone {
    fn clone_box(&self) -> Box<dyn Material>;
}

impl<T> MaterialClone for T
where
    T: 'static + Material + Clone,
{
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}
```