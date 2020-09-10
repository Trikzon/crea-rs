package com.trikzon.omni_rs;

import android.opengl.GLES20;
import android.opengl.GLSurfaceView;

import javax.microedition.khronos.egl.EGLConfig;
import javax.microedition.khronos.opengles.GL10;

public class OpenGLRenderer implements GLSurfaceView.Renderer {

    private long appPtr = 0;

    @Override
    public void onSurfaceCreated(GL10 gl10, EGLConfig eglConfig) {
        long enginePtr = MainActivity.engineInit(this);
        appPtr = MainActivity.appInit(enginePtr);
    }

    @Override
    public void onSurfaceChanged(GL10 gl10, int i, int i1) {

    }

    @Override
    public void onDrawFrame(GL10 gl10) {
        GLES20.glClearColor(1.0f, 0.0f, 0.0f, 1.0f);
        GLES20.glClear(GLES20.GL_COLOR_BUFFER_BIT);

        MainActivity.appUpdate(appPtr);
    }

    private float maxNum = 0;

    private void testCallback(float foo) {
        if (foo >= maxNum) {
            maxNum = foo;
            System.out.println(foo);
        } else {
            System.err.println("New foo: " + foo + "; maxNum: " + maxNum);
            System.exit(-1);
        }
    }
}
