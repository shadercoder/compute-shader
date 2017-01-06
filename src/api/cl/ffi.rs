/* automatically generated by rust-bindgen and then manually modified */

#![allow(non_camel_case_types)]

#[cfg(target_os = "macos")]
use io_surface::IOSurfaceRef;

pub type int32_t = i32;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type intptr_t = isize;
pub type cl_int = int32_t;
pub type cl_uint = uint32_t;
pub type cl_ulong = uint64_t;
pub type size_t = usize;
pub enum _cl_platform_id { }
pub type cl_platform_id = *mut _cl_platform_id;
pub enum _cl_device_id { }
pub type cl_device_id = *mut _cl_device_id;
pub enum _cl_context { }
pub type cl_context = *mut _cl_context;
pub enum _cl_command_queue { }
pub type cl_command_queue = *mut _cl_command_queue;
pub enum _cl_mem { }
pub type cl_mem = *mut _cl_mem;
pub enum _cl_program { }
pub type cl_program = *mut _cl_program;
pub enum _cl_kernel { }
pub type cl_kernel = *mut _cl_kernel;
pub enum _cl_event { }
pub type cl_event = *mut _cl_event;
pub type cl_bool = cl_uint;
pub type cl_bitfield = cl_ulong;
pub type cl_device_type = cl_bitfield;
pub type cl_command_queue_properties = cl_bitfield;
pub type cl_context_properties = intptr_t;
pub type cl_context_info = cl_uint;
pub type cl_program_build_info = cl_uint;
pub type cl_channel_order = cl_uint;
pub type cl_channel_type = cl_uint;
pub type cl_mem_flags = cl_bitfield;
pub type cl_image_info = cl_uint;
pub type cl_profiling_info = cl_uint;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct _cl_image_format {
    pub image_channel_order: cl_channel_order,
    pub image_channel_data_type: cl_channel_type,
}
impl ::std::default::Default for _cl_image_format {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type cl_image_format = _cl_image_format;

#[link(name = "OpenCL", kind = "framework")]
extern "C" {
    pub fn clGetDeviceIDs(arg1: cl_platform_id, arg2: cl_device_type,
                          arg3: cl_uint, arg4: *mut cl_device_id,
                          arg5: *mut cl_uint) -> cl_int;
    pub fn clCreateContext(arg1: *const cl_context_properties, arg2: cl_uint,
                           arg3: *const cl_device_id,
                           arg4:
                               ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                              *const ::std::os::raw::c_char,
                                                                          arg2:
                                                                              *const ::std::os::raw::c_void,
                                                                          arg3:
                                                                              size_t,
                                                                          arg4:
                                                                              *mut ::std::os::raw::c_void)>,
                           arg5: *mut ::std::os::raw::c_void,
                           arg6: *mut cl_int) -> cl_context;
    pub fn clReleaseContext(arg1: cl_context) -> cl_int;
    pub fn clGetContextInfo(arg1: cl_context, arg2: cl_context_info,
                            arg3: size_t, arg4: *mut ::std::os::raw::c_void,
                            arg5: *mut size_t) -> cl_int;
    pub fn clCreateCommandQueue(arg1: cl_context, arg2: cl_device_id,
                                arg3: cl_command_queue_properties,
                                arg4: *mut cl_int) -> cl_command_queue;
    pub fn clReleaseCommandQueue(arg1: cl_command_queue) -> cl_int;
    pub fn clCreateBuffer(arg1: cl_context, arg2: cl_mem_flags, arg3: size_t,
                          arg4: *mut ::std::os::raw::c_void,
                          arg5: *mut cl_int) -> cl_mem;
    pub fn clReleaseMemObject(arg1: cl_mem) -> cl_int;
    pub fn clGetImageInfo(arg1: cl_mem, arg2: cl_image_info, arg3: size_t,
                          arg4: *mut ::std::os::raw::c_void,
                          arg5: *mut size_t) -> cl_int;
    pub fn clCreateProgramWithSource(arg1: cl_context, arg2: cl_uint,
                                     arg3: *mut *const ::std::os::raw::c_char,
                                     arg4: *const size_t, arg5: *mut cl_int)
     -> cl_program;
    pub fn clReleaseProgram(arg1: cl_program) -> cl_int;
    pub fn clBuildProgram(arg1: cl_program, arg2: cl_uint,
                          arg3: *const cl_device_id,
                          arg4: *const ::std::os::raw::c_char,
                          arg5:
                              ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                             cl_program,
                                                                         arg2:
                                                                             *mut ::std::os::raw::c_void)>,
                          arg6: *mut ::std::os::raw::c_void) -> cl_int;
    pub fn clGetProgramBuildInfo(program: cl_program,
                                 device: cl_device_id,
                                 param_name: cl_program_build_info,
                                 param_value_size: size_t,
                                 param_value: *mut ::std::os::raw::c_void,
                                 param_value_size_ret: *mut size_t)
                                 -> cl_int;
    pub fn clCreateKernelsInProgram(arg1: cl_program, arg2: cl_uint,
                                    arg3: *mut cl_kernel, arg4: *mut cl_uint)
     -> cl_int;
    pub fn clReleaseKernel(arg1: cl_kernel) -> cl_int;
    pub fn clSetKernelArg(arg1: cl_kernel, arg2: cl_uint, arg3: size_t,
                          arg4: *const ::std::os::raw::c_void) -> cl_int;
    pub fn clWaitForEvents(arg1: cl_uint, arg2: *const cl_event) -> cl_int;
    pub fn clReleaseEvent(arg1: cl_event) -> cl_int;
    pub fn clGetEventProfilingInfo(arg1: cl_event, arg2: cl_profiling_info,
                                   arg3: size_t,
                                   arg4: *mut ::std::os::raw::c_void,
                                   arg5: *mut size_t) -> cl_int;
    pub fn clFinish(arg1: cl_command_queue) -> cl_int;
    pub fn clEnqueueReadBuffer(arg1: cl_command_queue, arg2: cl_mem,
                               arg3: cl_bool, arg4: size_t, arg5: size_t,
                               arg6: *mut ::std::os::raw::c_void,
                               arg7: cl_uint, arg8: *const cl_event,
                               arg9: *mut cl_event) -> cl_int;
    pub fn clEnqueueFillImage(arg1: cl_command_queue, arg2: cl_mem,
                              arg3: *const ::std::os::raw::c_void,
                              arg4: *const size_t, arg5: *const size_t,
                              arg6: cl_uint, arg7: *const cl_event,
                              arg8: *mut cl_event) -> cl_int;
    pub fn clEnqueueNDRangeKernel(arg1: cl_command_queue, arg2: cl_kernel,
                                  arg3: cl_uint, arg4: *const size_t,
                                  arg5: *const size_t, arg6: *const size_t,
                                  arg7: cl_uint, arg8: *const cl_event,
                                  arg9: *mut cl_event) -> cl_int;

    #[cfg(target_os = "macos")]
    pub fn clCreateImageFromIOSurface2DAPPLE(context: cl_context,
                                             flags: cl_mem_flags,
                                             image_format: *const cl_image_format,
                                             image_width: size_t,
                                             image_height: size_t,
                                             iosurface: IOSurfaceRef,
                                             errcode_ret: *mut cl_int)
                                             -> cl_mem;
}

