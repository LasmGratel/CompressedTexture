package dev.lasm.compressedtexture.mixin;

import com.mojang.blaze3d.platform.NativeImage;
import dev.lasm.compressedtexture.Native;
import dev.lasm.compressedtexture.NativeImageAccess;
import org.lwjgl.system.MemoryStack;
import org.lwjgl.system.MemoryUtil;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

import javax.annotation.Nullable;
import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.IntBuffer;

@Mixin(NativeImage.class)
public class MixinNativeImage {
    @Inject(method = "read(Lcom/mojang/blaze3d/platform/NativeImage$Format;Ljava/nio/ByteBuffer;)Lcom/mojang/blaze3d/platform/NativeImage;",
    at = @At(value = "INVOKE", target = "Lnet/minecraft/util/PngInfo;validateHeader(Ljava/nio/ByteBuffer;)V"), cancellable = true)
    private static void onRead(NativeImage.Format format, ByteBuffer textureData, CallbackInfoReturnable<NativeImage> cir) throws IOException {
        try (MemoryStack memorystack = MemoryStack.stackPush()) {
            var w = memorystack.mallocLong(1);
            var h = memorystack.mallocLong(1);
            var c = memorystack.mallocLong(1);
            var address = Native.loadImage(MemoryUtil.memAddress(textureData), textureData.capacity(), MemoryUtil.memAddress(w), MemoryUtil.memAddress(h),
                    MemoryUtil.memAddress(c), MemoryUtil.getAllocator().getMalloc());

            var nativeimage = NativeImageAccess.create(
                    getStbFormat((int)c.get(0)),
                    (int) w.get(0),
                    (int) h.get(0),
                    true,
                    address
            );
            cir.setReturnValue(nativeimage);
        }

    }

    private static NativeImage.Format getStbFormat(int channels) {
        switch (channels) {
            case 1:
                return NativeImage.Format.LUMINANCE;
            case 2:
                return NativeImage.Format.LUMINANCE_ALPHA;
            case 3:
                return NativeImage.Format.RGB;
            case 4:
            default:
                return NativeImage.Format.RGBA;
        }
    }
}
