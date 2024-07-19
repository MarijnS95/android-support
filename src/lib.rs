use android_activity::AndroidApp;
use jni::{
    objects::{JClass, JObject, JString, JValue},
    JavaVM,
};

pub fn enable_immersive(app: &AndroidApp) {
    // std::env::set_var("RUST_BACKTRACE", "1");

    let vm = unsafe { JavaVM::from_raw(app.vm_as_ptr().cast()) }.unwrap();

    let mut env = vm.attach_current_thread().unwrap();

    let activity = unsafe { JObject::from_raw(app.activity_as_ptr().cast()) };

    // First, get the ClassLoader that has our APK in DexPathList
    let cl = env
        .call_method(
            &activity,
            "getClassLoader",
            "()Ljava/lang/ClassLoader;",
            &[],
        )
        .unwrap()
        .l()
        .unwrap();
    dbg!(&cl);

    let current_thread = env
        .call_static_method(
            "java/lang/Thread",
            "currentThread",
            "()Ljava/lang/Thread;",
            &[],
        )
        .unwrap()
        .l()
        .unwrap();
    env.call_method(
        &current_thread,
        "setContextClassLoader",
        "(Ljava/lang/ClassLoader;)V",
        &[(&cl).into()],
    )
    .unwrap()
    .v()
    .unwrap();

    let cl = env
        .call_method(
            &current_thread,
            "getContextClassLoader",
            "()Ljava/lang/ClassLoader;",
            &[],
        )
        .unwrap()
        .l()
        .unwrap();

    let loader_desc: JString = env
        .call_method(&cl, "toString", "()Ljava/lang/String;", &[])
        .unwrap()
        .l()
        .unwrap()
        // Force this to be a JClass or an ICE happens:
        // error: internal compiler error: compiler/rustc_infer/src/infer/at.rs:400:21: relating different kinds: jni::objects::JObject<'?197> '?174
        .into();
    let loader_desc = env.get_string(&loader_desc).unwrap();
    dbg!(loader_desc.to_str().unwrap());

    // Find our support class - it happens to be an Activity for now, but we just need one static
    // method from it
    // let class_name = env.new_string("rust/android_support/Activity").unwrap();
    // let support_class: JClass = env
    //     .call_method(
    //         &cl,
    //         "loadClass",
    //         "(Ljava/lang/String;)Ljava/lang/Class;",
    //         &[JValue::Object(&class_name)],
    //     )
    //     .unwrap()
    //     .l()
    //     .unwrap()
    //     // Force this to be a JClass or an ICE happens:
    //     // error: internal compiler error: compiler/rustc_infer/src/infer/at.rs:400:21: relating different kinds: jni::objects::JObject<'?197> '?174
    //     .into();
    // dbg!(&support_class);

    env.call_static_method(
        // Use the class loaded from our Activity' ClassLoader, otherwise JNI won't find the class from the APK
        // &support_class,
        "rust/android_support/Activity",
        "enableImmersive",
        "(Landroid/app/Activity;)V",
        &[(&activity).into()],
    )
    .unwrap()
    .v()
    .unwrap()
}
