package rust.android_support;

import android.view.Window;
import android.view.View;

public class Activity extends android.app.Activity {
    // https://developer.android.com/training/system-ui/navigation#java
    public static void enableImmersive(android.app.Activity activity) {
        Window window = activity.getWindow();
        View decorView = window.getDecorView();

        // Hide both the navigation bar and the status bar.
        // SYSTEM_UI_FLAG_FULLSCREEN is only available on Android 4.1 and higher, but as
        // a general rule, you should design your app to hide the status bar whenever you
        // hide the navigation bar.
        int uiOptions = View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
                      | View.SYSTEM_UI_FLAG_FULLSCREEN;
        decorView.setSystemUiVisibility(uiOptions);
    }
}
