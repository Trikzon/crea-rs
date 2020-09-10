package com.trikzon.omni_rs;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.view.View;

public class MainActivity extends AppCompatActivity {

    // Used to load the libraries on application startup
    static {
        // TODO: read library name from a file so it's easy for a user to change it
        System.loadLibrary("app");
        System.loadLibrary("engine");
    }

    public static native long engineInit(OpenGLRenderer callbackOwner);
    public static native long appInit(long enginePtr);
    public static native void appUpdate(long appPtr);

    private OpenGLView openGLView;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        openGLView = findViewById(R.id.openGLView);
    }

    @Override
    protected void onResume() {
        super.onResume();
        openGLView.onResume();

        // Removes title, hides status bar, hides navigation bar
        openGLView.setSystemUiVisibility(View.SYSTEM_UI_FLAG_LAYOUT_STABLE
                | View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY
                | View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN
                | View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION
                | View.SYSTEM_UI_FLAG_FULLSCREEN
                | View.SYSTEM_UI_FLAG_HIDE_NAVIGATION);
    }

    @Override
    protected void onPause() {
        super.onPause();
        openGLView.onPause();
    }
}