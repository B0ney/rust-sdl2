//! Surface Rotozoomer

use libc::c_int;
use sdl2::surface::Surface;
use sdl2::SdlResult;
pub use std::f64::consts::PI;


mod ll {
    /* automatically generated by rust-bindgen */

    use libc::*;
    use sys::surface::SDL_Surface;
    extern "C" {
        pub fn rotozoomSurface(src: *mut SDL_Surface, angle: c_double,
                               zoom: c_double, smooth: c_int) -> *mut SDL_Surface;
        pub fn rotozoomSurfaceXY(src: *mut SDL_Surface, angle: c_double,
                                 zoomx: c_double, zoomy: c_double, smooth: c_int)
                                 -> *mut SDL_Surface;
        pub fn rotozoomSurfaceSize(width: c_int, height: c_int, angle: c_double,
                                   zoom: c_double, dstwidth: *mut c_int,
                                   dstheight: *mut c_int);
        pub fn rotozoomSurfaceSizeXY(width: c_int, height: c_int, angle: c_double,
                                     zoomx: c_double, zoomy: c_double,
                                     dstwidth: *mut c_int, dstheight: *mut c_int);
        pub fn zoomSurface(src: *mut SDL_Surface, zoomx: c_double,
                           zoomy: c_double, smooth: c_int) -> *mut SDL_Surface;
        pub fn zoomSurfaceSize(width: c_int, height: c_int, zoomx: c_double,
                               zoomy: c_double, dstwidth: *mut c_int,
                               dstheight: *mut c_int);
        pub fn shrinkSurface(src: *mut SDL_Surface, factorx: c_int,
                             factory: c_int) -> *mut SDL_Surface;
        pub fn rotateSurface90Degrees(src: *mut SDL_Surface,
                                      numClockwiseTurns: c_int) ->
            *mut SDL_Surface;
    }
}

/// RotozoomSurface for work with rust-sdl2 Surface type
pub trait RotozoomSurface {
    /// Rotates and zooms a surface and optional anti-aliasing.
    fn rotozoom(&self, angle: f64, zoom: f64, smooth: bool) -> SdlResult<Surface>;
    /// Rotates and zooms a surface with different horizontal and vertival scaling factors and optional anti-aliasing.
    fn rotozoom_xy(&self, angle: f64, zoomx: f64, zoomy: f64, smooth: bool) -> SdlResult<Surface>;
    /// Zoom a surface by independent horizontal and vertical factors with optional smoothing.
    fn zoom(&self, zoomx: f64, zoomy: f64, smooth: bool) -> SdlResult<Surface>;
    /// Shrink a surface by an integer ratio using averaging.
    fn shrink(&self, factorx: isize, factory: isize) -> SdlResult<Surface>;
    /// Rotates a 8/16/24/32 bit surface in increments of 90 degrees.
    fn rotate_90deg(&self, turns: isize) -> SdlResult<Surface>;
}

impl<'a> RotozoomSurface for Surface<'a> {
    fn rotozoom(&self, angle: f64, zoom: f64, smooth: bool) -> SdlResult<Surface> {
        let raw = unsafe {
            ll::rotozoomSurface(self.raw(), angle, zoom, smooth as c_int)
        };
        if raw.is_null() {
            Err("rotozoomSurface: error.".to_string())
        } else {
            unsafe { Ok(Surface::from_ll(raw, true)) }
        }
    }
    fn rotozoom_xy(&self, angle: f64, zoomx: f64, zoomy: f64, smooth: bool) -> SdlResult<Surface> {
        let raw = unsafe {
            ll::rotozoomSurfaceXY(self.raw(), angle, zoomx, zoomy, smooth as c_int)
        };
        if raw.is_null() {
            Err("rotozoomSurfaceXY: error.".to_string())
        } else {
            unsafe { Ok(Surface::from_ll(raw, true)) }
        }
    }
    fn zoom(&self, zoomx: f64, zoomy: f64, smooth: bool) -> SdlResult<Surface> {
        let raw = unsafe {
            ll::zoomSurface(self.raw(), zoomx, zoomy, smooth as c_int)
        };
        if raw.is_null() {
            Err("zoomSurface: error.".to_string())
        } else {
            unsafe { Ok(Surface::from_ll(raw, true)) }
        }
    }
    fn shrink(&self, factorx: isize, factory: isize) -> SdlResult<Surface> {
        let raw = unsafe {
            ll::shrinkSurface(self.raw(), factorx as c_int, factory as c_int)
        };
        if raw.is_null() {
            Err("shrinkSurface: error.".to_string())
        } else {
            unsafe { Ok(Surface::from_ll(raw, true)) }
        }
    }
    fn rotate_90deg(&self, turns: isize) -> SdlResult<Surface> {
        let raw = unsafe {
            ll::rotateSurface90Degrees(self.raw(), turns as c_int)
        };
        if raw.is_null() {
            Err("rotateSurface90Degrees: error.".to_string())
        } else {
            unsafe { Ok(Surface::from_ll(raw, true)) }
        }
    }
}

pub fn get_zoom_size(width: isize, height: isize, zoomx: f64, zoomy: f64) -> (isize, isize) {
    let mut w: c_int = 0;
    let mut h: c_int = 0;
    unsafe { ll::zoomSurfaceSize(width as c_int, height as c_int, zoomx, zoomy, &mut w, &mut h) }
    (w as isize, h as isize)
}

pub fn get_rotozoom_size(width: isize, height: isize, angle: f64, zoom: f64) -> (isize, isize) {
    let mut w: c_int = 0;
    let mut h: c_int = 0;
    unsafe { ll::rotozoomSurfaceSize(width as c_int, height as c_int, angle, zoom, &mut w, &mut h) }
    (w as isize, h as isize)
}

pub fn get_rotozoom_xy_size(width: isize, height: isize, angle: f64, zoomx: f64, zoomy: f64) -> (isize, isize) {
    let mut w: c_int = 0;
    let mut h: c_int = 0;
    unsafe { ll::rotozoomSurfaceSizeXY(width as c_int, height as c_int, angle, zoomx, zoomy, &mut w, &mut h) }
    (w as isize, h as isize)
}
