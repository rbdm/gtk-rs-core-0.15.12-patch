use gio;
use gio::prelude::*;
use glib;
use glib::subclass::prelude::*;
use glib::translate::*;

pub type FileSize = <super::imp::FileSize as ObjectSubclass>::Instance;

/// # Safety
///
/// This is the ffi method to asynchronously get the file size. It accepts a callback of type
/// GAsyncReadyCallback, that will be invoked when the async operation completes. Typically,
/// this callback will invoke the get_file_size_finish method (implemented below) to get the
/// Task result and perform some operation with it.
#[no_mangle]
pub unsafe extern "C" fn my_file_size_get_file_size_async(
    this: *mut FileSize,
    cancellable: *mut gio::ffi::GCancellable,
    callback: gio::ffi::GAsyncReadyCallback,
    user_data: glib::ffi::gpointer,
) {
    let cancellable = gio::Cancellable::from_glib_borrow(cancellable);
    let closure = move |task: gio::LocalTask<i64>, source_object: Option<&glib::Object>| {
        let result: *mut gio::ffi::GAsyncResult =
            task.upcast_ref::<gio::AsyncResult>().to_glib_none().0;
        let source_object: *mut glib::object::GObject = source_object.to_glib_none().0;
        callback.unwrap()(source_object, result, user_data)
    };

    let source_object = &super::FileSize::from_glib_borrow(this);
    let task = gio::LocalTask::new(
        Some(source_object.upcast_ref::<glib::Object>()),
        Some(cancellable.as_ref()),
        closure,
    );

    glib::MainContext::default().spawn_local(async move {
        let size = gio::File::for_path("Cargo.toml")
            .query_info_future("*", gio::FileQueryInfoFlags::NONE, glib::PRIORITY_DEFAULT)
            .await
            .unwrap()
            .size();

        let source_object = task
            .upcast_ref::<gio::AsyncResult>()
            .source_object()
            .unwrap();

        let source_object = source_object
            .downcast_ref::<super::FileSize>()
            .unwrap()
            .imp();

        *source_object.size.lock().unwrap() = Some(size);
        task.return_result(Ok(size));
    });
}

/// # Safety
///
/// This method will typically be invoked in the callback passed to my_file_size_get_file_size_async in order
/// to propagate the Task result.
#[no_mangle]
pub unsafe extern "C" fn my_file_size_get_file_size_finish(
    _this: *mut FileSize,
    result: *mut gio::ffi::GAsyncResult,
    error: *mut *mut glib::ffi::GError,
) -> i64 {
    match gio::AsyncResult::from_glib_none(result)
        .downcast::<gio::Task<i64>>()
        .unwrap()
        .propagate()
    {
        Ok(v) => v,
        Err(e) => {
            if !error.is_null() {
                *error = e.into_raw();
            }
            0
        }
    }
}

/// # Safety
///
/// Simple getter
#[no_mangle]
pub unsafe extern "C" fn my_file_size_get_retrieved_size(this: *mut FileSize) -> i64 {
    let simple_object = super::FileSize::from_glib_borrow(this);
    let simple_object = simple_object
        .downcast_ref::<super::FileSize>()
        .unwrap()
        .imp();
    let x = *simple_object.size.lock().unwrap();
    x.unwrap_or(-1)
}
