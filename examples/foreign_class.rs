#[macro_use]
extern crate wren;
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::mem;
use std::ptr;
use wren::{Configuration, ForeignClassMethods, ForeignMethodFn, Pointer, VM};

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            (self.y * rhs.z) - (self.z * rhs.y),
            (self.z * rhs.x) - (self.x * rhs.z),
            (self.x * rhs.y) - (self.y * rhs.x),
        )
    }
}

impl ToString for Vec3 {
    fn to_string(&self) -> String {
        format!("{}, {}, {}", self.x, self.y, self.z)
    }
}

lazy_static! {
    static ref FOREIGN_METHODS: HashMap<&'static str, ForeignMethodFn> = {
        let mut map = HashMap::new();
        map.insert(
            "vectorVec3toString",
            wren_foreign_method_fn!(vec3_to_string),
        );
        map.insert("vectorVec3norm()", wren_foreign_method_fn!(vec3_norm));
        map.insert("vectorVec3dot(_)", wren_foreign_method_fn!(vec3_dot));
        map.insert("vectorVec3cross(_)", wren_foreign_method_fn!(vec3_cross));
        map.insert("vectorVec3x", wren_foreign_method_fn!(vec3_get_x));
        map.insert("vectorVec3x=(_)", wren_foreign_method_fn!(vec3_set_x));
        map.insert("vectorVec3y", wren_foreign_method_fn!(vec3_get_y));
        map.insert("vectorVec3y=(_)", wren_foreign_method_fn!(vec3_set_y));
        map.insert("vectorVec3z", wren_foreign_method_fn!(vec3_get_z));
        map.insert("vectorVec3z=(_)", wren_foreign_method_fn!(vec3_set_z));
        map
    };
}

lazy_static! {
    static ref FOREIGN_CLASSES: HashMap<&'static str, ForeignClassMethods> = {
        let mut map = HashMap::new();

        let mut vec3_class_methods = ForeignClassMethods::new();
        vec3_class_methods.set_allocate_fn(wren_foreign_method_fn!(vec3_allocate));
        vec3_class_methods.set_finalize_fn(wren_finalizer_fn!(vec3_finalize));

        map.insert("vectorVec3", vec3_class_methods);
        map
    };
}

fn vec3_allocate(vm: &mut VM) {
    let ptr = vm.set_slot_new_foreign(0, 0, mem::size_of::<Vec3>()) as *mut Vec3;
    let vec = Vec3::new(
        vm.get_slot_double(1).unwrap(),
        vm.get_slot_double(2).unwrap(),
        vm.get_slot_double(3).unwrap(),
    );
    unsafe { ptr::write(ptr, vec) };
}

fn vec3_finalize(_: Pointer) {
    // do nothing.
}

fn vec3_to_string(vm: &mut VM) {
    let vec = vm.get_slot_foreign(0).unwrap() as *mut Vec3;
    vm.set_slot_string(0, unsafe { &(*vec).to_string() });
}

fn vec3_norm(vm: &mut VM) {
    let vec = vm.get_slot_foreign(0).unwrap() as *mut Vec3;
    let result = unsafe { (*vec).norm() };
    vm.set_slot_double(0, result);
}

fn vec3_dot(vm: &mut VM) {
    let lhs = vm.get_slot_foreign(0).unwrap() as *mut Vec3;
    let rhs = vm.get_slot_foreign(1).unwrap() as *mut Vec3;
    let result = unsafe { (*lhs).dot(&*rhs) };
    vm.set_slot_double(0, result);
}

fn vec3_cross(vm: &mut VM) {
    let lhs = vm.get_slot_foreign(0).unwrap() as *mut Vec3;
    let rhs = vm.get_slot_foreign(1).unwrap() as *mut Vec3;
    let result = unsafe { (*lhs).cross(&*rhs) };

    // Retrieve the Vec3 class and create a new object.
    vm.get_variable("vector", "Vec3", 0);
    let result_ptr = vm.set_slot_new_foreign(0, 0, mem::size_of::<Vec3>()) as *mut Vec3;
    unsafe { ptr::write(result_ptr, result) };
}

fn vec3_get_x(vm: &mut VM) {
    let vec = vm.get_slot_foreign(0).unwrap() as *mut Vec3;
    let x = unsafe { (*vec).x };
    vm.set_slot_double(0, x);
}

fn vec3_set_x(vm: &mut VM) {
    let vec = vm.get_slot_foreign(0).unwrap() as *mut Vec3;
    let x = vm.get_slot_double(1).unwrap();
    unsafe { (*vec).x = x };
}

fn vec3_get_y(vm: &mut VM) {
    let vec = vm.get_slot_foreign(0).unwrap() as *mut Vec3;
    let y = unsafe { (*vec).y };
    vm.set_slot_double(0, y);
}

fn vec3_set_y(vm: &mut VM) {
    let vec = vm.get_slot_foreign(0).unwrap() as *mut Vec3;
    let y = vm.get_slot_double(1).unwrap();
    unsafe { (*vec).y = y };
}

fn vec3_get_z(vm: &mut VM) {
    let vec = vm.get_slot_foreign(0).unwrap() as *mut Vec3;
    let z = unsafe { (*vec).z };
    vm.set_slot_double(0, z);
}

fn vec3_set_z(vm: &mut VM) {
    let vec = vm.get_slot_foreign(0).unwrap() as *mut Vec3;
    let z = vm.get_slot_double(1).unwrap();
    unsafe { (*vec).z = z };
}

fn bind_method(
    _: &mut VM,
    module: &str,
    class_name: &str,
    is_static: bool,
    signature: &str,
) -> ForeignMethodFn {
    let full_signature = format!(
        "{}{}{}{}",
        module,
        class_name,
        signature,
        if is_static { "s" } else { "" }
    );
    *FOREIGN_METHODS.get::<str>(&full_signature).unwrap_or(&None)
}

fn bind_class(_: &mut VM, module: &str, class_name: &str) -> ForeignClassMethods {
    let full_signature = format!("{}{}", module, class_name);
    let methods = FOREIGN_CLASSES.get::<str>(&full_signature);
    if let Some(methods) = methods {
        return *methods;
    }
    panic!("Failed to bind foreign class");
}

fn load_module(_: &mut VM, name: &str) -> Option<String> {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let mut path = Path::new("examples/scripts").join(&name);
    path.set_extension("wren");
    let mut buffer = String::new();
    if File::open(path)
        .map(|mut f| f.read_to_string(&mut buffer))
        .is_ok()
    {
        Some(buffer)
    } else {
        None
    }
}

fn main() {
    let source = r#"
import "vector" for Vec3
var vec = Vec3.new(1, 2, 3)
var vec2 = Vec3.new(45, 30, 15)
System.print("vec = %(vec)")
System.print("vec2 = %(vec2)")
System.print("vec.norm() = %(vec.norm())")
System.print("vec.dot(vec2) = %(vec.dot(vec2))")
System.print("vec.cross(vec2) = %(vec.cross(vec2))")
"#;
    let mut cfg = Configuration::new();
    cfg.set_bind_foreign_method_fn(wren_bind_foreign_method_fn!(bind_method));
    cfg.set_bind_foreign_class_fn(wren_bind_foreign_class_fn!(bind_class));
    cfg.set_load_module_fn(wren_load_module_fn!(load_module));
    let mut vm = VM::new(cfg);
    vm.interpret(source);
}
