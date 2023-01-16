use std::mem::transmute;
use winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent},
};

// Could add own keys, so we can remove the dependency to winit

pub struct Input
{
    keys: [u8; 512],
    changes: [u8; 512]
}

impl Input
{
    pub fn new() -> Self
    {
        let s: Self = unsafe { std::mem::zeroed() };
        return s;
    }

    pub fn reset(&mut self)
    {
        unsafe { self.changes = std::mem::zeroed() };
    }

    pub fn is_down(&self, key: &VirtualKeyCode) -> bool
    {
        let transformed: u32 = unsafe { transmute(*key) };
        let index = transformed as usize;
        let result = self.keys[index] == 1;
        return result;
    }

    pub fn is_released(&self, key: &VirtualKeyCode) -> bool
    {
        let transformed: u32 = unsafe { transmute(*key) };
        let index = transformed as usize;
        let result = self.keys[index] == 0
            && self.changes[index] > 0;
        return result;
    }

    pub fn is_pressed(&self, key: &VirtualKeyCode) -> bool
    {
        let transformed: u32 = unsafe { transmute(*key) };
        let index = transformed as usize;
        let result = self.keys[index] == 1
            && self.changes[index] > 0;
        return result;
    }

    pub fn update(&mut self, event: &WindowEvent)
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
                    ElementState::Pressed => self.set_pressed(keycode),
                    ElementState::Released => self.set_released(keycode),
                }
            }
            _ => {},
        }
    }


    fn set_pressed(&mut self, key: &VirtualKeyCode)
    {
        let transformed: u32 = unsafe { transmute(*key) };
        let index = transformed as usize;
        self.changes[index] += 1;
        self.keys[index] = 1;
    }
    fn set_released(&mut self, key: &VirtualKeyCode)
    {
        let transformed: u32 = unsafe { transmute(*key) };
        let index = transformed as usize;
        self.changes[index] += 1;
        self.keys[index] = 0;
    }
}