pub const CL_SUCCESS: cl_int = 0;

pub const CL_TRUE: cl_bool = 1;

pub const CL_DEVICE_TYPE_GPU: cl_device_type = 1 << 2;

pub const CL_QUEUE_PROFILING_ENABLE: cl_command_queue_properties = 1 << 2;

pub const CL_CONTEXT_DEVICES: cl_context_info = 0x1081;

pub const CL_R: cl_channel_order = 0x10b0;

pub const CL_UNSIGNED_INT8: cl_channel_type = 0x10da;

pub const CL_MEM_READ_WRITE: cl_mem_flags = 1 << 0;
pub const CL_MEM_WRITE_ONLY: cl_mem_flags = 1 << 1;
pub const CL_MEM_READ_ONLY: cl_mem_flags = 1 << 2;
pub const CL_MEM_USE_HOST_PTR: cl_mem_flags = 1 << 3;
pub const CL_MEM_COPY_HOST_PTR: cl_mem_flags = 1 << 5;

pub const CL_IMAGE_WIDTH: cl_image_info = 0x1114;
pub const CL_IMAGE_HEIGHT: cl_image_info = 0x1115;
pub const CL_IMAGE_DEPTH: cl_image_info = 0x1116;

pub const CL_PROGRAM_BUILD_LOG: cl_program_build_info = 0x1183;
