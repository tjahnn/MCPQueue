use serde::{Deserialize, Serialize};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use uuid::Uuid;

unsafe extern "C" {
    fn queue_create() -> *mut std::ffi::c_void;
    fn queue_destroy(handle: *mut std::ffi::c_void);
    fn queue_enqueue(handle: *mut std::ffi::c_void, json: *const c_char);
    fn queue_dequeue(handle: *mut std::ffi::c_void) -> *mut c_char;
    fn queue_peek(handle: *mut std::ffi::c_void) -> *mut c_char;
    fn queue_get_all(handle: *mut std::ffi::c_void) -> *mut c_char;
    fn queue_size(handle: *mut std::ffi::c_void) -> usize;
    fn queue_clear(handle: *mut std::ffi::c_void);
    fn queue_free_string(ptr: *mut c_char);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueItem {
    pub id: String,
    pub data: serde_json::Value,
}

pub struct Queue(*mut std::ffi::c_void);

// Queue 내부 포인터는 Mutex 보호 하에서만 접근하므로 안전
unsafe impl Send for Queue {}
unsafe impl Sync for Queue {}

impl Queue {
    pub fn new() -> Self {
        Queue(unsafe { queue_create() })
    }

    pub fn enqueue(&mut self, data: serde_json::Value) -> QueueItem {
        let item = QueueItem { id: Uuid::new_v4().to_string(), data };
        let json = CString::new(serde_json::to_string(&item).unwrap()).unwrap();
        unsafe { queue_enqueue(self.0, json.as_ptr()) };
        item
    }

    pub fn dequeue(&mut self) -> Option<QueueItem> {
        self.take_ptr(unsafe { queue_dequeue(self.0) })
    }

    pub fn peek(&self) -> Option<QueueItem> {
        self.take_ptr(unsafe { queue_peek(self.0) })
    }

    pub fn list(&self) -> Vec<QueueItem> {
        let ptr = unsafe { queue_get_all(self.0) };
        let json = self.ptr_to_string(ptr);
        serde_json::from_str(&json).unwrap_or_default()
    }

    pub fn size(&self) -> usize {
        unsafe { queue_size(self.0) }
    }

    pub fn clear(&mut self) {
        unsafe { queue_clear(self.0) }
    }

    fn take_ptr(&self, ptr: *mut c_char) -> Option<QueueItem> {
        if ptr.is_null() {
            return None;
        }
        let json = self.ptr_to_string(ptr);
        serde_json::from_str(&json).ok()
    }

    fn ptr_to_string(&self, ptr: *mut c_char) -> String {
        let s = unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() };
        unsafe { queue_free_string(ptr) };
        s
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe { queue_destroy(self.0) };
    }
}
