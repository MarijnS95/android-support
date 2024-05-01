use android_activity::AndroidApp;
use android_support::enable_immersive;

#[no_mangle]
fn android_main(app: AndroidApp) {
    enable_immersive(&app)
}
