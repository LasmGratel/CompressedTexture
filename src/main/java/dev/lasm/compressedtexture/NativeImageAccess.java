package dev.lasm.compressedtexture;

import com.mojang.blaze3d.platform.NativeImage;

import java.lang.reflect.Constructor;

public final class NativeImageAccess {
    private static final Constructor<NativeImage> CTOR;

    static {
        try {
            CTOR = NativeImage.class.getDeclaredConstructor(
                    NativeImage.Format.class,
                    int.class,
                    int.class,
                    boolean.class,
                    long.class
            );
            CTOR.setAccessible(true);
        } catch (ReflectiveOperationException e) {
            throw new ExceptionInInitializerError(e);
        }
    }

    public static NativeImage create(NativeImage.Format format, int width, int height, boolean useStbFree, long pixels) {
        try {
            return CTOR.newInstance(format, width, height, useStbFree, pixels);
        } catch (ReflectiveOperationException e) {
            throw new RuntimeException("Failed to create NativeImage", e);
        }
    }

    private NativeImageAccess() {}
}