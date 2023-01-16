use std::{cell::RefCell, mem::transmute};
use winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
};

// Could add own keys, so we can remove the dependency to winit

pub enum KeyState
{
    Pressed,
    Released,
}

struct Input
{
    keys: [u8; 512],
    changes: [u8; 512]
}

impl Input
{
    fn new() -> Self
    {
        let s: Self = unsafe { std::mem::zeroed() };
        return s;
    }
}

thread_local!(static GLOBAL_KEYS: RefCell<Input> = RefCell::new(Input::new()));

pub fn reset()
{
    GLOBAL_KEYS.with(|s|
    {
        unsafe { s.borrow_mut().changes = std::mem::zeroed() };
    });
}

pub fn is_down(key: &VirtualKeyCode) -> bool
{
    let transformed: u32 = unsafe { transmute(*key) };
    let index = transformed as usize;
    let mut result = false;
    GLOBAL_KEYS.with(|s|
    {
        result = s.borrow().keys[index] == 1
    });
    return result;
}

pub fn is_released(key: &VirtualKeyCode) -> bool
{
    let transformed: u32 = unsafe { transmute(*key) };
    let index = transformed as usize;
    let mut result = false;
    GLOBAL_KEYS.with(|s|
    {
        result = s.borrow().keys[index] == 0
            && s.borrow().changes[index] > 0
    });
    return result;
}

pub fn is_pressed(key: &VirtualKeyCode) -> bool
{
    let transformed: u32 = unsafe { transmute(*key) };
    let index = transformed as usize;
    let mut result = false;
    GLOBAL_KEYS.with(|s|
    {
        result = s.borrow().keys[index] == 1
            && s.borrow().changes[index] > 0
    });
    return result;
}

pub fn update(event: &WindowEvent)
{
    match event
    {
        WindowEvent::KeyboardInput
        {
            input: KeyboardInput
            {
                    state,
                    virtual_keycode: Some(keycode),
                    ..
            },
            ..
        } =>
        {
            match *state
            {
                ElementState::Pressed => set_pressed(keycode),
                ElementState::Released => set_released(keycode),
            }
        }
        _ => {},
    }
}




fn set_pressed(key: &VirtualKeyCode)
{
    let transformed: u32 = unsafe { transmute(*key) };
    let index = transformed as usize;
    GLOBAL_KEYS.with(|s|
    {
        s.borrow_mut().changes[index] += 1;
        s.borrow_mut().keys[index] = 1;
    });
}
fn set_released(key: &VirtualKeyCode)
{
    let transformed: u32 = unsafe { transmute(*key) };
    let index = transformed as usize;
    GLOBAL_KEYS.with(|s|
    {
        s.borrow_mut().changes[index] += 1;
        s.borrow_mut().keys[index] = 0;
    });
}

