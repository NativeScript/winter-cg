package org.nativescript.wcg.core

import dalvik.annotation.optimization.CriticalNative
import dalvik.annotation.optimization.FastNative
import java.nio.ByteBuffer

class TextDecoder @JvmOverloads constructor(encoding: String = "utf-8") {
  private var native: Long

  init {
    native = nativeCreate(encoding)
    if (native == 0L) {
      throw Error("Invalid Encoding")
    }
  }

  fun decode(value: ByteArray): String {
    return nativeDecode(native, value)
  }

  fun decode(value: ByteBuffer): String {
    if (value.isDirect) {
      return nativeDecodeBuffer(native, value)
    }

    return nativeDecode(native, value.array())
  }

  @Synchronized
  @Throws(Throwable::class)
  protected fun finalize() {
    if (native != 0L) {
      nativeDestroy(native)
      native = 0L
    }
  }


  companion object {

    init {
      GC.loadLib()
    }
    @FastNative
    @JvmStatic
    private external fun nativeCreate(encoding: String): Long

    @FastNative
    @JvmStatic
    private external fun nativeGetEncoding(decoder: Long): String

    @FastNative
    @JvmStatic
    private external fun nativeDecode(decoder: Long, value: ByteArray): String

    @FastNative
    @JvmStatic
    private external fun nativeDecodeBuffer(decoder: Long, value: ByteBuffer): String

    @CriticalNative
    @JvmStatic
    private external fun nativeDestroy(decoder: Long)
  }
}
