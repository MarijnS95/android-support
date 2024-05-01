use android_activity::AndroidApp;
use jni::{
    objects::{JObject, JValue},
    JavaVM,
};

pub fn enable_immersive(app: &AndroidApp) {
    let vm = unsafe { JavaVM::from_raw(app.vm_as_ptr().cast()) }.unwrap();

    let mut env = vm.attach_current_thread().unwrap();

    let activity = unsafe { JObject::from_raw(app.activity_as_ptr().cast()) };

    let r = env
        .call_static_method(
            "com/example/example/Activity",
            "enableImmersive",
            "(Landroid/app/Activity;)V",
            &[JValue::Object(&activity)],
        )
        .unwrap();
    dbg!(r.v());
}
