package org.nativescript.wcg.core

import dalvik.annotation.optimization.CriticalNative
import dalvik.annotation.optimization.FastNative

class URL @JvmOverloads constructor(value: String, base: String? = null) {
  private var native: Long

  init {
    native = nativeCreate(value, base)
    if (native == 0L) {
      throw Error("Invalid URL")
    }
  }


  var hash: String
    get() {
      return nativeGetHash(native)
    }
    set(value) {
      nativeSetHash(native, value)
    }


  var host: String
    get() {
      return nativeGetHost(native)
    }
    set(value) {
      nativeSetHost(native, value)
    }

  var hostname: String
    get() {
      return nativeGetHostName(native)
    }
    set(value) {
      nativeSetHostName(native, value)
    }


  var href: String
    get() {
      return nativeGetHref(native)
    }
    set(value) {
      nativeSetHref(native, value)
    }

  val origin: String
    get() {
      return nativeGetOrigin(native)
    }

  var password: String
    get() {
      return nativeGetPassword(native)
    }
    set(value) {
      nativeSetPassword(native, value)
    }


  var pathname: String
    get() {
      return nativeGetPathName(native)
    }
    set(value) {
      nativeSetPathName(native, value)
    }

  var port: String
    get() {
      return nativeGetPort(native)
    }
    set(value) {
      nativeSetPort(native, value)
    }


  var protocol: String
    get() {
      return nativeGetProtocol(native)
    }
    set(value) {
      nativeSetProtocol(native, value)
    }

  var search: String
    get() {
      return nativeGetSearch(native)
    }
    set(value) {
      nativeSetSearch(native, value)
    }

  var username: String
    get() {
      return nativeGetUsername(native)
    }
    set(value) {
      nativeSetUserName(native, value)
    }


  override fun toString(): String {
    return nativeToString(native)
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

    @JvmStatic
    fun canParse(value: String, base: String): Boolean {
      return nativeCanParse(value, base)
    }

    @FastNative
    @JvmStatic
    external fun nativeCanParse(value: String, base: String): Boolean

    @FastNative
    @JvmStatic
    external fun nativeCreate(value: String, base: String?): Long

    @CriticalNative
    @JvmStatic
    external fun nativeDestroy(url: Long)

    @FastNative
    @JvmStatic
    external fun nativeToString(url: Long): String

    @FastNative
    @JvmStatic
    external fun nativeSetHash(url: Long, value: String)

    @FastNative
    @JvmStatic
    external fun nativeGetHash(url: Long): String


    @FastNative
    @JvmStatic
    external fun nativeSetHost(url: Long, value: String)

    @FastNative
    @JvmStatic
    external fun nativeGetHost(url: Long): String

    @FastNative
    @JvmStatic
    external fun nativeSetHostName(url: Long, value: String)

    @FastNative
    @JvmStatic
    external fun nativeGetHostName(url: Long): String

    @FastNative
    @JvmStatic
    external fun nativeSetHref(url: Long, value: String)

    @FastNative
    @JvmStatic
    external fun nativeGetHref(url: Long): String

    @FastNative
    @JvmStatic
    external fun nativeGetOrigin(url: Long): String

    @FastNative
    @JvmStatic
    external fun nativeSetPassword(url: Long, value: String)

    @FastNative
    @JvmStatic
    external fun nativeGetPassword(url: Long): String

    @FastNative
    @JvmStatic
    external fun nativeSetPathName(url: Long, value: String)


    @FastNative
    @JvmStatic
    external fun nativeGetPathName(url: Long): String

    @FastNative
    @JvmStatic
    external fun nativeSetPort(url: Long, value: String)

    @FastNative
    @JvmStatic
    external fun nativeGetPort(url: Long): String

    @FastNative
    @JvmStatic
    external fun nativeSetProtocol(url: Long, value: String)

    @FastNative
    @JvmStatic
    external fun nativeGetProtocol(url: Long): String


    @FastNative
    @JvmStatic
    external fun nativeSetSearch(url: Long, value: String)

    @FastNative
    @JvmStatic
    external fun nativeGetSearch(url: Long): String


    @FastNative
    @JvmStatic
    external fun nativeSetUserName(url: Long, value: String)

    @FastNative
    @JvmStatic
    external fun nativeGetUsername(url: Long): String

  }
}
