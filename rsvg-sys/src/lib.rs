// Generated by gir (https://github.com/gtk-rs/gir @ 1bef39f)
// from gir-files (https://github.com/selaux/gir-files.git @ ec4c204)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(clippy::approx_constant, clippy::type_complexity, clippy::unreadable_literal, clippy::upper_case_acronyms)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

use glib_sys as glib;
use gobject_sys as gobject;
use gio_sys as gio;
use gdk_pixbuf_sys as gdk_pixbuf;
use cairo_sys as cairo;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType};

// Enums
pub type RsvgError = c_int;
pub const RSVG_ERROR_FAILED: RsvgError = 0;

// Constants
pub const LIBRSVG_MAJOR_VERSION: c_int = 2;
pub const LIBRSVG_MICRO_VERSION: c_int = 16;
pub const LIBRSVG_MINOR_VERSION: c_int = 40;
pub const LIBRSVG_VERSION: *const c_char = b"2.40.16\0" as *const u8 as *const c_char;

// Flags
pub type RsvgHandleFlags = c_uint;
pub const RSVG_HANDLE_FLAGS_NONE: RsvgHandleFlags = 0;
pub const RSVG_HANDLE_FLAG_UNLIMITED: RsvgHandleFlags = 1;
pub const RSVG_HANDLE_FLAG_KEEP_IMAGE_DATA: RsvgHandleFlags = 2;

// Records
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RsvgDimensionData {
    pub width: c_int,
    pub height: c_int,
    pub em: c_double,
    pub ex: c_double,
}

impl ::std::fmt::Debug for RsvgDimensionData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("RsvgDimensionData @ {:p}", self))
         .field("width", &self.width)
         .field("height", &self.height)
         .field("em", &self.em)
         .field("ex", &self.ex)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RsvgHandleClass {
    pub parent: gobject::GObjectClass,
    pub _abi_padding: [gpointer; 15],
}

impl ::std::fmt::Debug for RsvgHandleClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("RsvgHandleClass @ {:p}", self))
         .field("parent", &self.parent)
         .finish()
    }
}

#[repr(C)]
pub struct _RsvgHandlePrivate(c_void);

pub type RsvgHandlePrivate = *mut _RsvgHandlePrivate;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RsvgPositionData {
    pub x: c_int,
    pub y: c_int,
}

impl ::std::fmt::Debug for RsvgPositionData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("RsvgPositionData @ {:p}", self))
         .field("x", &self.x)
         .field("y", &self.y)
         .finish()
    }
}

// Classes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RsvgHandle {
    pub parent: gobject::GObject,
    pub priv_: *mut RsvgHandlePrivate,
    pub _abi_padding: [gpointer; 15],
}

impl ::std::fmt::Debug for RsvgHandle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("RsvgHandle @ {:p}", self))
         .field("parent", &self.parent)
         .finish()
    }
}

#[link(name = "rsvg-2")]
extern "C" {

    //=========================================================================
    // RsvgError
    //=========================================================================
    pub fn rsvg_error_get_type() -> GType;
    pub fn rsvg_error_quark() -> glib::GQuark;

    //=========================================================================
    // RsvgHandleFlags
    //=========================================================================
    pub fn rsvg_handle_flags_get_type() -> GType;

    //=========================================================================
    // RsvgHandle
    //=========================================================================
    pub fn rsvg_handle_get_type() -> GType;
    pub fn rsvg_handle_new() -> *mut RsvgHandle;
    pub fn rsvg_handle_new_from_data(data: *mut u8, data_len: size_t, error: *mut *mut glib::GError) -> *mut RsvgHandle;
    pub fn rsvg_handle_new_from_file(file_name: *const c_char, error: *mut *mut glib::GError) -> *mut RsvgHandle;
    pub fn rsvg_handle_new_from_gfile_sync(file: *mut gio::GFile, flags: RsvgHandleFlags, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> *mut RsvgHandle;
    pub fn rsvg_handle_new_from_stream_sync(input_stream: *mut gio::GInputStream, base_file: *mut gio::GFile, flags: RsvgHandleFlags, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> *mut RsvgHandle;
    pub fn rsvg_handle_new_with_flags(flags: RsvgHandleFlags) -> *mut RsvgHandle;
    pub fn rsvg_handle_close(handle: *mut RsvgHandle, error: *mut *mut glib::GError) -> gboolean;
    pub fn rsvg_handle_get_base_uri(handle: *mut RsvgHandle) -> *const c_char;
    pub fn rsvg_handle_get_dimensions(handle: *mut RsvgHandle, dimension_data: *mut RsvgDimensionData);
    pub fn rsvg_handle_get_dimensions_sub(handle: *mut RsvgHandle, dimension_data: *mut RsvgDimensionData, id: *const c_char) -> gboolean;
    pub fn rsvg_handle_get_pixbuf(handle: *mut RsvgHandle) -> *mut gdk_pixbuf::GdkPixbuf;
    pub fn rsvg_handle_get_pixbuf_sub(handle: *mut RsvgHandle, id: *const c_char) -> *mut gdk_pixbuf::GdkPixbuf;
    pub fn rsvg_handle_get_position_sub(handle: *mut RsvgHandle, position_data: *mut RsvgPositionData, id: *const c_char) -> gboolean;
    pub fn rsvg_handle_has_sub(handle: *mut RsvgHandle, id: *const c_char) -> gboolean;
    pub fn rsvg_handle_read_stream_sync(handle: *mut RsvgHandle, stream: *mut gio::GInputStream, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> gboolean;
    pub fn rsvg_handle_render_cairo(handle: *mut RsvgHandle, cr: *mut cairo::cairo_t) -> gboolean;
    pub fn rsvg_handle_render_cairo_sub(handle: *mut RsvgHandle, cr: *mut cairo::cairo_t, id: *const c_char) -> gboolean;
    pub fn rsvg_handle_set_base_gfile(handle: *mut RsvgHandle, base_file: *mut gio::GFile);
    pub fn rsvg_handle_set_base_uri(handle: *mut RsvgHandle, base_uri: *const c_char);
    pub fn rsvg_handle_set_dpi(handle: *mut RsvgHandle, dpi: c_double);
    pub fn rsvg_handle_set_dpi_x_y(handle: *mut RsvgHandle, dpi_x: c_double, dpi_y: c_double);
    pub fn rsvg_handle_write(handle: *mut RsvgHandle, buf: *mut u8, count: size_t, error: *mut *mut glib::GError) -> gboolean;

    //=========================================================================
    // Other functions
    //=========================================================================
    pub fn rsvg_cleanup();
    pub fn rsvg_set_default_dpi(dpi: c_double);
    pub fn rsvg_set_default_dpi_x_y(dpi_x: c_double, dpi_y: c_double);

}
