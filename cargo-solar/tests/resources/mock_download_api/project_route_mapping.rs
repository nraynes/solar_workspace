use std::cell::{Ref, RefMut};

use mocked_up::{
    database::Database,
    rest_service::{Request, Response, RouteBuilder, RouteBuilderMut},
};

pub fn project_get_mapping(
    b: RouteBuilder,
    project: &str,
    bin_name: &str,
    config_name: &str,
    arm_macos_call: fn(Request, Ref<Database>) -> Response,
    intel_macos_call: fn(Request, Ref<Database>) -> Response,
    linux_call: fn(Request, Ref<Database>) -> Response,
    windows_call: fn(Request, Ref<Database>) -> Response,
    config_call: fn(Request, Ref<Database>) -> Response,
) -> RouteBuilder {
    b.add(project, None, |b| {
        b.add("raw", None, |b| {
            b.add("refs", None, |b| {
                b.add("heads", None, |b| {
                    b.add("master", None, |b| {
                        b.add("bin", None, |b| {
                            b.add("arm-macos", None, |b| {
                                b.add(bin_name, Some(arm_macos_call), |b| b)
                            })
                            .add("intel-macos", None, |b| {
                                b.add(bin_name, Some(intel_macos_call), |b| b)
                            })
                            .add("linux", None, |b| b.add(bin_name, Some(linux_call), |b| b))
                            .add("windows", None, |b| {
                                b.add(bin_name, Some(windows_call), |b| b)
                            })
                        })
                        .add(config_name, Some(config_call), |b| b)
                    })
                })
            })
        })
    })
}

pub fn project_post_mapping(
    b: RouteBuilderMut,
    project: &str,
    bin_name: &str,
    config_name: &str,
    arm_macos_call: fn(Request, RefMut<Database>) -> Response,
    intel_macos_call: fn(Request, RefMut<Database>) -> Response,
    linux_call: fn(Request, RefMut<Database>) -> Response,
    windows_call: fn(Request, RefMut<Database>) -> Response,
    config_call: fn(Request, RefMut<Database>) -> Response,
) -> RouteBuilderMut {
    b.add(project, None, |b| {
        b.add("raw", None, |b| {
            b.add("refs", None, |b| {
                b.add("heads", None, |b| {
                    b.add("master", None, |b| {
                        b.add("bin", None, |b| {
                            b.add("arm-macos", None, |b| {
                                b.add(bin_name, Some(arm_macos_call), |b| b)
                            })
                            .add("intel-macos", None, |b| {
                                b.add(bin_name, Some(intel_macos_call), |b| b)
                            })
                            .add("linux", None, |b| b.add(bin_name, Some(linux_call), |b| b))
                            .add("windows", None, |b| {
                                b.add(bin_name, Some(windows_call), |b| b)
                            })
                        })
                        .add(config_name, Some(config_call), |b| b)
                    })
                })
            })
        })
    })
}
