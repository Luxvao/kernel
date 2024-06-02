use core::ffi::c_void;

use limine::request::EfiSystemTableRequest;
use uefi::table::{Runtime, SystemTable};

use crate::{lazy::Lazy, sync::Mutex, utils::hcf};

pub static EFI_TABLE_REQUEST: EfiSystemTableRequest = EfiSystemTableRequest::new();

pub static EFI_TABLE: Lazy<Mutex<SystemTableWrapper>> = Lazy::new(get_efi_table);

pub struct SystemTableWrapper(pub SystemTable<Runtime>);

unsafe impl Send for SystemTableWrapper {}

unsafe impl Sync for SystemTableWrapper {}

fn get_efi_table() -> Mutex<SystemTableWrapper> {
    if let Some(table) = EFI_TABLE_REQUEST.get_response() {
        unsafe {
            if let Some(table) = SystemTable::from_ptr(table.address() as *mut c_void) {
                let wrapper = SystemTableWrapper(table);

                return Mutex::new(wrapper);
            }
        }
    }

    hcf();
}
