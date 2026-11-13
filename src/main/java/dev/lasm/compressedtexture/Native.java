package dev.lasm.compressedtexture;

import java.nio.LongBuffer;

public class Native {
    public static native long loadImage(long byteBuf, long size, long width, long height, long channels, long malloc);


    public static void loadNative() {
        System.loadLibrary("compressedtexture");
    }
}
